//! Composable Views Example
//!
//! This example demonstrates:
//! - Composing multiple sub-models (timer and spinner) into a single application
//! - Focus management between different views using Tab key
//! - Context-aware keyboard shortcuts ('n' behaves differently based on focus)
//! - Visual styling with borders to indicate focus state
//! - Coordinating commands between sub-models
//!
//! The example shows a timer counting down from 60 seconds alongside a spinner
//! with multiple styles. Users can switch focus between views and interact with
//! each component independently.

use bubbletea_rs::{batch, quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};

use lipgloss_extras::lipgloss::{Color, Style};
use std::time::Duration;

/// Message for timer ticks
#[derive(Debug)]
struct TimerTickMsg;

/// Message for spinner animation ticks
#[derive(Debug)]
struct SpinnerTickMsg;

/// Key bindings for the composable views example
#[derive(Debug)]
pub struct KeyBindings {
    pub quit: Binding,
    pub quit_alt: Binding,
    pub tab: Binding,
    pub new: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![
                with_keys_str(&["q"]),
                with_help("q", "quit"),
            ]),
            quit_alt: new_binding(vec![
                with_keys_str(&["ctrl+c"]),
                with_help("ctrl+c", "quit"),
            ]),
            tab: new_binding(vec![
                with_keys_str(&["tab"]),
                with_help("tab", "focus next"),
            ]),
            new: new_binding(vec![
                with_keys_str(&["n"]),
                with_help("n", "new"),
            ]),
        }
    }
}

/// Tracks which model has focus
#[derive(Debug, Clone, Copy, PartialEq)]
enum SessionState {
    TimerView,
    SpinnerView,
}

/// Different spinner styles available
#[derive(Debug, Clone, Copy, PartialEq)]
enum SpinnerType {
    Line,
    Dot,
    MiniDot,
    Jump,
    Pulse,
    Points,
    Globe,
    Moon,
    Monkey,
}

impl SpinnerType {
    fn frames(&self) -> &'static [&'static str] {
        match self {
            SpinnerType::Line => &["|", "/", "-", "\\"],
            SpinnerType::Dot => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerType::MiniDot => &["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"],
            SpinnerType::Jump => &["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"],
            SpinnerType::Pulse => &["█", "▓", "▒", "░", "▒", "▓"],
            SpinnerType::Points => &["∙∙∙", "●∙∙", "∙●∙", "∙∙●"],
            SpinnerType::Globe => &["🌍", "🌎", "🌏"],
            SpinnerType::Moon => &["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
            SpinnerType::Monkey => &["🙈", "🙉", "🙊"],
        }
    }

    fn interval(&self) -> Duration {
        match self {
            SpinnerType::Line => Duration::from_millis(130),
            SpinnerType::Dot => Duration::from_millis(100),
            SpinnerType::MiniDot => Duration::from_millis(100),
            SpinnerType::Jump => Duration::from_millis(100),
            SpinnerType::Pulse => Duration::from_millis(100),
            SpinnerType::Points => Duration::from_millis(120),
            SpinnerType::Globe => Duration::from_millis(130),
            SpinnerType::Moon => Duration::from_millis(80),
            SpinnerType::Monkey => Duration::from_millis(300),
        }
    }

    fn next(&self) -> Self {
        match self {
            SpinnerType::Line => SpinnerType::Dot,
            SpinnerType::Dot => SpinnerType::MiniDot,
            SpinnerType::MiniDot => SpinnerType::Jump,
            SpinnerType::Jump => SpinnerType::Pulse,
            SpinnerType::Pulse => SpinnerType::Points,
            SpinnerType::Points => SpinnerType::Globe,
            SpinnerType::Globe => SpinnerType::Moon,
            SpinnerType::Moon => SpinnerType::Monkey,
            SpinnerType::Monkey => SpinnerType::Line,
        }
    }
}

/// Timer sub-model that counts down from a given duration
#[derive(Debug)]
struct TimerModel {
    duration: Duration,
    remaining: Duration,
    running: bool,
}

impl TimerModel {
    fn new(duration: Duration) -> Self {
        Self {
            duration,
            remaining: duration,
            running: true,
        }
    }

    fn tick(&mut self) {
        if !self.running {
            return;
        }
        
        if self.remaining > Duration::ZERO {
            self.remaining = self.remaining.saturating_sub(Duration::from_secs(1));
            if self.remaining == Duration::ZERO {
                self.running = false;
            }
        }
    }

    fn is_done(&self) -> bool {
        self.remaining == Duration::ZERO
    }

    fn reset(&mut self) {
        self.remaining = self.duration;
        self.running = true;
    }

    fn view(&self) -> String {
        if self.is_done() {
            "✓".to_string()
        } else {
            let mins = self.remaining.as_secs() / 60;
            let secs = self.remaining.as_secs() % 60;
            format!("{:02}:{:02}", mins, secs)
        }
    }
}

/// Spinner sub-model
#[derive(Debug)]
struct SpinnerModel {
    spinner_type: SpinnerType,
    frame: usize,
}

impl SpinnerModel {
    fn new() -> Self {
        Self {
            spinner_type: SpinnerType::Line,
            frame: 0,
        }
    }

    fn tick(&mut self) {
        let frames = self.spinner_type.frames();
        self.frame = (self.frame + 1) % frames.len();
    }

    fn next_spinner(&mut self) {
        self.spinner_type = self.spinner_type.next();
        self.frame = 0;
    }

    fn view(&self) -> String {
        let frames = self.spinner_type.frames();
        // Use lipgloss-extras spinner style with color #69
        Style::new()
            .foreground(Color::from("69"))
            .render(frames[self.frame])
    }
}

/// Main model that composes timer and spinner
#[derive(Debug)]
struct MainModel {
    state: SessionState,
    timer: TimerModel,
    spinner: SpinnerModel,
    keys: KeyBindings,
}

impl MainModel {
    fn new() -> Self {
        Self {
            state: SessionState::TimerView,
            timer: TimerModel::new(Duration::from_secs(60)),
            spinner: SpinnerModel::new(),
            keys: KeyBindings::default(),
        }
    }

    fn current_focused_model(&self) -> &str {
        match self.state {
            SessionState::TimerView => "timer",
            SessionState::SpinnerView => "spinner",
        }
    }

    /// Render a view with border and centered content (simplified manual approach)
    fn render_view(&self, content: &str, width: usize, height: usize, focused: bool) -> String {
        if focused {
            // Create a bordered box manually for focused view
            let border_color = Color::from("69");
            let top_border = Style::new().foreground(border_color.clone()).render(&format!("┌{}┐", "─".repeat(width - 2)));
            let bottom_border = Style::new().foreground(border_color.clone()).render(&format!("└{}┘", "─".repeat(width - 2)));
            
            let mut lines = vec![top_border];
            
            // Add vertical padding and center the content
            let padding_top = (height - 3) / 2; // -3 for top, content, bottom borders
            for _ in 0..padding_top {
                let line = Style::new().foreground(border_color.clone()).render(&format!("│{}│", " ".repeat(width - 2)));
                lines.push(line);
            }
            
            // Center the content line
            let content_width = content.len();
            let left_pad = if content_width >= width.saturating_sub(2) { 0 } else { (width - 2 - content_width) / 2 };
            let right_pad = width.saturating_sub(2).saturating_sub(content_width).saturating_sub(left_pad);
            let content_line = format!(
                "{}{}{}{}",
                Style::new().foreground(border_color.clone()).render("│"),
                " ".repeat(left_pad),
                content,
                " ".repeat(right_pad)
            ) + &Style::new().foreground(border_color.clone()).render("│");
            lines.push(content_line);
            
            // Fill remaining vertical space
            let remaining_height = height.saturating_sub(lines.len()).saturating_sub(1); // -1 for bottom border
            for _ in 0..remaining_height {
                let line = Style::new().foreground(border_color.clone()).render(&format!("│{}│", " ".repeat(width - 2)));
                lines.push(line);
            }
            
            lines.push(bottom_border);
            lines.join("\n")
        } else {
            // No border, just center the content
            let mut lines = Vec::new();
            let padding_top = (height - 1) / 2;
            
            // Add top padding
            for _ in 0..padding_top {
                lines.push(" ".repeat(width));
            }
            
            // Center content
            let content_width = content.len();
            let left_pad = if content_width >= width { 0 } else { (width - content_width) / 2 };
            let right_pad = width.saturating_sub(content_width).saturating_sub(left_pad);
            lines.push(format!("{}{}{}", " ".repeat(left_pad), content, " ".repeat(right_pad)));
            
            // Fill remaining space
            let remaining = height.saturating_sub(lines.len());
            for _ in 0..remaining {
                lines.push(" ".repeat(width));
            }
            
            lines.join("\n")
        }
    }


}

impl Model for MainModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = MainModel::new();
        
        // Use tick() re-arming pattern like the Go example
        let timer_cmd = tick(Duration::from_secs(1), |_| {
            Box::new(TimerTickMsg) as Msg
        });
        let spinner_cmd = tick(model.spinner.spinner_type.interval(), |_| {
            Box::new(SpinnerTickMsg) as Msg
        });
        
        (model, Some(batch(vec![timer_cmd, spinner_cmd])))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        let mut cmds: Vec<Cmd> = Vec::new();
        
        // Handle keyboard input
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if self.keys.quit.matches(key_msg) {
                return Some(quit());
            } else if self.keys.quit_alt.matches(key_msg) {
                return Some(quit());
            } else if self.keys.tab.matches(key_msg) {
                // Toggle focus between views
                self.state = match self.state {
                    SessionState::TimerView => SessionState::SpinnerView,
                    SessionState::SpinnerView => SessionState::TimerView,
                };
            } else if self.keys.new.matches(key_msg) {
                // Context-aware 'n' key handling
                match self.state {
                    SessionState::TimerView => {
                        // Reset timer and immediately start new countdown
                        self.timer.reset();
                        cmds.push(tick(Duration::from_secs(1), |_| {
                            Box::new(TimerTickMsg) as Msg
                        }));
                    }
                    SessionState::SpinnerView => {
                        // Change spinner style and immediately start with new interval
                        self.spinner.next_spinner();
                        cmds.push(tick(self.spinner.spinner_type.interval(), |_| {
                            Box::new(SpinnerTickMsg) as Msg
                        }));
                    }
                }
            }
        }

        // Handle timer ticks - re-arm for next tick
        if msg.downcast_ref::<TimerTickMsg>().is_some() {
            self.timer.tick();
            // Only re-arm if timer is still running
            if !self.timer.is_done() {
                cmds.push(tick(Duration::from_secs(1), |_| {
                    Box::new(TimerTickMsg) as Msg
                }));
            }
        }

        // Handle spinner ticks - always re-arm for continuous animation
        if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
            self.spinner.tick();
            // Re-arm with current interval (allows dynamic speed changes)
            cmds.push(tick(self.spinner.spinner_type.interval(), |_| {
                Box::new(SpinnerTickMsg) as Msg
            }));
        }

        // Return commands
        if cmds.is_empty() {
            None
        } else if cmds.len() == 1 {
            Some(cmds.into_iter().next().unwrap())
        } else {
            Some(batch(cmds))
        }
    }

    fn view(&self) -> String {
        let timer_view = self.render_view(
            &self.timer.view(),
            15,
            5,
            self.state == SessionState::TimerView,
        );

        let spinner_view = self.render_view(
            &self.spinner.view(),
            15,
            5,
            self.state == SessionState::SpinnerView,
        );

        // Manual horizontal join since lipgloss-extras join_horizontal API is unclear
        let timer_lines: Vec<&str> = timer_view.lines().collect();
        let spinner_lines: Vec<&str> = spinner_view.lines().collect();
        let max_lines = timer_lines.len().max(spinner_lines.len());
        
        let mut result_lines = Vec::new();
        for i in 0..max_lines {
            let timer_line = timer_lines.get(i).unwrap_or(&"");
            let spinner_line = spinner_lines.get(i).unwrap_or(&"");
            result_lines.push(format!("{}{}", timer_line, spinner_line));
        }
        let views = result_lines.join("\n");

        // Help text with styling matching Go version helpStyle with Color("241")
        let help_style = Style::new().foreground(Color::from("241"));
        let help = help_style.render(&format!(
            "tab: focus next • n: new {} • q: exit",
            self.current_focused_model()
        ));

        format!("{}\n{}\n", views, help)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the program
    let program = Program::<MainModel>::builder()
        .alt_screen(false) // Match Go version - no alternate screen
        .signal_handler(true) // Enable Ctrl+C handling
        .build()?;

    program.run().await?;

    Ok(())
}
