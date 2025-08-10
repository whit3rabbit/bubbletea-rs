// List Default Example - Matches the Go Bubble Tea list-default example
//
// Now uses bubbletea-widgets v0.1.6 with the enhanced List API for proper
// filter state management. No workarounds needed!

use bubbletea_rs::{window_size, Cmd, KeyMsg, Model as BubbleTeaModel, Msg, Program, WindowSizeMsg};
use bubbletea_widgets::list::{DefaultItem, DefaultDelegate, Model as List};
use crossterm::event::{KeyCode, KeyModifiers};
use lipgloss_extras::lipgloss::{Style, Color};

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
            DefaultItem::new("The vernal equinox", "The autumnal equinox is pretty good too"),
            DefaultItem::new("Gaffer's tape", "Basically sticky fabric"),
            DefaultItem::new("Terrycloth", "In other words, towel fabric"),
        ];

        // Create list with default delegate and try to fix filter highlighting issues
        let mut delegate = DefaultDelegate::new();
        
        // Test the v0.1.2 fix with visible highlighting
        // If the fix worked, we should see proper highlighting without character separation
        delegate.styles.filter_match = Style::new()
            .bold(true)
            .foreground(Color::from("#FBBF24")); // Yellow highlighting to make it visible
        
        let list = List::new(items, delegate, 80, 24)
            .with_title("My Fave Things");

        (Model { list }, Some(init_render_cmd()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle initial render message
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            // Request window size to get proper initial dimensions
            return Some(window_size());
        }

        // Handle key messages using the enhanced v0.1.6 API
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                // Always quit on Ctrl+C regardless of filter state
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(bubbletea_rs::quit());
                }
                
                // Handle Escape key - clear filter or quit using new API
                KeyCode::Esc => {
                    if self.list.is_filtering() {
                        // Use the new clear_filter() method for clean single-escape behavior
                        return self.list.clear_filter();
                    } else {
                        // Not filtering: quit
                        return Some(bubbletea_rs::quit());
                    }
                }
                
                // Handle 'q' key - quit only when not filtering using new API
                KeyCode::Char('q') => {
                    if !self.list.is_filtering() {
                        return Some(bubbletea_rs::quit());
                    }
                    // Let list handle 'q' during filtering
                }
                
                // All other keys: delegate to list widget
                _ => {}
            }
        }

        // Handle window size exactly like Go version
        if let Some(_size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            // Go version: m.list.SetSize(msg.Width-h, msg.Height-v)
            // bubbletea-widgets handles this internally, so we just delegate
        }

        // Always delegate to list widget (like Go version)
        self.list.update(msg)
    }

    fn view(&self) -> String {
        // Render list view with document style (matching Go version)
        let view = self.list.view();
        
        // Additional debug info can be added here if needed
        // For example, we could show our filter state vs widget filter state
        
        doc_style().render(&view)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create program with alt screen (matching Go version)
    let program = Program::<Model>::builder()
        .alt_screen(true)
        .build()?;

    // Run the program
    let _result = program.run().await?;

    Ok(())
}