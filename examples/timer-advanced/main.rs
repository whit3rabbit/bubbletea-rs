//! Advanced Timer Example
//!
//! This example demonstrates comprehensive usage of bubbletea-widgets components:
//! - `timer::Model` for precise countdown timers
//! - `key::Binding` for organized key binding management
//! - `KeyMap` trait for help system integration
//! - Multiple timer types (Quick, Pomodoro, Custom)
//! - Visual styling with lipgloss-extras
//!
//! Features:
//! - Multiple timer presets with easy switching
//! - Start/stop/reset functionality for each timer
//! - Expandable help system (short and full views)
//! - Visual progress indicators and status styling
//! - Responsive layout that adapts to terminal width

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
use bubbletea_widgets::key::{new_binding, with_keys_str, with_help, Binding, KeyMap};
use bubbletea_widgets::timer::{new as new_timer, Model as TimerModel, TimeoutMsg, TickMsg, StartStopMsg};
use lipgloss_extras::lipgloss::{Color, Style};
use std::time::Duration;

// Synthetic message used to trigger the initial render immediately after startup.
struct InitRenderMsg;

/// Timer preset types for different use cases
#[derive(Debug, Clone, Copy, PartialEq)]
enum TimerType {
    Quick,      // 5 seconds - for demos and quick tasks
    Pomodoro,   // 25 minutes - work session
    Break,      // 5 minutes - short break
}

impl TimerType {
    fn duration(self) -> Duration {
        match self {
            TimerType::Quick => Duration::from_secs(5),
            TimerType::Pomodoro => Duration::from_secs(25 * 60), // 25 minutes
            TimerType::Break => Duration::from_secs(5 * 60),     // 5 minutes
        }
    }

    fn name(self) -> &'static str {
        match self {
            TimerType::Quick => "Quick Timer",
            TimerType::Pomodoro => "Pomodoro Work",
            TimerType::Break => "Short Break",
        }
    }

    fn description(self) -> &'static str {
        match self {
            TimerType::Quick => "5 second demonstration timer",
            TimerType::Pomodoro => "25 minute focused work session",
            TimerType::Break => "5 minute relaxation break",
        }
    }
}

/// Main application model containing multiple timers and UI state
pub struct TimerApp {
    // Timer state
    timers: Vec<(TimerType, TimerModel)>,
    active_timer: usize,
    
    // UI state  
    help_expanded: bool,
    terminal_width: u16,
    quitting: bool,
    
    // Key bindings
    key_bindings: TimerKeyBindings,
}

/// Organized key bindings for the timer application
pub struct TimerKeyBindings {
    pub timer_start_stop: Binding,
    pub timer_reset: Binding,
    pub timer_next: Binding,
    pub timer_prev: Binding,
    pub help_toggle: Binding,
    pub quit: Binding,
}

impl TimerKeyBindings {
    pub fn new() -> Self {
        Self {
            timer_start_stop: new_binding(vec![
                with_keys_str(&["space", "s"]),
                with_help("space/s", "start/stop timer"),
            ]),
            timer_reset: new_binding(vec![
                with_keys_str(&["r"]),
                with_help("r", "reset timer"),
            ]),
            timer_next: new_binding(vec![
                with_keys_str(&["right", "l", "n"]),
                with_help("→/l/n", "next timer"),
            ]),
            timer_prev: new_binding(vec![
                with_keys_str(&["left", "h", "p"]),
                with_help("←/h/p", "previous timer"),
            ]),
            help_toggle: new_binding(vec![
                with_keys_str(&["?"]),
                with_help("?", "toggle help"),
            ]),
            quit: new_binding(vec![
                with_keys_str(&["q", "esc", "ctrl+c"]),
                with_help("q", "quit"),
            ]),
        }
    }
}

impl KeyMap for TimerKeyBindings {
    fn short_help(&self) -> Vec<&Binding> {
        vec![&self.timer_start_stop, &self.help_toggle, &self.quit]
    }

    fn full_help(&self) -> Vec<Vec<&Binding>> {
        vec![
            // Timer controls column
            vec![
                &self.timer_start_stop,
                &self.timer_reset,
                &self.timer_next,
                &self.timer_prev,
            ],
            // Application controls column
            vec![
                &self.help_toggle,
                &self.quit,
            ],
        ]
    }
}

impl TimerApp {
    pub fn new() -> Self {
        // Create timer instances for each preset
        let timers = vec![
            (TimerType::Quick, new_timer(TimerType::Quick.duration())),
            (TimerType::Pomodoro, new_timer(TimerType::Pomodoro.duration())),
            (TimerType::Break, new_timer(TimerType::Break.duration())),
        ];

        Self {
            timers,
            active_timer: 0,
            help_expanded: false,
            terminal_width: 80,
            quitting: false,
            key_bindings: TimerKeyBindings::new(),
        }
    }

    fn current_timer(&self) -> &TimerModel {
        &self.timers[self.active_timer].1
    }

    fn current_timer_mut(&mut self) -> &mut TimerModel {
        &mut self.timers[self.active_timer].1
    }

    fn current_timer_type(&self) -> TimerType {
        self.timers[self.active_timer].0
    }

    fn switch_timer(&mut self, direction: i32) {
        let new_index = if direction > 0 {
            (self.active_timer + 1) % self.timers.len()
        } else {
            (self.active_timer + self.timers.len() - 1) % self.timers.len()
        };
        self.active_timer = new_index;
    }

    fn reset_current_timer(&mut self) {
        let timer_type = self.current_timer_type();
        let new_timer = new_timer(timer_type.duration());
        self.timers[self.active_timer].1 = new_timer;
    }

    fn render_timer_status(&self) -> String {
        let timer = self.current_timer();
        let timer_type = self.current_timer_type();
        
        // Timer name and description
        let name_style = Style::new()
            .foreground(Color::from("#FF75B7"))
            .bold(true);
        let desc_style = Style::new()
            .foreground(Color::from("#666666"));
            
        let timer_info = format!(
            "{}\n{}",
            name_style.render(timer_type.name()),
            desc_style.render(timer_type.description())
        );

        // Timer display with status
        let time_display = timer.view();
        let status = if timer.timedout() {
            Style::new()
                .foreground(Color::from("#FF0000"))
                .bold(true)
                .render("⏰ TIME'S UP! Press 'r' to reset")
        } else if timer.running() {
            Style::new()
                .foreground(Color::from("#00FF00"))
                .render("▶ Running")
        } else {
            Style::new()
                .foreground(Color::from("#FFAA00"))
                .render("⏸ Paused")
        };

        // Progress indicator
        let progress = self.render_progress_bar();

        // Timer selector
        let selector = self.render_timer_selector();

        format!(
            "{}\n\n{}\n{}\n\n{}\n\n{}",
            timer_info, time_display, status, progress, selector
        )
    }

    fn render_progress_bar(&self) -> String {
        let timer = self.current_timer();
        let timer_type = self.current_timer_type();
        let total = timer_type.duration();
        let remaining = timer.timeout;
        
        if total.is_zero() {
            return String::new();
        }
        
        let progress = 1.0 - (remaining.as_secs_f64() / total.as_secs_f64());
        let width = (self.terminal_width as usize).min(50).max(20);
        let filled = ((width as f64) * progress).round() as usize;
        let empty = width - filled;
        
        let filled_char = "█";
        let empty_char = "░";
        
        let filled_style = Style::new().foreground(Color::from("#00AAFF"));
        let empty_style = Style::new().foreground(Color::from("#333333"));
        
        format!(
            "{}{}",
            filled_style.render(&filled_char.repeat(filled)),
            empty_style.render(&empty_char.repeat(empty))
        )
    }

    fn render_timer_selector(&self) -> String {
        let mut parts = Vec::new();
        
        for (i, (timer_type, _)) in self.timers.iter().enumerate() {
            let name = timer_type.name();
            let styled_name = if i == self.active_timer {
                Style::new()
                    .foreground(Color::from("#FFFFFF"))
                    .background(Color::from("#0087FF"))
                    .render(&format!(" {} ", name))
            } else {
                Style::new()
                    .foreground(Color::from("#666666"))
                    .render(&format!(" {} ", name))
            };
            parts.push(styled_name);
        }
        
        format!("Timers: {}", parts.join(" "))
    }

    fn render_help(&self) -> String {
        if self.help_expanded {
            let full_help = self.key_bindings.full_help();
            self.format_help_columns(&full_help[0], &full_help[1])
        } else {
            let short_help = self.key_bindings.short_help();
            self.format_help_line(&short_help)
        }
    }

    fn format_help_line(&self, bindings: &[&Binding]) -> String {
        bindings
            .iter()
            .map(|binding| {
                let help = binding.help();
                format!("{} {}", help.key, help.desc)
            })
            .collect::<Vec<_>>()
            .join(" • ")
    }

    fn format_help_columns(&self, col1: &[&Binding], col2: &[&Binding]) -> String {
        let left_items: Vec<String> = col1
            .iter()
            .map(|b| {
                let help = b.help();
                format!("{} {}", help.key, help.desc)
            })
            .collect();
            
        let right_items: Vec<String> = col2
            .iter()
            .map(|b| {
                let help = b.help();
                format!("{} {}", help.key, help.desc)
            })
            .collect();

        let max_len = col1.len().max(col2.len());
        let mut lines = Vec::new();
        
        for i in 0..max_len {
            let left = if i < left_items.len() { &left_items[i] } else { "" };
            let right = if i < right_items.len() { &right_items[i] } else { "" };
            
            if right.is_empty() {
                lines.push(left.to_string());
            } else {
                lines.push(format!("{:<30} {}", left, right));
            }
        }
        
        lines.join("\n")
    }
}

impl Model for TimerApp {
    fn init() -> (Self, Option<Cmd>) {
        let mut app = Self::new();
        
        // Set initial terminal width
        if let Ok((w, _h)) = crossterm::terminal::size() {
            app.terminal_width = w;
        }

        // Initialize the first timer like the basic timer example
        let init_cmd = app.current_timer().init();
        
        (app, Some(init_cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle timer-specific messages first
        if let Some(timeout) = msg.downcast_ref::<TimeoutMsg>() {
            // Check if any of our timers timed out
            for (_, timer) in &self.timers {
                if timeout.id == timer.id() {
                    // Could add notification or auto-switch logic here
                    break;
                }
            }
        }

        // Handle timer tick messages  
        if let Some(_tick_msg) = msg.downcast_ref::<TickMsg>() {
            return self.current_timer_mut().update(msg);
        }
        
        // Handle timer start/stop messages
        if let Some(_start_stop_msg) = msg.downcast_ref::<StartStopMsg>() {
            return self.current_timer_mut().update(msg);
        }
        
        // Forward timeout messages to active timer
        if msg.downcast_ref::<TimeoutMsg>().is_some() {
            return self.current_timer_mut().update(msg);
        }

        // Handle keyboard input
        if let Some(key) = msg.downcast_ref::<KeyMsg>() {
            if self.key_bindings.timer_start_stop.matches(key) {
                return Some(self.current_timer().toggle());
            } else if self.key_bindings.timer_reset.matches(key) {
                self.reset_current_timer();
                return Some(self.current_timer().init());
            } else if self.key_bindings.timer_next.matches(key) {
                self.switch_timer(1);
            } else if self.key_bindings.timer_prev.matches(key) {
                self.switch_timer(-1);
            } else if self.key_bindings.help_toggle.matches(key) {
                self.help_expanded = !self.help_expanded;
            } else if self.key_bindings.quit.matches(key) {
                self.quitting = true;
                return Some(quit());
            }
        }

        // Handle window size changes
        if let Some(size) = msg.downcast_ref::<WindowSizeMsg>() {
            self.terminal_width = size.width;
        }

        // Handle init render message
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            // No-op: just triggers render
        }

        None
    }

    fn view(&self) -> String {
        if self.quitting {
            return "Thanks for using the Advanced Timer! 👋\n".to_string();
        }

        let timer_status = self.render_timer_status();
        let help_view = self.render_help();
        
        // Create a visually separated layout
        let separator = Style::new()
            .foreground(Color::from("#333333"))
            .render(&"─".repeat((self.terminal_width as usize).min(60)));

        format!("{}\n\n{}\n\n{}", timer_status, separator, help_view)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<TimerApp>::builder()
        .signal_handler(true)
        .alt_screen(true)
        .build()?;

    program.run().await?;
    Ok(())
}