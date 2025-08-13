//! Composable Views Example
//!
//! This example demonstrates:
//! - Composing multiple sub-models (timer and spinner) using bubbletea-widgets
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
use bubbletea_widgets::timer;

use lipgloss_extras::lipgloss::position::CENTER;
use lipgloss_extras::lipgloss::{border, Color, Style};
use std::time::Duration;

/// Message for spinner animation ticks
#[derive(Debug)]
pub struct SpinnerTickMsg;

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
            quit: new_binding(vec![with_keys_str(&["q"]), with_help("q", "quit")]),
            quit_alt: new_binding(vec![
                with_keys_str(&["ctrl+c"]),
                with_help("ctrl+c", "quit"),
            ]),
            tab: new_binding(vec![
                with_keys_str(&["tab"]),
                with_help("tab", "focus next"),
            ]),
            new: new_binding(vec![with_keys_str(&["n"]), with_help("n", "new")]),
        }
    }
}

/// Tracks which model has focus
#[derive(Debug, Clone, Copy, PartialEq)]
enum SessionState {
    TimerView,
    SpinnerView,
}

/// Available spinner styles (matching Go example)
#[derive(Debug, Clone, Copy)]
enum SpinnerStyle {
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

impl SpinnerStyle {
    fn all() -> &'static [SpinnerStyle] {
        &[
            SpinnerStyle::Line,
            SpinnerStyle::Dot,
            SpinnerStyle::MiniDot,
            SpinnerStyle::Jump,
            SpinnerStyle::Pulse,
            SpinnerStyle::Points,
            SpinnerStyle::Globe,
            SpinnerStyle::Moon,
            SpinnerStyle::Monkey,
        ]
    }
}

/// Main model that composes timer and spinner using bubbletea-widgets
#[derive(Debug)]
struct MainModel {
    state: SessionState,
    timer_model: timer::Model,
    spinner_frame: usize,
    spinner_styles: Vec<SpinnerStyle>,
    current_spinner_index: usize,
    keys: KeyBindings,
}

impl MainModel {
    fn new() -> Self {
        let spinner_styles = SpinnerStyle::all().to_vec();
        let current_spinner_index = 0;

        // Create timer widget (60 second countdown)
        let timer_model = timer::new(Duration::from_secs(60));

        Self {
            state: SessionState::TimerView,
            timer_model,
            spinner_frame: 0,
            spinner_styles,
            current_spinner_index,
            keys: KeyBindings::default(),
        }
    }

    fn current_focused_model(&self) -> &str {
        match self.state {
            SessionState::TimerView => "timer",
            SessionState::SpinnerView => "spinner",
        }
    }

    fn next_spinner(&mut self) {
        self.current_spinner_index = (self.current_spinner_index + 1) % self.spinner_styles.len();
        self.spinner_frame = 0; // Reset frame when changing spinner style
    }

    /// Get the current spinner frame display based on current style
    fn spinner_view(&self) -> String {
        let frames = self.get_spinner_frames();
        let frame = frames[self.spinner_frame % frames.len()];

        // Apply color styling matching Go example
        Style::new().foreground(Color::from("69")).render(frame)
    }

    /// Get frames for the current spinner style
    fn get_spinner_frames(&self) -> &'static [&'static str] {
        match self.spinner_styles[self.current_spinner_index] {
            SpinnerStyle::Line => &["|", "/", "-", "\\"],
            SpinnerStyle::Dot => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerStyle::MiniDot => &["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"],
            SpinnerStyle::Jump => &["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"],
            SpinnerStyle::Pulse => &[
                "█", "▉", "▊", "▋", "▌", "▍", "▎", "▏", "▎", "▍", "▌", "▋", "▊", "▉",
            ],
            SpinnerStyle::Points => &["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"],
            SpinnerStyle::Globe => &["🌍", "🌎", "🌏"],
            SpinnerStyle::Moon => &["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
            SpinnerStyle::Monkey => &["🙈", "🙉", "🙊"],
        }
    }

    /// Advance to the next spinner frame
    fn advance_spinner(&mut self) {
        let frames = self.get_spinner_frames();
        self.spinner_frame = (self.spinner_frame + 1) % frames.len();
    }

    /// Style for the focused model box
    fn focused_style() -> Style {
        Style::new()
            .width(15)
            .height(5)
            .align_horizontal(CENTER)
            .align_vertical(CENTER)
            .border(border::normal_border())
            .border_foreground(Color::from("69"))
    }

    /// Style for the unfocused model box  
    fn model_style() -> Style {
        Style::new()
            .width(15)
            .height(5)
            .align_horizontal(CENTER)
            .align_vertical(CENTER)
            .border(border::hidden_border())
    }

    /// Join two views horizontally with proper spacing
    fn join_horizontal(left: &str, right: &str) -> String {
        let left_lines: Vec<&str> = left.lines().collect();
        let right_lines: Vec<&str> = right.lines().collect();
        let max_lines = left_lines.len().max(right_lines.len());

        let mut result = Vec::new();
        for i in 0..max_lines {
            let left_line = left_lines.get(i).unwrap_or(&"");
            let right_line = right_lines.get(i).unwrap_or(&"");
            result.push(format!("{}{}", left_line, right_line));
        }
        result.join("\n")
    }
}

impl Model for MainModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = MainModel::new();

        // Start both timer and spinner animations
        let timer_cmd = model.timer_model.start();
        let spinner_cmd = tick(Duration::from_millis(80), |_| {
            Box::new(SpinnerTickMsg) as Msg
        });

        (model, Some(batch(vec![timer_cmd, spinner_cmd])))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        let mut cmds: Vec<Cmd> = Vec::new();

        // Handle spinner tick messages
        if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
            self.advance_spinner();
            // Schedule next spinner tick
            cmds.push(tick(Duration::from_millis(80), |_| {
                Box::new(SpinnerTickMsg) as Msg
            }));
        }

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
                        // Reset and restart timer
                        self.timer_model = timer::new(Duration::from_secs(60));
                        cmds.push(self.timer_model.start());
                    }
                    SessionState::SpinnerView => {
                        // Change to next spinner style
                        self.next_spinner();
                    }
                }
            }
        }

        // Update timer widget - let it process the message
        if let Some(timer_cmd) = self.timer_model.update(msg) {
            cmds.push(timer_cmd);
        }

        // Return commands
        match cmds.len() {
            0 => None,
            1 => Some(cmds.into_iter().next().unwrap()),
            _ => Some(batch(cmds)),
        }
    }

    fn view(&self) -> String {
        // Format timer display to match Go example (show as MM:SS or checkmark when done)
        let timer_display = if self.timer_model.timedout() {
            "✓".to_string()
        } else {
            let remaining = self.timer_model.view();
            // Timer widget returns duration, format as MM:SS to match Go example
            remaining
        };

        // Render timer view
        let timer_view = if self.state == SessionState::TimerView {
            Self::focused_style().render(&format!("{:>4}", timer_display))
        } else {
            Self::model_style().render(&format!("{:>4}", timer_display))
        };

        // Render spinner view using manual frame animation
        let spinner_display = self.spinner_view();
        let spinner_view = if self.state == SessionState::SpinnerView {
            Self::focused_style().render(&spinner_display)
        } else {
            Self::model_style().render(&spinner_display)
        };

        // Join horizontally (side by side)
        let views = Self::join_horizontal(&timer_view, &spinner_view);

        // Help text with styling matching Go version
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
