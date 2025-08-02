//! List Default Example
//!
//! Demonstrates:
//! - Rich list items with titles and descriptions
//! - Default styling with professional appearance
//! - Built-in list features like filtering and pagination
//! - Window resize handling and responsive design
//! - Status bar and help text functionality
//!
//! This example shows a "My Fave Things" list with sophisticated styling
//! and full-featured list capabilities using default delegates.

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
use crossterm::event::{KeyCode, KeyModifiers};

/// Represents a rich list item with title and description
#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub title: String,
    pub description: String,
}

impl ListItem {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
        }
    }

    pub fn filter_value(&self) -> &str {
        &self.title
    }
}

/// The application state
#[derive(Debug)]
pub struct ListDefaultModel {
    pub items: Vec<ListItem>,
    pub cursor: usize,
    pub selected: Option<usize>,
    pub filter_text: String,
    pub show_help: bool,
    pub show_status: bool,
    pub filtered_indices: Vec<usize>,
    pub window_width: usize,
    pub window_height: usize,
    pub view_offset: usize,
    pub items_per_page: usize,
}

impl ListDefaultModel {
    pub fn new(items: Vec<ListItem>) -> Self {
        let filtered_indices = (0..items.len()).collect();
        Self {
            items,
            cursor: 0,
            selected: None,
            filter_text: String::new(),
            show_help: true,
            show_status: true,
            filtered_indices,
            window_width: 80,
            window_height: 24,
            view_offset: 0,
            items_per_page: 20,
        }
    }

    pub fn filtered_items(&self) -> Vec<&ListItem> {
        self.filtered_indices
            .iter()
            .map(|&i| &self.items[i])
            .collect()
    }

    pub fn current_item(&self) -> Option<&ListItem> {
        if self.cursor < self.filtered_indices.len() {
            Some(&self.items[self.filtered_indices[self.cursor]])
        } else {
            None
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.adjust_view_offset();
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor < self.filtered_indices.len().saturating_sub(1) {
            self.cursor += 1;
            self.adjust_view_offset();
        }
    }

    pub fn adjust_view_offset(&mut self) {
        let visible_items = self.items_per_page;

        if self.cursor < self.view_offset {
            self.view_offset = self.cursor;
        } else if self.cursor >= self.view_offset + visible_items {
            self.view_offset = self.cursor.saturating_sub(visible_items - 1);
        }
    }

    pub fn select_current(&mut self) {
        if self.cursor < self.filtered_indices.len() {
            self.selected = Some(self.filtered_indices[self.cursor]);
        }
    }

    pub fn apply_filter(&mut self) {
        if self.filter_text.is_empty() {
            self.filtered_indices = (0..self.items.len()).collect();
        } else {
            self.filtered_indices = self
                .items
                .iter()
                .enumerate()
                .filter(|(_, item)| {
                    item.title
                        .to_lowercase()
                        .contains(&self.filter_text.to_lowercase())
                        || item
                            .description
                            .to_lowercase()
                            .contains(&self.filter_text.to_lowercase())
                })
                .map(|(i, _)| i)
                .collect();
        }

        // Reset cursor to first item after filtering
        self.cursor = 0;
        self.view_offset = 0;
    }

    pub fn update_window_size(&mut self, width: u16, height: u16) {
        self.window_width = width as usize;
        self.window_height = height as usize;

        // Reserve space for title, status, help, and borders
        self.items_per_page = (height as usize).saturating_sub(6);
        self.adjust_view_offset();
    }

    pub fn render_item(&self, index: usize, item: &ListItem) -> (String, String) {
        let is_selected = index == self.cursor;
        let number = index + 1;

        let title = if is_selected {
            format!("▸ {}. {}", number, item.title)
        } else {
            format!("  {}. {}", number, item.title)
        };

        let description = if is_selected {
            format!("   {}", item.description)
        } else {
            format!("   {}", item.description)
        };

        (title, description)
    }
}

impl Model for ListDefaultModel {
    fn init() -> (Self, Option<Cmd>) {
        let items = vec![
            ListItem::new("Raspberry Pi's", "I have 'em all over my house"),
            ListItem::new("Nutella", "It's good on toast"),
            ListItem::new("Bitter melon", "It cools you down"),
            ListItem::new("Nice socks", "And by that I mean socks without holes"),
            ListItem::new("Eight hours of sleep", "I had this once"),
            ListItem::new("Cats", "Usually"),
            ListItem::new("Plantasia, the album", "My plants love it too"),
            ListItem::new("Pour over coffee", "It takes forever to make though"),
            ListItem::new("VR", "Virtual reality...what is there to say?"),
            ListItem::new("Noguchi Lamps", "Such pleasing organic forms"),
            ListItem::new("Linux", "Pretty much the best OS"),
            ListItem::new("Business school", "Just kidding"),
            ListItem::new("Pottery", "Wet clay is a great feeling"),
            ListItem::new("Shampoo", "Nothing like a good hair wash"),
            ListItem::new("Table tennis", "It's surprisingly exhausting"),
            ListItem::new("Milk crates", "Great for packing in your extra stuff"),
            ListItem::new("Afternoon tea", "Especially the tea sandwich part"),
            ListItem::new("Stickers", "The thicker the better"),
            ListItem::new("20° Weather", "Celsius, not Fahrenheit"),
            ListItem::new("Warm light", "Like around 2700 Kelvin"),
            ListItem::new(
                "The vernal equinox",
                "The autumnal equinox is pretty good too",
            ),
            ListItem::new("Gaffer's tape", "Basically sticky fabric"),
            ListItem::new("Terrycloth", "In other words, towel fabric"),
        ];

        let model = ListDefaultModel::new(items);
        (model, None)
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle window size changes
        if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            self.update_window_size(size_msg.width, size_msg.height);
            return None;
        }

        // Handle keyboard input
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit());
                }
                KeyCode::Esc => {
                    if !self.filter_text.is_empty() {
                        // Clear filter on Esc
                        self.filter_text.clear();
                        self.apply_filter();
                    } else {
                        return Some(quit());
                    }
                }
                KeyCode::Up => {
                    self.move_cursor_up();
                }
                KeyCode::Down => {
                    self.move_cursor_down();
                }
                KeyCode::Enter => {
                    self.select_current();
                    return Some(quit());
                }
                KeyCode::Char('q') => {
                    return Some(quit());
                }
                KeyCode::Char('h') => {
                    self.show_help = !self.show_help;
                }
                KeyCode::Char('s') => {
                    self.show_status = !self.show_status;
                }
                KeyCode::Char('/') => {
                    // Start filtering mode (simplified - just add to filter)
                    // In a real implementation, you'd enter a special filtering mode
                }
                KeyCode::Char(c) if c.is_alphanumeric() || c == ' ' => {
                    // Add to filter
                    self.filter_text.push(c);
                    self.apply_filter();
                }
                KeyCode::Backspace => {
                    if !self.filter_text.is_empty() {
                        self.filter_text.pop();
                        self.apply_filter();
                    }
                }
                _ => {}
            }
        }

        None
    }

    fn view(&self) -> String {
        let mut view = String::new();

        // Title
        view.push_str("  My Fave Things\n");
        view.push_str("  ═══════════════\n\n");

        // Show filter if active
        if !self.filter_text.is_empty() {
            view.push_str(&format!("  Filter: {}\n\n", self.filter_text));
        }

        // List items
        let filtered_items = self.filtered_items();
        let visible_items = self.items_per_page.min(filtered_items.len());
        let end_index = (self.view_offset + visible_items).min(filtered_items.len());

        for i in self.view_offset..end_index {
            if i < filtered_items.len() {
                let item = filtered_items[i];
                let (title_line, desc_line) = self.render_item(i, item);
                view.push_str(&title_line);
                view.push('\n');
                view.push_str(&desc_line);
                view.push_str("\n\n");
            }
        }

        // Status bar
        if self.show_status {
            let current_pos = self.cursor + 1;
            let total_items = filtered_items.len();
            view.push_str(
                "  ─────────────────────────────────────────────────────────────────────────\n",
            );
            view.push_str(&format!("  {} of {} items", current_pos, total_items));
            if !self.filter_text.is_empty() {
                view.push_str(&format!(" (filtered from {})", self.items.len()));
            }
            view.push('\n');
        }

        // Help text
        if self.show_help {
            view.push_str(
                "  ─────────────────────────────────────────────────────────────────────────\n",
            );
            view.push_str("  ↑/↓: navigate • enter: select • q/ctrl+c: quit • h: toggle help\n");
            view.push_str("  type: filter • backspace: clear filter • esc: clear/quit\n");
        }

        view
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting list-default example...");

    // Create and run the program
    let program = Program::<ListDefaultModel>::builder()
        .alt_screen(true) // Use alternate screen for cleaner display
        .signal_handler(true) // Enable Ctrl+C handling
        .build()?;

    // Run the program and get the final model state
    let final_model = program.run().await?;

    if let Some(selected_idx) = final_model.selected {
        let selected_item = &final_model.items[selected_idx];
        println!("You selected: {}", selected_item.title);
        println!("Description: {}", selected_item.description);
    } else {
        println!("No selection made.");
    }

    Ok(())
}
