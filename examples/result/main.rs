//! Result Example
//!
//! Demonstrates:
//! - Menu navigation with up/down arrow keys
//! - Option selection with Enter key
//! - Result display after selection
//! - Keyboard controls for navigation and quitting
//!
//! This example shows a choice menu where users can select an option
//! and see the result of their selection.

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::KeyCode;

/// Represents the different menu options available
#[derive(Debug, Clone, PartialEq)]
pub enum Choice {
    Option1,
    Option2,
    Option3,
    Option4,
}

impl Choice {
    pub fn as_str(&self) -> &'static str {
        match self {
            Choice::Option1 => "Continue",
            Choice::Option2 => "Settings",
            Choice::Option3 => "Help",
            Choice::Option4 => "Exit",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Choice::Option1 => "Proceed with the current operation",
            Choice::Option2 => "Configure application settings",
            Choice::Option3 => "Get help and documentation",
            Choice::Option4 => "Exit the application",
        }
    }
}

/// The application state
#[derive(Debug)]
pub struct ResultModel {
    pub choices: Vec<Choice>,
    pub cursor: usize,
    pub selected: Option<Choice>,
}

impl Model for ResultModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = ResultModel {
            choices: vec![
                Choice::Option1,
                Choice::Option2,
                Choice::Option3,
                Choice::Option4,
            ],
            cursor: 0,
            selected: None,
        };
        (model, None)
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // If a selection has been made, only allow quitting
        if self.selected.is_some() {
            if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
                match key_msg.key {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        return Some(quit());
                    }
                    _ => {
                        return Some(quit()); // Any key quits after selection
                    }
                }
            }
            return None;
        }

        // Handle keyboard input for menu navigation
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    return Some(quit());
                }
                KeyCode::Up => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.cursor < self.choices.len() - 1 {
                        self.cursor += 1;
                    }
                }
                KeyCode::Enter => {
                    // Make a selection
                    self.selected = Some(self.choices[self.cursor].clone());

                    // If "Exit" was selected, quit immediately
                    if self.selected == Some(Choice::Option4) {
                        return Some(quit());
                    }
                }
                _ => {}
            }
        }

        None
    }

    fn view(&self) -> String {
        // Show result if selection has been made
        if let Some(selected) = &self.selected {
            return format!(
                "You selected: {}\n\n{}\n\nPress any key to exit.",
                selected.as_str(),
                selected.description()
            );
        }

        // Show menu for selection
        let mut s = String::new();
        s.push_str("What would you like to do?\n\n");

        for (i, choice) in self.choices.iter().enumerate() {
            let cursor_symbol = if i == self.cursor { "→" } else { " " };
            s.push_str(&format!(" {} {}\n", cursor_symbol, choice.as_str()));
        }

        s.push_str("\nUse ↑/↓ to navigate, Enter to select, q/ESC to quit.");
        s
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting result example...");

    // Create and run the program
    let program = Program::<ResultModel>::builder()
        .alt_screen(true) // Use alternate screen for cleaner display
        .signal_handler(true) // Enable Ctrl+C handling
        .build()?;

    // Run the program and get the final model state
    let final_model = program.run().await?;

    if let Some(selected) = &final_model.selected {
        println!("You selected: {}", selected.as_str());
    } else {
        println!("No selection made.");
    }

    Ok(())
}
