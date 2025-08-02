//! Text Inputs Example (Multiple Fields)
//!
//! Demonstrates:
//! - Multiple text input fields in a form
//! - Focus switching between inputs with Tab/Shift+Tab
//! - Different input types (text, email, password)
//! - Form submission and validation
//! - Cursor blinking and field styling
//!
//! This example shows a registration form with nickname, email, and password fields.

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};

/// Message for cursor blinking
#[derive(Debug)]
pub struct BlinkMsg;

/// Represents different types of input fields
#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    Text,
    Email,
    Password,
}

/// A single text input field
#[derive(Debug, Clone)]
pub struct TextInput {
    pub value: String,
    pub cursor_pos: usize,
    pub placeholder: String,
    pub char_limit: usize,
    pub width: usize,
    pub input_type: InputType,
    pub focused: bool,
}

impl TextInput {
    pub fn new(placeholder: &str, input_type: InputType) -> Self {
        Self {
            value: String::new(),
            cursor_pos: 0,
            placeholder: placeholder.to_string(),
            char_limit: if input_type == InputType::Email {
                64
            } else {
                32
            },
            width: 20,
            input_type,
            focused: false,
        }
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn blur(&mut self) {
        self.focused = false;
    }

    pub fn insert_char(&mut self, c: char) {
        if self.value.len() < self.char_limit {
            self.value.insert(self.cursor_pos, c);
            self.cursor_pos += 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_pos < self.value.len() {
            self.value.remove(self.cursor_pos);
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.value.remove(self.cursor_pos);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.value.len() {
            self.cursor_pos += 1;
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.cursor_pos = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_pos = self.value.len();
    }

    pub fn render(&self, show_cursor: bool) -> String {
        let display_value = if self.value.is_empty() {
            if self.focused {
                self.placeholder.clone()
            } else {
                self.placeholder.clone()
            }
        } else {
            match self.input_type {
                InputType::Password => "*".repeat(self.value.len()),
                _ => self.value.clone(),
            }
        };

        let text_with_cursor = if self.focused && show_cursor && !self.value.is_empty() {
            let mut chars: Vec<char> = display_value.chars().collect();
            if self.cursor_pos >= chars.len() {
                chars.push('│');
            } else {
                chars.insert(self.cursor_pos, '│');
            }
            let full_text: String = chars.into_iter().collect();

            // Truncate to width if necessary
            if full_text.chars().count() > self.width {
                full_text.chars().take(self.width).collect()
            } else {
                full_text
            }
        } else if self.focused && show_cursor && self.value.is_empty() {
            let placeholder_with_cursor = format!("│{}", self.placeholder);
            // Truncate placeholder to width if necessary
            if placeholder_with_cursor.chars().count() > self.width {
                placeholder_with_cursor.chars().take(self.width).collect()
            } else {
                placeholder_with_cursor
            }
        } else {
            // Truncate display value to width if necessary
            if display_value.chars().count() > self.width {
                display_value.chars().take(self.width).collect()
            } else {
                display_value
            }
        };

        let focus_indicator = if self.focused { "→" } else { " " };
        let style_open = if self.focused { "[" } else { " " };
        let style_close = if self.focused { "]" } else { " " };

        format!(
            "{} {}{}{}",
            focus_indicator, style_open, text_with_cursor, style_close
        )
    }
}

/// The application state
#[derive(Debug)]
pub struct TextInputsModel {
    pub inputs: Vec<TextInput>,
    pub focus_index: usize,
    pub show_cursor: bool,
    pub submitted: bool,
    pub submit_focused: bool,
}

impl Model for TextInputsModel {
    fn init() -> (Self, Option<Cmd>) {
        let mut inputs = vec![
            TextInput::new("Nickname", InputType::Text),
            TextInput::new("Email", InputType::Email),
            TextInput::new("Password", InputType::Password),
        ];

        // Focus the first input
        inputs[0].focus();

        let model = TextInputsModel {
            inputs,
            focus_index: 0,
            show_cursor: true,
            submitted: false,
            submit_focused: false,
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
                KeyCode::Esc => {
                    return Some(quit());
                }
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit());
                }
                KeyCode::Tab if key_msg.modifiers.contains(KeyModifiers::SHIFT) => {
                    // Move focus to previous input
                    if self.submit_focused {
                        self.submit_focused = false;
                        self.focus_index = self.inputs.len() - 1;
                        self.inputs[self.focus_index].focus();
                    } else if self.focus_index > 0 {
                        self.inputs[self.focus_index].blur();
                        self.focus_index -= 1;
                        self.inputs[self.focus_index].focus();
                    } else {
                        // Wrap to submit button
                        self.inputs[self.focus_index].blur();
                        self.submit_focused = true;
                    }
                    self.show_cursor = true;
                }
                KeyCode::Tab => {
                    // Move focus to next input
                    if self.submit_focused {
                        // Wrap to first input
                        self.submit_focused = false;
                        self.focus_index = 0;
                        self.inputs[self.focus_index].focus();
                    } else if self.focus_index < self.inputs.len() - 1 {
                        self.inputs[self.focus_index].blur();
                        self.focus_index += 1;
                        self.inputs[self.focus_index].focus();
                    } else {
                        // Move to submit button
                        self.inputs[self.focus_index].blur();
                        self.submit_focused = true;
                    }
                    self.show_cursor = true;
                }
                KeyCode::Enter => {
                    if self.submit_focused {
                        // Submit the form
                        self.submitted = true;
                        return Some(quit());
                    } else {
                        // Move to next field or submit
                        if self.focus_index < self.inputs.len() - 1 {
                            self.inputs[self.focus_index].blur();
                            self.focus_index += 1;
                            self.inputs[self.focus_index].focus();
                        } else {
                            self.inputs[self.focus_index].blur();
                            self.submit_focused = true;
                        }
                        self.show_cursor = true;
                    }
                }
                // Handle input for focused field
                _ if !self.submit_focused => {
                    let input = &mut self.inputs[self.focus_index];
                    match key_msg.key {
                        KeyCode::Char(c) => {
                            input.insert_char(c);
                            self.show_cursor = true;
                        }
                        KeyCode::Backspace => {
                            input.backspace();
                            self.show_cursor = true;
                        }
                        KeyCode::Delete => {
                            input.delete_char();
                            self.show_cursor = true;
                        }
                        KeyCode::Left => {
                            input.move_cursor_left();
                            self.show_cursor = true;
                        }
                        KeyCode::Right => {
                            input.move_cursor_right();
                            self.show_cursor = true;
                        }
                        KeyCode::Home => {
                            input.move_cursor_home();
                            self.show_cursor = true;
                        }
                        KeyCode::End => {
                            input.move_cursor_end();
                            self.show_cursor = true;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        None
    }

    fn view(&self) -> String {
        if self.submitted {
            let mut result = String::from("Form submitted successfully!\n\n");
            result.push_str(&format!("Nickname: {}\n", self.inputs[0].value));
            result.push_str(&format!("Email: {}\n", self.inputs[1].value));
            result.push_str(&format!(
                "Password: {}\n",
                "*".repeat(self.inputs[2].value.len())
            ));
            result.push_str("\nPress any key to exit.");
            return result;
        }

        let mut view = String::from("Registration Form\n");
        view.push_str("=================\n\n");

        // Render input fields
        for (i, input) in self.inputs.iter().enumerate() {
            let label = match i {
                0 => "Nickname:",
                1 => "Email:   ",
                2 => "Password:",
                _ => "Input:   ",
            };

            view.push_str(&format!("{} {}\n", label, input.render(self.show_cursor)));
        }

        // Render submit button
        let submit_indicator = if self.submit_focused { "→" } else { " " };
        let submit_style = if self.submit_focused {
            "[ Submit ]"
        } else {
            "  Submit  "
        };
        view.push_str(&format!("\n{} {}\n", submit_indicator, submit_style));

        view.push_str("\nNavigation:\n");
        view.push_str("• Tab/Shift+Tab to switch fields\n");
        view.push_str("• Enter to move to next field or submit\n");
        view.push_str("• Esc to quit\n");

        view
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting textinputs example...");

    // Create and run the program
    let program = Program::<TextInputsModel>::builder()
        .alt_screen(true) // Use alternate screen for cleaner display
        .signal_handler(true) // Enable Ctrl+C handling
        .build()?;

    // Run the program and get the final model state
    let final_model = program.run().await?;

    if final_model.submitted {
        println!("Registration completed!");
        println!("Nickname: {}", final_model.inputs[0].value);
        println!("Email: {}", final_model.inputs[1].value);
        println!("Password: {} characters", final_model.inputs[2].value.len());
    } else {
        println!("Registration cancelled.");
    }

    Ok(())
}
