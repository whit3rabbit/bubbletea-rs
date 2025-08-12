// List Default Example - Matches the Go Bubble Tea list-default example
//
// This is a minimal implementation that matches the simplicity of the Go version.
// The rich help text is provided automatically by bubbletea-widgets.

use bubbletea_rs::{
    window_size, Cmd, KeyMsg, Model as BubbleTeaModel, Msg, Program, WindowSizeMsg,
};
use bubbletea_widgets::list::{DefaultDelegate, DefaultItem, Model as List};
use bubbletea_widgets::paginator::Type as PaginatorType;
use crossterm::event::{KeyCode, KeyModifiers};
use lipgloss_extras::lipgloss::renderer::{self, ColorProfileKind};
use lipgloss_extras::lipgloss::Style;

// Synthetic message used to trigger the initial render immediately after startup.
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

// Document style with margin(1, 2) matching Go's docStyle.Margin(1, 2)
// Go's Margin(1, 2) = 1 vertical, 2 horizontal = margin(top=1, right=2, bottom=1, left=2)
fn doc_style() -> Style {
    Style::new().margin(1, 2, 1, 2)
}

// Main model struct containing the list
pub struct Model {
    pub list: List<DefaultItem>,
}

impl BubbleTeaModel for Model {
    fn init() -> (Self, Option<Cmd>) {
        // Create items matching the Go example exactly
        let items = vec![
            DefaultItem::new("Raspberry Pi's", "I have 'em all over my house"),
            DefaultItem::new("Nutella", "It's good on toast"),
            DefaultItem::new("Bitter melon", "It cools you down"),
            DefaultItem::new("Nice socks", "And by that I mean socks without holes"),
            DefaultItem::new("Eight hours of sleep", "I had this once"),
            DefaultItem::new("Cats", "Usually"),
            DefaultItem::new("Plantasia, the album", "My plants love it too"),
            DefaultItem::new("Pour over coffee", "It takes forever to make though"),
            DefaultItem::new("VR", "Virtual reality...what is there to say?"),
            DefaultItem::new("Noguchi Lamps", "Such pleasing organic forms"),
            DefaultItem::new("Linux", "Pretty much the best OS"),
            DefaultItem::new("Business school", "Just kidding"),
            DefaultItem::new("Pottery", "Wet clay is a great feeling"),
            DefaultItem::new("Shampoo", "Nothing like clean hair"),
            DefaultItem::new("Table tennis", "It's surprisingly exhausting"),
            DefaultItem::new("Milk crates", "Great for packing in your extra stuff"),
            DefaultItem::new("Afternoon tea", "Especially the tea sandwich part"),
            DefaultItem::new("Stickers", "The thicker the vinyl the better"),
            DefaultItem::new("20° Weather", "Celsius, not Fahrenheit"),
            DefaultItem::new("Warm light", "Like around 2700 Kelvin"),
            DefaultItem::new(
                "The vernal equinox",
                "The autumnal equinox is pretty good too",
            ),
            DefaultItem::new("Gaffer's tape", "Basically sticky fabric"),
            DefaultItem::new("Terrycloth", "In other words, towel fabric"),
        ];

        // Create list with default delegate - simple like Go version
        let delegate = DefaultDelegate::new();

        // Get initial terminal size (like Go version)
        let (terminal_width, terminal_height) = crossterm::terminal::size().unwrap_or((80, 24));
        let frame_width = 4; // 2 left + 2 right margin from doc_style
        let frame_height = 2; // 1 top + 1 bottom margin from doc_style

        let list_width = (terminal_width as usize).saturating_sub(frame_width);
        let list_height = (terminal_height as usize).saturating_sub(frame_height);

        let list = List::new(items, delegate, list_width, list_height)
            .with_title("My Fave Things")
            .with_pagination_type(PaginatorType::Dots); // Use dots pagination to match Go version

        (Model { list }, Some(init_render_cmd()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle initial render message
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            return Some(window_size());
        }

        // Handle Ctrl+C like the Go version (only custom key handling)
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if key_msg.key == KeyCode::Char('c')
                && key_msg.modifiers.contains(KeyModifiers::CONTROL)
            {
                return Some(bubbletea_rs::quit());
            }
        }

        // Handle window size like Go version
        if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            // Go version: h, v := docStyle.GetFrameSize(); m.list.SetSize(msg.Width-h, msg.Height-v)
            // Calculate frame size from doc_style (margin 1,2 = 2 vertical, 4 horizontal)
            let frame_width = 4; // 2 left + 2 right margin
            let frame_height = 2; // 1 top + 1 bottom margin

            let new_width = (size_msg.width as usize).saturating_sub(frame_width);
            let new_height = (size_msg.height as usize).saturating_sub(frame_height);

            // Update list size to match terminal dimensions
            self.list.set_size(new_width, new_height);
        }

        // Let list handle all other messages (like Go version)
        self.list.update(msg)
    }

    fn view(&self) -> String {
        // Render list view with document style (matching Go version)
        let view = self.list.view();

        doc_style().render(&view)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Force TrueColor mode FIRST to ensure proper color rendering
    // Must be set before any lipgloss styling operations
    renderer::set_color_profile(ColorProfileKind::TrueColor);

    // Create program with alt screen (matching Go version)
    let program = Program::<Model>::builder().alt_screen(true).build()?;

    // Run the program
    let _result = program.run().await?;

    Ok(())
}
