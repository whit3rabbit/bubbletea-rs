//! Spinner Example
//!
//! Demonstrates:
//! - Loading spinner animation with customizable styles
//! - Timed animation updates using `every()` command
//! - Different spinner patterns and styles
//! - Continuous animation loop until user quits
//! - Error handling and state management
//!
//! This example shows a spinning loading indicator that animates continuously,
//! demonstrating how to create smooth animations in terminal applications.

use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};
use std::time::Duration;

/// Message for spinner animation ticks
#[derive(Debug)]
pub struct SpinnerTickMsg;

/// Different spinner styles available
#[derive(Debug, Clone, PartialEq)]
pub enum SpinnerStyle {
    Dots,
    Line,
    Arc,
    Bounce,
    Clock,
}

impl SpinnerStyle {
    pub fn frames(&self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Dots => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerStyle::Line => &["|", "/", "-", "\\"],
            SpinnerStyle::Arc => &["◜", "◠", "◝", "◞", "◡", "◟"],
            SpinnerStyle::Bounce => &["⠁", "⠂", "⠄", "⠂"],
            SpinnerStyle::Clock => &[
                "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
            ],
        }
    }

    pub fn interval(&self) -> Duration {
        match self {
            SpinnerStyle::Dots => Duration::from_millis(100),
            SpinnerStyle::Line => Duration::from_millis(150),
            SpinnerStyle::Arc => Duration::from_millis(120),
            SpinnerStyle::Bounce => Duration::from_millis(300),
            SpinnerStyle::Clock => Duration::from_millis(500),
        }
    }
}

/// The application state
#[derive(Debug)]
pub struct SpinnerModel {
    pub style: SpinnerStyle,
    pub current_frame: usize,
    pub message: String,
    pub quitting: bool,
    pub error: Option<String>,
}

impl SpinnerModel {
    pub fn new() -> Self {
        Self {
            style: SpinnerStyle::Dots,
            current_frame: 0,
            message: "Loading forever...press q to quit".to_string(),
            quitting: false,
            error: None,
        }
    }

    pub fn with_style(mut self, style: SpinnerStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn current_spinner_frame(&self) -> &str {
        let frames = self.style.frames();
        frames[self.current_frame % frames.len()]
    }

    pub fn advance_frame(&mut self) {
        let frames = self.style.frames();
        self.current_frame = (self.current_frame + 1) % frames.len();
    }

    pub fn change_style(&mut self) {
        self.style = match self.style {
            SpinnerStyle::Dots => SpinnerStyle::Line,
            SpinnerStyle::Line => SpinnerStyle::Arc,
            SpinnerStyle::Arc => SpinnerStyle::Bounce,
            SpinnerStyle::Bounce => SpinnerStyle::Clock,
            SpinnerStyle::Clock => SpinnerStyle::Dots,
        };
        self.current_frame = 0; // Reset animation
    }
}

impl Model for SpinnerModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = SpinnerModel::new();
        let interval = model.style.interval();

        // Start the spinner animation with a single-shot tick
        let cmd = tick(interval, |_| Box::new(SpinnerTickMsg) as Msg);
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle spinner tick messages
        if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
            if !self.quitting {
                self.advance_frame();
                let interval = self.style.interval();
                // Schedule the next single-shot tick; avoids accumulating timers
                return Some(tick(interval, |_| Box::new(SpinnerTickMsg) as Msg));
            }
        }

        // Handle keyboard input
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Char(' ') => {
                    // Space bar changes spinner style
                    self.change_style();
                    let interval = self.style.interval();
                    // Re-arm single-shot tick with new interval
                    return Some(tick(interval, |_| Box::new(SpinnerTickMsg) as Msg));
                }
                _ => {}
            }
        }

        None
    }

    fn view(&self) -> String {
        if let Some(error) = &self.error {
            return format!("Error: {}", error);
        }

        let spinner_frame = self.current_spinner_frame();
        let style_name = match self.style {
            SpinnerStyle::Dots => "Dots",
            SpinnerStyle::Line => "Line",
            SpinnerStyle::Arc => "Arc",
            SpinnerStyle::Bounce => "Bounce",
            SpinnerStyle::Clock => "Clock",
        };

        let mut view = String::new();
        view.push_str(&format!("\n\n   {} {}\n\n", spinner_frame, self.message));
        view.push_str(&format!(
            "   Style: {} (press space to change)\n",
            style_name
        ));

        if self.quitting {
            view.push('\n');
        }

        view
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting spinner example...");

    // Create and run the program
    let program = Program::<SpinnerModel>::builder()
        .alt_screen(true) // Use alternate screen for cleaner display
        .signal_handler(true) // Enable Ctrl+C handling
        .build()?;

    // Run the program
    program.run().await?;

    println!("Spinner stopped.");

    Ok(())
}
