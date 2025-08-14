//! Set Window Title Example
//!
//! A simple example illustrating how to set a window title.
//!
//! This example demonstrates:
//! - Setting the terminal window title using `set_window_title()`
//! - Basic program structure with styled output
//! - Quitting on any key press
//!
//! Note: Window title setting depends on terminal support. Some terminals
//! like tmux, screen, or certain configurations may ignore title changes.
//! Try running directly in your terminal (not through cargo run) for best results.

use bubbletea_rs::{quit, set_window_title, Cmd, KeyMsg, Model, Msg, Program};
use lipgloss_extras::lipgloss::{Color, Style};

/// Message for title updates
#[derive(Debug)]
struct TitleUpdateMsg;

/// The application model with enhanced title setting
#[derive(Debug)]
struct SetWindowTitleModel {
    title_set: bool,
}

impl Model for SetWindowTitleModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = SetWindowTitleModel { title_set: false };

        // Set the window title to "Bubble Tea Example" like the Go version
        let set_title_cmd = set_window_title("Bubble Tea Example".to_string());

        (model, Some(set_title_cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle title update confirmation
        if msg.downcast_ref::<TitleUpdateMsg>().is_some() {
            self.title_set = true;
            return None;
        }

        // Handle keyboard input - any key quits (matching Go version)
        if let Some(_key_msg) = msg.downcast_ref::<KeyMsg>() {
            return Some(quit());
        }

        None
    }

    fn view(&self) -> String {
        let mut content = String::new();

        // Title for the app
        let title_style = Style::new()
            .foreground(Color::from("5")) // Magenta
            .bold(true);
        content.push_str(&title_style.render("Set Window Title Example"));
        content.push_str("\n\n");

        // Status message
        let status_style = Style::new().foreground(Color::from("3")); // Yellow
        content
            .push_str(&status_style.render("Window title has been set to: \"Bubble Tea Example\""));
        content.push_str("\n\n");

        // Instructions with better formatting
        let instruction_style = Style::new()
            .foreground(Color::from("6")) // Cyan color
            .padding(1, 0, 0, 0); // Top padding

        content.push_str(&instruction_style.render("Press any key to quit."));

        // Add troubleshooting info
        content.push_str("\n\n");
        let help_style = Style::new()
            .foreground(Color::from("8")) // Gray
            .italic(true);
        content.push_str(&help_style.render(
            "Note: If the title doesn't change, your terminal might not support it\nor may be configured to ignore title changes."
        ));

        content
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<SetWindowTitleModel>::builder().build()?;

    // Run the program
    program.run().await?;

    Ok(())
}
