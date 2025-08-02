//! Textarea Example
//!
//! Demonstrates:
//! - Multi-line text editing with cursor navigation
//! - Line wrapping and scrolling
//! - Focus/blur state management
//! - Text area with placeholder text
//! - Keyboard controls for multi-line input
//!
//! This example shows a text area where users can write multi-line text
//! with proper cursor handling and line navigation.

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};

/// Message for cursor blinking
#[derive(Debug)]
pub struct BlinkMsg;

/// The application state
#[derive(Debug)]
pub struct TextAreaModel {
    pub content: Vec<String>, // Lines of text
    pub cursor_line: usize,   // Current line index
    pub cursor_col: usize,    // Current column position
    pub placeholder: String,  // Placeholder text
    pub focused: bool,        // Whether textarea is focused
    pub show_cursor: bool,    // Cursor visibility for blinking
    pub scroll_offset: usize, // TODO: For scrolling long content (not yet implemented)
    pub height: usize,        // Visible height of textarea
    pub width: usize,         // Width of textarea
}

impl TextAreaModel {
    pub fn new() -> Self {
        Self {
            content: vec![String::new()],
            cursor_line: 0,
            cursor_col: 0,
            placeholder: "Once upon a time...".to_string(),
            focused: true,
            show_cursor: true,
            scroll_offset: 0,
            height: 5,
            width: 50,
        }
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn blur(&mut self) {
        self.focused = false;
    }

    pub fn is_empty(&self) -> bool {
        self.content.len() == 1 && self.content[0].is_empty()
    }

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_line >= self.content.len() {
            self.content.push(String::new());
        }

        self.content[self.cursor_line].insert(self.cursor_col, c);
        self.cursor_col += 1;
    }

    pub fn insert_newline(&mut self) {
        if self.cursor_line >= self.content.len() {
            self.content.push(String::new());
        }

        let current_line = self.content[self.cursor_line].clone();
        let (left, right) = current_line.split_at(self.cursor_col);

        // Update current line with left part
        self.content[self.cursor_line] = left.to_string();

        // Insert new line with right part
        self.cursor_line += 1;
        self.content.insert(self.cursor_line, right.to_string());
        self.cursor_col = 0;
    }

    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            // Remove character from current line
            self.cursor_col -= 1;
            self.content[self.cursor_line].remove(self.cursor_col);
        } else if self.cursor_line > 0 {
            // Join with previous line
            let current_line = self.content.remove(self.cursor_line);
            self.cursor_line -= 1;
            self.cursor_col = self.content[self.cursor_line].len();
            self.content[self.cursor_line].push_str(&current_line);
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_line >= self.content.len() {
            return;
        }

        let current_line = &mut self.content[self.cursor_line];
        if self.cursor_col < current_line.len() {
            // Delete character at cursor
            current_line.remove(self.cursor_col);
        } else if self.cursor_line < self.content.len() - 1 {
            // Join with next line
            let next_line = self.content.remove(self.cursor_line + 1);
            self.content[self.cursor_line].push_str(&next_line);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.cursor_col = self.content[self.cursor_line].len();
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_line >= self.content.len() {
            return;
        }

        if self.cursor_col < self.content[self.cursor_line].len() {
            self.cursor_col += 1;
        } else if self.cursor_line < self.content.len() - 1 {
            self.cursor_line += 1;
            self.cursor_col = 0;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            let line_len = self.content[self.cursor_line].len();
            if self.cursor_col > line_len {
                self.cursor_col = line_len;
            }
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_line < self.content.len() - 1 {
            self.cursor_line += 1;
            let line_len = self.content[self.cursor_line].len();
            if self.cursor_col > line_len {
                self.cursor_col = line_len;
            }
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.cursor_col = 0;
    }

    pub fn move_cursor_end(&mut self) {
        if self.cursor_line < self.content.len() {
            self.cursor_col = self.content[self.cursor_line].len();
        }
    }

    pub fn get_display_content(&self) -> Vec<String> {
        if self.is_empty() && !self.focused {
            // Show placeholder when empty and not focused
            self.placeholder.lines().map(|s| s.to_string()).collect()
        } else {
            // Show actual content with cursor if focused
            let mut display_lines = self.content.clone();

            if self.focused && self.show_cursor {
                if self.cursor_line < display_lines.len() {
                    let line = &mut display_lines[self.cursor_line];
                    if self.cursor_col >= line.len() {
                        line.push('│');
                    } else {
                        line.insert(self.cursor_col, '│');
                    }
                }
            }

            display_lines
        }
    }
}

impl Model for TextAreaModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = TextAreaModel::new();

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
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit());
                }
                KeyCode::Esc => {
                    if self.focused {
                        self.blur();
                    } else {
                        return Some(quit());
                    }
                    self.show_cursor = true;
                }
                _ if !self.focused => {
                    // Re-focus on any key when not focused
                    self.focus();
                    self.show_cursor = true;
                }
                KeyCode::Char(c) => {
                    self.insert_char(c);
                    self.show_cursor = true;
                }
                KeyCode::Enter => {
                    self.insert_newline();
                    self.show_cursor = true;
                }
                KeyCode::Backspace => {
                    self.backspace();
                    self.show_cursor = true;
                }
                KeyCode::Delete => {
                    self.delete_char();
                    self.show_cursor = true;
                }
                KeyCode::Left => {
                    self.move_cursor_left();
                    self.show_cursor = true;
                }
                KeyCode::Right => {
                    self.move_cursor_right();
                    self.show_cursor = true;
                }
                KeyCode::Up => {
                    self.move_cursor_up();
                    self.show_cursor = true;
                }
                KeyCode::Down => {
                    self.move_cursor_down();
                    self.show_cursor = true;
                }
                KeyCode::Home => {
                    self.move_cursor_home();
                    self.show_cursor = true;
                }
                KeyCode::End => {
                    self.move_cursor_end();
                    self.show_cursor = true;
                }
                _ => {}
            }
        }

        None
    }

    fn view(&self) -> String {
        let mut view = String::from("Tell me a story.\n\n");

        let display_content = self.get_display_content();

        // Create bordered textarea view
        view.push_str("┌");
        view.push_str(&"─".repeat(self.width));
        view.push_str("┐\n");

        // Show content lines (with padding to height)
        // TODO: Use scroll_offset for vertical scrolling when content exceeds height
        for i in 0..self.height {
            view.push('│');
            if i < display_content.len() {
                let line = &display_content[i];
                if line.len() > self.width {
                    // Truncate long lines
                    view.push_str(&line[..self.width]);
                } else {
                    // Pad short lines
                    view.push_str(line);
                    view.push_str(&" ".repeat(self.width - line.len()));
                }
            } else {
                // Empty line padding
                if i == 0 && self.is_empty() && !self.focused {
                    // Show placeholder on first empty line when not focused
                    let placeholder_text = if self.placeholder.len() > self.width {
                        &self.placeholder[..self.width]
                    } else {
                        &self.placeholder
                    };
                    view.push_str(placeholder_text);
                    view.push_str(&" ".repeat(self.width - placeholder_text.len()));
                } else {
                    view.push_str(&" ".repeat(self.width));
                }
            }
            view.push('│');
            view.push('\n');
        }

        view.push_str("└");
        view.push_str(&"─".repeat(self.width));
        view.push_str("┘\n");

        view.push_str("\n(ctrl+c to quit, esc to blur/focus)");

        view
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting textarea example...");

    // Create and run the program
    let program = Program::<TextAreaModel>::builder()
        .alt_screen(true) // Use alternate screen for cleaner display
        .signal_handler(true) // Enable Ctrl+C handling
        .build()?;

    // Run the program and get the final model state
    let final_model = program.run().await?;

    if !final_model.is_empty() {
        println!("Your story:");
        for line in &final_model.content {
            println!("{}", line);
        }
    } else {
        println!("No story written.");
    }

    Ok(())
}
