use bubbletea_rs::{Cmd, KeyMsg, Model as BubbleTeaModel, Msg, WindowSizeMsg};
use bubbletea_widgets::list::{Item, ItemDelegate, Model as List};
use crossterm::event::{KeyCode, KeyModifiers};
use lipgloss_extras::lipgloss::{Color, Style};
use std::fmt::Display;

// Synthetic message used to trigger the initial render immediately after startup.
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

// Simple item type (equivalent to Go's item string)
#[derive(Debug, Clone)]
struct FoodItem(String);

impl Display for FoodItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Item for FoodItem {
    fn filter_value(&self) -> String {
        self.0.clone()
    }
}

// Custom delegate that matches the Go version exactly
#[derive(Debug, Clone)]
struct FoodDelegate {
    item_style: Style,
    selected_item_style: Style,
}

impl Default for FoodDelegate {
    fn default() -> Self {
        Self {
            item_style: Style::new().padding_left(4),
            selected_item_style: Style::new()
                .padding_left(2)
                .foreground(Color::from("170")),
        }
    }
}

impl ItemDelegate<FoodItem> for FoodDelegate {
    fn render(&self, m: &List<FoodItem>, index: usize, item: &FoodItem) -> String {
        let str = format!("{}. {}", index + 1, item.0);

        if index == m.cursor() {
            self.selected_item_style.render(&format!("> {}", str))
        } else {
            self.item_style.render(&str)
        }
    }

    fn height(&self) -> usize {
        1 // Each item takes exactly 1 line
    }

    fn spacing(&self) -> usize {
        0 // No spacing between items
    }

    fn update(&self, _msg: &Msg, _m: &mut List<FoodItem>) -> Option<Cmd> {
        None
    }
}

// Main application model
struct Model {
    list: List<FoodItem>,
    choice: Option<String>,
    quitting: bool,
}

impl Model {
    fn new() -> Self {
        let items = vec![
            FoodItem("Ramen".to_string()),
            FoodItem("Tomato Soup".to_string()),
            FoodItem("Hamburgers".to_string()),
            FoodItem("Cheeseburgers".to_string()),
            FoodItem("Currywurst".to_string()),
            FoodItem("Okonomiyaki".to_string()),
            FoodItem("Pasta".to_string()),
            FoodItem("Fillet Mignon".to_string()),
            FoodItem("Caviar".to_string()),
            FoodItem("Just Wine".to_string()),
        ];

        let delegate = FoodDelegate::default();
        let mut list = List::new(items, delegate, 80, 30) // Much larger dimensions to force vertical
            .with_title("What do you want for dinner?");

        // Note: Filtering behavior will be handled by the widget defaults

        // Configure list styles to match Go example exactly
        let title_style = Style::new().margin_left(2);
        let pagination_style = Style::new().padding_left(4);
        let help_style = Style::new().padding_left(4).padding_bottom(1);

        list.styles.title = title_style;
        list.styles.pagination_style = pagination_style;
        list.styles.help_style = help_style;

        Self {
            list,
            choice: None,
            quitting: false,
        }
    }
}

impl BubbleTeaModel for Model {
    fn init() -> (Self, Option<Cmd>) {
        let model = Self::new();
        (model, Some(init_render_cmd()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle initial render message
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            // Just trigger a render, no state change needed
            return None;
        }
        
        // Handle window size changes
        if let Some(_size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            // List widget handles resizing internally
            return None;
        }

        // Handle key messages
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match &key_msg.key {
                KeyCode::Char('q') => {
                    self.quitting = true;
                    return Some(bubbletea_rs::quit());
                }
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quitting = true;
                    return Some(bubbletea_rs::quit());
                }
                KeyCode::Enter => {
                    if let Some(item) = self.list.selected_item() {
                        self.choice = Some(item.0.clone());
                    }
                    return Some(bubbletea_rs::quit());
                }
                _ => {}
            }
        }

        // Delegate to list for navigation
        self.list.update(msg)
    }

    fn view(&self) -> String {
        let quit_text_style = Style::new().margin(1, 0, 2, 4);

        if let Some(choice) = &self.choice {
            return quit_text_style.render(&format!("{}? Sounds good to me.", choice));
        }
        if self.quitting {
            return quit_text_style.render("Not hungry? That's cool.");
        }

        format!("\n{}", self.list.view())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Try to ensure clean terminal state
    crossterm::terminal::enable_raw_mode()?;

    let result = {
        let program = bubbletea_rs::Program::<Model>::builder()
            .alt_screen(false) // Disable alt_screen to help with layout
            .signal_handler(true)
            .build()?;

        program.run().await
    };

    // Always restore terminal state, even on error
    let _ = crossterm::terminal::disable_raw_mode();

    result?;
    Ok(())
}