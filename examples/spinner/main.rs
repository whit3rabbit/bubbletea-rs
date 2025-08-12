//! Spinner Example
//!
//! A simple program demonstrating the spinner component, matching the Go
//! Bubble Tea spinner example.
//!
//! This example shows a simple loading spinner that runs forever until
//! the user quits, demonstrating basic spinner functionality.

use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};
use lipgloss_extras::lipgloss::{Color, Style};
use std::time::Duration;

/// Message for spinner animation ticks
#[derive(Debug)]
pub struct SpinnerTickMsg;

/// Error message type
#[derive(Debug)]
pub struct ErrMsg(String);

/// The application model
#[derive(Debug)]
pub struct SpinnerModel {
    current_frame: usize,
    quitting: bool,
    err: Option<String>,
}

impl SpinnerModel {
    /// Create a new spinner model
    fn new() -> Self {
        Self {
            current_frame: 0,
            quitting: false,
            err: None,
        }
    }

    /// Get the dot spinner frames (matching Go version)
    fn frames() -> &'static [&'static str] {
        &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
    }

    /// Get the spinner interval
    fn interval() -> Duration {
        Duration::from_millis(100)
    }

    /// Get the current spinner frame with pink styling (#205)
    fn current_spinner_frame(&self) -> String {
        let frames = Self::frames();
        let frame = frames[self.current_frame % frames.len()];
        
        // Apply pink styling (#205) to match Go version exactly
        let style = Style::new().foreground(Color::from("205"));
        style.render(frame)
    }

    /// Advance to the next frame
    fn advance_frame(&mut self) {
        let frames = Self::frames();
        self.current_frame = (self.current_frame + 1) % frames.len();
    }

    /// Set an error
    fn set_error(&mut self, error: String) {
        self.err = Some(error);
    }
}

impl Model for SpinnerModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = SpinnerModel::new();
        
        // Start the spinner animation
        let cmd = tick(Self::interval(), |_| Box::new(SpinnerTickMsg) as Msg);
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle spinner tick messages
        if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
            if !self.quitting {
                self.advance_frame();
                return Some(tick(Self::interval(), |_| Box::new(SpinnerTickMsg) as Msg));
            }
        }

        // Handle error messages
        if let Some(err_msg) = msg.downcast_ref::<ErrMsg>() {
            self.set_error(err_msg.0.clone());
            return None;
        }

        // Handle keyboard input - matching Go version exactly
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quitting = true;
                    return Some(quit());
                }
                _ => return None,
            }
        }

        None
    }

    fn view(&self) -> String {
        // Handle errors first (matching Go version)
        if let Some(error) = &self.err {
            return error.clone();
        }

        // Main view - matching Go version format exactly
        let str = format!(
            "\n\n   {} Loading forever...press q to quit\n\n",
            self.current_spinner_frame()
        );
        
        if self.quitting {
            return str + "\n";
        }
        
        str
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the program
    let program = Program::<SpinnerModel>::builder()
        .signal_handler(true)
        .build()?;

    // Run the program
    if let Err(err) = program.run().await {
        println!("{}", err);
        std::process::exit(1);
    }

    Ok(())
}