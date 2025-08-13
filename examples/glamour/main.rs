//! Glamour Example
//!
//! TODO: This is a workaround implementation using manual styling. When a Rust equivalent
//! of the `glamour` markdown renderer becomes available, we should revisit this example
//! to use proper markdown parsing and rendering instead of manual formatting.
//!
//! This example demonstrates how to use a viewport widget from bubbletea-widgets
//! to display scrollable markdown-style content with lipgloss styling. Key features:
//!
//! - **Viewport Component**: Scrollable content display with borders and padding
//! - **Lipgloss Styling**: Rounded borders, colors, and padding
//! - **Styled Markdown Content**: Colored headers, tables, and text simulating glamour rendering
//! - **Keyboard Navigation**: Arrow keys for scrolling, multiple quit options
//! - **Responsive Layout**: Width calculations accounting for borders and padding
//!
//! This example is inspired by the Go Bubble Tea glamour example but uses manual
//! markdown formatting with colors since Rust doesn't have a direct glamour equivalent.
//! We simulate glamour rendering by applying colors and styling to different markdown elements.

use bubbletea_rs::{quit, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::{
    key::{new_binding, with_help, with_keys_str, Binding},
    viewport,
};
use lipgloss_extras::lipgloss::{border, Color, Style};
use lipgloss_extras::table::Table;

/// Simulates glamour markdown rendering with colors and formatting
fn render_glamour_content() -> String {
    // Define styles to match typical glamour themes
    let h1_style = Style::new()
        .foreground(Color::from("212")) // Bright pink
        .bold(true);
    let h2_style = Style::new()
        .foreground(Color::from("39")) // Bright blue
        .bold(true);
    let _table_header_style = Style::new()
        .foreground(Color::from("14")) // Cyan
        .bold(true);
    let _price_style = Style::new().foreground(Color::from("10")); // Green
    let checkmark_style = Style::new().foreground(Color::from("10")); // Green
    let checkbox_style = Style::new().foreground(Color::from("8")); // Gray
    let text_style = Style::new().foreground(Color::from("15")); // White/default

    let mut content = String::new();

    // Main title
    content.push_str(&h1_style.render("# Today's Menu"));
    content.push_str("\n\n");

    // Appetizers section
    content.push_str(&h2_style.render("## Appetizers"));
    content.push_str("\n\n");

    // Create appetizers table
    let mut appetizers_table = Table::new()
        .headers(vec!["Name", "Price", "Notes"])
        .rows(vec![
            vec!["Tsukemono", "$2", "Just an appetizer"],
            vec!["Tomato Soup", "$4", "Made with San Marzano tomatoes"],
            vec!["Okonomiyaki", "$4", "Takes a few minutes to make"],
            vec!["Curry", "$3", "We can add squash if you'd like"],
        ])
        .style_func_boxed(Box::new(|row, col| {
            if row == -1 {
                // Header row
                Style::new().foreground(Color::from("14")).bold(true)
            } else if col == 1 {
                // Price column
                Style::new().foreground(Color::from("10"))
            } else {
                Style::new().foreground(Color::from("15"))
            }
        }));

    content.push_str(&appetizers_table.render());
    content.push('\n');

    // Seasonal dishes section
    content.push_str(&h2_style.render("## Seasonal Dishes"));
    content.push_str("\n\n");

    // Create seasonal dishes table
    let mut seasonal_table = Table::new()
        .headers(vec!["Name", "Price", "Notes"])
        .rows(vec![
            vec!["Steamed bitter melon", "$2", "Not so bitter"],
            vec!["Takoyaki", "$3", "Fun to eat"],
            vec!["Winter squash", "$3", "Today it's pumpkin"],
        ])
        .style_func_boxed(Box::new(|row, col| {
            if row == -1 {
                // Header row
                Style::new().foreground(Color::from("14")).bold(true)
            } else if col == 1 {
                // Price column
                Style::new().foreground(Color::from("10"))
            } else {
                Style::new().foreground(Color::from("15"))
            }
        }));

    content.push_str(&seasonal_table.render());
    content.push('\n');

    // Desserts section
    content.push_str(&h2_style.render("## Desserts"));
    content.push_str("\n\n");

    // Create desserts table
    let mut desserts_table = Table::new()
        .headers(vec!["Name", "Price", "Notes"])
        .rows(vec![
            vec!["Dorayaki", "$4", "Looks good on rabbits"],
            vec!["Banana Split", "$5", "A classic"],
            vec!["Cream Puff", "$3", "Pretty creamy!"],
        ])
        .style_func_boxed(Box::new(|row, col| {
            if row == -1 {
                // Header row
                Style::new().foreground(Color::from("14")).bold(true)
            } else if col == 1 {
                // Price column
                Style::new().foreground(Color::from("10"))
            } else {
                Style::new().foreground(Color::from("15"))
            }
        }));

    content.push_str(&desserts_table.render());

    // Additional text
    content.push('\n');
    content
        .push_str(&text_style.render(
            "All our dishes are made in-house by Karen, our chef. Most of our ingredients",
        ));
    content.push('\n');
    content.push_str(&text_style.render("are from our garden or the fish market down the street."));
    content.push_str("\n\n");
    content.push_str(&text_style.render("Some famous people that have eaten here lately:"));
    content.push_str("\n\n");

    // Checkbox list with colors
    content.push_str(&format!(
        "• {} René Redzepi\n",
        checkmark_style.render("[✓]")
    ));
    content.push_str(&format!(
        "• {} David Chang\n",
        checkmark_style.render("[✓]")
    ));
    content.push_str(&format!(
        "• {} Jiro Ono (maybe some day)\n",
        checkbox_style.render("[ ]")
    ));
    content.push('\n');
    content.push_str(&text_style.render("Bon appétit!"));
    content.push('\n');

    content
}

/// Key bindings for the glamour example
#[derive(Debug)]
pub struct KeyBindings {
    pub quit: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![
                with_keys_str(&["q", "ctrl+c", "esc"]),
                with_help("q/ctrl+c/esc", "quit"),
            ]),
        }
    }
}

/// The model containing the viewport and styling
#[derive(Debug)]
pub struct GlamourModel {
    /// Viewport widget for scrollable content
    viewport: viewport::Model,
    /// Key bindings for user input
    keys: KeyBindings,
    /// Style for help text
    help_style: Style,
    /// Style for the viewport border
    viewport_style: Style,
}

impl GlamourModel {
    /// Creates a new glamour model with styled viewport
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        const WIDTH: usize = 78;
        const HEIGHT: usize = 20;

        // Create viewport with specified dimensions
        let mut vp = viewport::new(WIDTH, HEIGHT);

        // Set the styled content in the viewport
        let rendered_content = render_glamour_content();
        vp.set_content(&rendered_content);

        // Create lipgloss styling to match the Go example
        let viewport_style = Style::new()
            .border(border::rounded_border())
            .border_foreground(Color::from("62"))
            .padding_right(2);

        // Create help text style (dim gray)
        let help_style = Style::new().foreground(Color::from("241"));

        Ok(GlamourModel {
            viewport: vp,
            keys: KeyBindings::default(),
            help_style,
            viewport_style,
        })
    }

    /// Renders the help text at the bottom
    fn help_view(&self) -> String {
        self.help_style.render("\n  ↑/↓: Navigate • q: Quit\n")
    }
}

impl Model for GlamourModel {
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) {
        match GlamourModel::new() {
            Ok(model) => (model, None),
            Err(e) => {
                eprintln!("Could not initialize Glamour model: {}", e);
                std::process::exit(1);
            }
        }
    }

    fn update(&mut self, msg: Msg) -> Option<bubbletea_rs::Cmd> {
        // Handle keyboard input - check for quit keys first
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if self.keys.quit.matches(key_msg) {
                return Some(quit());
            }
        }

        // Pass all other messages to the viewport for scrolling
        self.viewport.update(msg)
    }

    fn view(&self) -> String {
        // Apply styling to the viewport and combine with help text
        let styled_viewport = self.viewport_style.render(&self.viewport.view());
        format!("{}{}", styled_viewport, self.help_view())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the program
    let program = Program::<GlamourModel>::builder().build()?;

    program.run().await?;

    Ok(())
}
