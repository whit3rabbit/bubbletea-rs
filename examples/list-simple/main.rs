//! List Simple Example
//!
//! Demonstrates:
//! - Basic list navigation with up/down arrow keys
//! - Item selection with Enter key
//! - Simple list display with numbering
//! - Selection highlighting and cursor movement
//! - Quit options (q key or Ctrl+C)
//!
//! This example shows a simple dinner selection menu with basic list functionality.

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};

/// Represents a list item
#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub title: String,
}

impl ListItem {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

/// The application state
#[derive(Debug)]
pub struct ListSimpleModel {
    pub items: Vec<ListItem>,
    pub cursor: usize,
    pub selected: Option<usize>,
    pub choice: Option<String>,
    pub quitting: bool,
    pub height: usize,
}

impl ListSimpleModel {
    pub fn new(items: Vec<ListItem>) -> Self {
        Self {
            items,
            cursor: 0,
            selected: None,
            choice: None,
            quitting: false,
            height: 14,
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor < self.items.len().saturating_sub(1) {
            self.cursor += 1;
        }
    }

    pub fn select_current(&mut self) {
        if self.cursor < self.items.len() {
            self.selected = Some(self.cursor);
            self.choice = Some(self.items[self.cursor].title.clone());
        }
    }

    pub fn render_item(&self, index: usize, item: &ListItem) -> String {
        let number = index + 1;
        let title = &item.title;

        if index == self.cursor {
            format!("  > {}. {}", number, title)
        } else {
            format!("    {}. {}", number, title)
        }
    }
}

impl Model for ListSimpleModel {
    fn init() -> (Self, Option<Cmd>) {
        let items = vec![
            ListItem::new("Ramen"),
            ListItem::new("Tomato Soup"),
            ListItem::new("Grilled Cheese"),
            ListItem::new("Burger"),
            ListItem::new("Pizza"),
            ListItem::new("Tacos"),
            ListItem::new("Pasta"),
            ListItem::new("Salad"),
            ListItem::new("Sushi"),
            ListItem::new("Curry"),
        ];

        let model = ListSimpleModel::new(items);
        (model, None)
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // If we have a selection or are quitting, only handle quit
        if self.selected.is_some() || self.quitting {
            if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
                match key_msg.key {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        return Some(quit());
                    }
                    KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Some(quit());
                    }
                    _ => {
                        return Some(quit());
                    }
                }
            }
            return None;
        }

        // Handle keyboard input for navigation
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.quitting = true;
                }
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quitting = true;
                }
                KeyCode::Esc => {
                    self.quitting = true;
                }
                KeyCode::Up => {
                    self.move_cursor_up();
                }
                KeyCode::Down => {
                    self.move_cursor_down();
                }
                KeyCode::Enter => {
                    self.select_current();
                }
                _ => {}
            }
        }

        None
    }

    fn view(&self) -> String {
        // If we made a selection, show the choice
        if let Some(ref choice) = self.choice {
            return format!("You chose {}.\n\nPress any key to quit.", choice);
        }

        // If we're quitting without selection
        if self.quitting {
            return "Goodbye!\n\nPress any key to quit.".to_string();
        }

        // Show the list
        let mut view = String::new();

        // Title
        view.push_str("  What do you want for dinner?\n\n");

        // List items
        for (index, item) in self.items.iter().enumerate() {
            view.push_str(&self.render_item(index, item));
            view.push('\n');
        }

        // Help text
        view.push_str("\n  Use ↑/↓ to navigate, Enter to select, q to quit\n");

        view
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting list-simple example...");

    // Create and run the program
    let program = Program::<ListSimpleModel>::builder()
        .alt_screen(true) // Use alternate screen for cleaner display
        .signal_handler(true) // Enable Ctrl+C handling
        .build()?;

    // Run the program and get the final model state
    let final_model = program.run().await?;

    if let Some(ref choice) = final_model.choice {
        println!("You selected: {}", choice);
    } else if final_model.quitting {
        println!("No selection made.");
    }

    Ok(())
}
