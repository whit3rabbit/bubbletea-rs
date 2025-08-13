use bubbletea_rs::{Cmd, KeyMsg, Model as BubbleTeaModel, Msg, WindowSizeMsg};
use bubbletea_widgets::key::{matches_binding, new_binding, with_help, with_keys_str, Binding};
use bubbletea_widgets::list::{Item, ItemDelegate, Model as List};
use bubbletea_widgets::paginator::Type as PaginatorType;
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
            selected_item_style: Style::new().padding_left(2).foreground(Color::from("170")),
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

// Key bindings for the application
struct AppKeyMap {
    quit: Binding,
    force_quit: Binding,
    select: Binding,
}

impl Default for AppKeyMap {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![with_keys_str(&["q"]), with_help("q", "quit")]),
            force_quit: new_binding(vec![
                with_keys_str(&["ctrl+c"]),
                with_help("ctrl+c", "force quit"),
            ]),
            select: new_binding(vec![
                with_keys_str(&["enter"]),
                with_help("enter", "select item"),
            ]),
        }
    }
}

// Main application model
struct Model {
    list: List<FoodItem>,
    choice: Option<String>,
    quitting: bool,
    keymap: AppKeyMap,
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
            .with_title("What do you want for dinner?")
            .with_pagination_type(PaginatorType::Dots); // Use dots pagination to match Go version

        // Note: Filtering behavior will be handled by the widget defaults

        // Configure list styles to match Go example exactly using the v0.1.8 API
        {
            let styles = list.styles_mut();
            styles.title = Style::new().margin_left(2);
            styles.pagination_style = Style::new().padding_left(4);
            styles.help_style = Style::new().padding_left(4).padding_bottom(1);
        }

        Self {
            list,
            choice: None,
            quitting: false,
            keymap: AppKeyMap::default(),
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

        // Handle key messages using semantic bindings
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            // Check semantic key bindings instead of raw key codes
            if matches_binding(key_msg, &self.keymap.quit) {
                self.quitting = true;
                return Some(bubbletea_rs::quit());
            } else if matches_binding(key_msg, &self.keymap.force_quit) {
                self.quitting = true;
                return Some(bubbletea_rs::quit());
            } else if matches_binding(key_msg, &self.keymap.select) {
                if let Some(item) = self.list.selected_item() {
                    self.choice = Some(item.0.clone());
                }
                return Some(bubbletea_rs::quit());
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
