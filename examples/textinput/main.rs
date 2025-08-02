//! Text Input Example
//!
//! Demonstrates:
//! - Text input handling with character insertion
//! - Cursor movement and text editing
//! - Character limits and placeholders
//! - Basic text field functionality
//!
//! This example shows a simple text input field where users can type
//! their favorite Pokémon name with a character limit.

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};

/// Message for cursor blinking
#[derive(Debug)]
pub struct BlinkMsg;

/// The application state
#[derive(Debug)]
pub struct TextInputModel {
    pub input: String,
    pub cursor_pos: usize,
    pub placeholder: String,
    pub char_limit: usize,
    pub width: usize,
    pub focused: bool,
    pub show_cursor: bool,
    pub error: Option<String>,
}

impl Model for TextInputModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = TextInputModel {
            input: String::new(),
            cursor_pos: 0,
            placeholder: "Pikachu".to_string(),
            char_limit: 156,
            width: 20,
            focused: true,
            show_cursor: true,
            error: None,
        };

        // Start cursor blinking
        (
            model,
            Some(bubbletea_rs::tick(
                std::time::Duration::from_millis(500),
                |_| Box::new(BlinkMsg) as Msg,
            )),
        )
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle cursor blink messages
        if msg.downcast_ref::<BlinkMsg>().is_some() {
            self.show_cursor = !self.show_cursor;
            return Some(bubbletea_rs::tick(
                std::time::Duration::from_millis(500),
                |_| Box::new(BlinkMsg) as Msg,
            ));
        }

        // Handle keyboard input
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Enter | KeyCode::Esc => {
                    return Some(quit());
                }
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit());
                }
                KeyCode::Char(c) => {
                    if self.input.len() < self.char_limit {
                        // Insert character at cursor position
                        self.input.insert(self.cursor_pos, c);
                        self.cursor_pos += 1;
                        self.show_cursor = true; // Show cursor when typing
                    }
                }
                KeyCode::Backspace => {
                    if self.cursor_pos > 0 {
                        self.cursor_pos -= 1;
                        self.input.remove(self.cursor_pos);
                        self.show_cursor = true; // Show cursor when editing
                    }
                }
                KeyCode::Delete => {
                    if self.cursor_pos < self.input.len() {
                        self.input.remove(self.cursor_pos);
                        self.show_cursor = true; // Show cursor when editing
                    }
                }
                KeyCode::Left => {
                    if self.cursor_pos > 0 {
                        self.cursor_pos -= 1;
                        self.show_cursor = true; // Show cursor when moving
                    }
                }
                KeyCode::Right => {
                    if self.cursor_pos < self.input.len() {
                        self.cursor_pos += 1;
                        self.show_cursor = true; // Show cursor when moving
                    }
                }
                KeyCode::Home => {
                    self.cursor_pos = 0;
                    self.show_cursor = true;
                }
                KeyCode::End => {
                    self.cursor_pos = self.input.len();
                    self.show_cursor = true;
                }
                _ => {}
            }
        }

        None
    }

    fn view(&self) -> String {
        let mut display_text = if self.input.is_empty() {
            self.placeholder.clone()
        } else {
            self.input.clone()
        };

        // Truncate to width if necessary
        if display_text.len() > self.width {
            display_text.truncate(self.width);
        }

        // Add cursor if focused and should show
        let text_with_cursor = if self.focused && self.show_cursor {
            if self.input.is_empty() {
                format!("│{}", &self.placeholder)
            } else {
                let mut chars: Vec<char> = self.input.chars().collect();
                if self.cursor_pos >= chars.len() {
                    chars.push('│');
                } else {
                    chars.insert(self.cursor_pos, '│');
                }
                chars.into_iter().collect()
            }
        } else {
            display_text
        };

        let input_display = if self.input.is_empty() {
            format!("[ {} ]", text_with_cursor)
        } else {
            format!("[ {} ]", text_with_cursor)
        };

        format!(
            "What's your favorite Pokémon?\n\n{}\n\n(esc to quit)",
            input_display
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting textinput example...");

    // Create and run the program
    let program = Program::<TextInputModel>::builder()
        .alt_screen(true) // Use alternate screen for cleaner display
        .signal_handler(true) // Enable Ctrl+C handling
        .build()?;

    // Run the program and get the final model state
    let final_model = program.run().await?;

    if !final_model.input.is_empty() {
        println!("You entered: {}", final_model.input);
    } else {
        println!("No input provided.");
    }

    Ok(())
}
