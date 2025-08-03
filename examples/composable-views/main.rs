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
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::{style, Stylize};
use std::time::{Duration, Instant};

/// Message for timer ticks
#[derive(Debug)]
struct TimerTickMsg;

/// Message for spinner animation ticks
#[derive(Debug)]
struct SpinnerTickMsg;

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
    last_tick: Option<Instant>,
}

impl TimerModel {
    fn new(duration: Duration) -> Self {
        Self {
            duration,
            remaining: duration,
            running: true, // Start automatically like Go example
            last_tick: None,
        }
    }

    fn tick(&mut self) {
        if !self.running || self.remaining == Duration::ZERO {
            return;
        }

        let now = Instant::now();
        let elapsed = match self.last_tick {
            Some(prev) => now.saturating_duration_since(prev),
            None => Duration::from_secs(1), // First tick
        };
        self.last_tick = Some(now);

        // Subtract elapsed time
        if elapsed >= self.remaining {
            self.remaining = Duration::ZERO;
        } else {
            self.remaining -= elapsed;
        }
    }

    fn is_done(&self) -> bool {
        self.remaining == Duration::ZERO
    }

    fn reset(&mut self) {
        self.remaining = self.duration;
        self.running = true;
        self.last_tick = Some(Instant::now());
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
        // TODO: Apply lipgloss spinner style with color #69 (currently using crossterm blue)
        style(frames[self.frame]).blue().to_string()
    }
}

/// Main model that composes timer and spinner
#[derive(Debug)]
struct MainModel {
    state: SessionState,
    timer: TimerModel,
    spinner: SpinnerModel,
    timer_started: bool,
}

impl MainModel {
    fn new() -> Self {
        Self {
            state: SessionState::TimerView,
            timer: TimerModel::new(Duration::from_secs(60)),
            spinner: SpinnerModel::new(),
            timer_started: false,
        }
    }

    fn current_focused_model(&self) -> &str {
        match self.state {
            SessionState::TimerView => "timer",
            SessionState::SpinnerView => "spinner",
        }
    }

    /// Render a view with border
    fn render_view(&self, content: &str, width: usize, height: usize, focused: bool) -> String {
        // TODO: Use lipgloss for proper styling
        // Currently implementing manual borders
        let border_color = if focused {
            // TODO: lipgloss Color("69") - using blue for now
            crossterm::style::Color::Rgb {
                r: 105,
                g: 105,
                b: 255,
            }
        } else {
            crossterm::style::Color::Reset
        };

        let mut result = String::new();

        // Calculate padding for centering content
        let content_lines: Vec<&str> = content.lines().collect();
        let content_height = content_lines.len();
        let vertical_padding = (height.saturating_sub(content_height + 2)) / 2;

        if focused {
            // Draw top border
            result.push_str(
                &style(format!("┌{}┐", "─".repeat(width - 2)))
                    .with(border_color)
                    .to_string(),
            );
            result.push('\n');

            // Add top padding
            for _ in 0..vertical_padding {
                result.push_str(
                    &style(format!("│{}│", " ".repeat(width - 2)))
                        .with(border_color)
                        .to_string(),
                );
                result.push('\n');
            }

            // Draw content with borders
            for line in content_lines {
                let content_width = line.chars().count();
                let left_padding = (width.saturating_sub(content_width + 2)) / 2;
                let right_padding = width.saturating_sub(content_width + left_padding + 2);

                result.push_str(&style("│").with(border_color).to_string());
                result.push_str(&" ".repeat(left_padding));
                result.push_str(line);
                result.push_str(&" ".repeat(right_padding));
                result.push_str(&style("│").with(border_color).to_string());
                result.push('\n');
            }

            // Add bottom padding
            for _ in 0..(height.saturating_sub(content_height + vertical_padding + 2)) {
                result.push_str(
                    &style(format!("│{}│", " ".repeat(width - 2)))
                        .with(border_color)
                        .to_string(),
                );
                result.push('\n');
            }

            // Draw bottom border
            result.push_str(
                &style(format!("└{}┘", "─".repeat(width - 2)))
                    .with(border_color)
                    .to_string(),
            );
        } else {
            // No border for unfocused view
            // Add top padding
            for _ in 0..vertical_padding {
                result.push_str(&" ".repeat(width));
                result.push('\n');
            }

            // Draw content centered
            for line in content_lines {
                let content_width = line.chars().count();
                let left_padding = (width.saturating_sub(content_width)) / 2;
                let right_padding = width.saturating_sub(content_width + left_padding);

                result.push_str(&" ".repeat(left_padding));
                result.push_str(line);
                result.push_str(&" ".repeat(right_padding));
                result.push('\n');
            }

            // Add bottom padding
            for _ in 0..(height.saturating_sub(content_height + vertical_padding)) {
                result.push_str(&" ".repeat(width));
                result.push('\n');
            }
        }

        result
    }

    /// Join two views horizontally
    fn join_horizontal(&self, left: &str, right: &str) -> String {
        let left_lines: Vec<&str> = left.lines().collect();
        let right_lines: Vec<&str> = right.lines().collect();
        let max_lines = left_lines.len().max(right_lines.len());

        let mut result = String::new();
        for i in 0..max_lines {
            if i < left_lines.len() {
                result.push_str(left_lines[i]);
            } else {
                result.push_str(&" ".repeat(15)); // Width of the box
            }

            if i < right_lines.len() {
                result.push_str(right_lines[i]);
            }

            if i < max_lines - 1 {
                result.push('\n');
            }
        }

        result
    }
}

impl Model for MainModel {
    fn init() -> (Self, Option<Cmd>) {
        let mut model = MainModel::new();
        // Don't start timer yet; start spinner independently to avoid batch gating
        model.timer.last_tick = None;

        let spinner_cmd = tick(model.spinner.spinner_type.interval(), |_| {
            Box::new(SpinnerTickMsg) as Msg
        });

        (model, Some(spinner_cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        let mut cmds = Vec::new();

        // Handle keyboard input
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit());
                }
                KeyCode::Char('q') => {
                    return Some(quit());
                }
                KeyCode::Tab => {
                    // Toggle focus between views
                    self.state = match self.state {
                        SessionState::TimerView => SessionState::SpinnerView,
                        SessionState::SpinnerView => SessionState::TimerView,
                    };
                }
                KeyCode::Char('n') => {
                    // Context-aware 'n' key handling
                    match self.state {
                        SessionState::TimerView => {
                            // Reset timer
                            self.timer.reset();
                            // Ensure timer ticking resumes after reset
                            cmds.push(tick(Duration::from_secs(1), |_| {
                                Box::new(TimerTickMsg) as Msg
                            }));
                            self.timer_started = true;
                        }
                        SessionState::SpinnerView => {
                            // Next spinner style
                            self.spinner.next_spinner();
                            // Continue spinner ticking with the new interval
                            cmds.push(tick(self.spinner.spinner_type.interval(), |_| {
                                Box::new(SpinnerTickMsg) as Msg
                            }));
                        }
                    }
                }
                _ => {}
            }
        }

        // Handle timer ticks
        if msg.downcast_ref::<TimerTickMsg>().is_some() {
            if !self.timer.is_done() {
                self.timer.tick();
                // Schedule next timer tick (one-shot)
                cmds.push(tick(Duration::from_secs(1), |_| {
                    Box::new(TimerTickMsg) as Msg
                }));
                self.timer_started = true;
            }
        }

        // Handle spinner ticks
        if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
            self.spinner.tick();
            // Kick off timer ticking the first time we see spinner progress
            if !self.timer_started {
                cmds.push(tick(Duration::from_secs(1), |_| {
                    Box::new(TimerTickMsg) as Msg
                }));
                self.timer_started = true;
            }
            // Schedule next spinner tick (one-shot) with current interval
            cmds.push(tick(self.spinner.spinner_type.interval(), |_| {
                Box::new(SpinnerTickMsg) as Msg
            }));
        }

        if cmds.is_empty() {
            None
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

        let mut result = String::new();
        result.push_str(&self.join_horizontal(&timer_view, &spinner_view));
        result.push('\n');

        // Help text
        // TODO: Apply lipgloss helpStyle with Color("241")
        let help = format!(
            "tab: focus next • n: new {} • q: exit",
            self.current_focused_model()
        );
        result.push_str(&style(help).dark_grey().to_string());
        result.push('\n');

        result
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the program
    let program = Program::<MainModel>::builder()
        .alt_screen(true)
        .signal_handler(true)
        .build()?;

    program.run().await?;

    Ok(())
}
