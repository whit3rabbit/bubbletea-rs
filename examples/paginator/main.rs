// Paginator Example - Matches the Go Bubble Tea paginator example
//
// A simple program demonstrating the paginator component from the bubbletea-widgets library.

use bubbletea_rs::{
    window_size, Cmd, KeyMsg, Model as BubbleTeaModel, Msg, Program, WindowSizeMsg,
};
use bubbletea_widgets::paginator::{Model as Paginator, Type};
use crossterm::event::{KeyCode, KeyModifiers};
use lipgloss_extras::lipgloss::{renderer, Color, ColorProfileKind, Style};

// Synthetic message used to trigger the initial render immediately after startup.
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

fn new_model() -> Model {
    // Create 100 items matching the Go example
    let mut items = Vec::new();
    for i in 1..=100 {
        items.push(format!("Item {}", i));
    }

    // Create paginator with dots style matching Go example
    let mut paginator = Paginator::new();
    paginator.paginator_type = Type::Dots;
    paginator.set_per_page(10);
    paginator.set_total_items(items.len());

    // Set custom dots matching Go's lipgloss styling
    // Go: ActiveDot = lipgloss.NewStyle().Foreground(lipgloss.AdaptiveColor{Light: "235", Dark: "252"}).Render("•")
    // Go: InactiveDot = lipgloss.NewStyle().Foreground(lipgloss.AdaptiveColor{Light: "250", Dark: "238"}).Render("•")
    let active_style = Style::new().foreground(Color::from("252")); // Use dark mode color
    let inactive_style = Style::new().foreground(Color::from("238")); // Use dark mode color

    paginator.active_dot = active_style.render("•");
    paginator.inactive_dot = inactive_style.render("•");

    Model {
        paginator,
        items,
        width: 80,  // Default width
        height: 24, // Default height
    }
}

pub struct Model {
    items: Vec<String>,
    paginator: Paginator,
    width: u16,
    height: u16,
}

impl BubbleTeaModel for Model {
    fn init() -> (Self, Option<Cmd>) {
        let model = new_model();
        (model, Some(init_render_cmd()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle initial render message
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            return Some(window_size());
        }

        // Handle window size changes
        if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            self.width = size_msg.width;
            self.height = size_msg.height;
            return None;
        }

        // Handle key messages
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('q') | KeyCode::Esc => {
                    return Some(bubbletea_rs::quit());
                }
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(bubbletea_rs::quit());
                }
                // Handle navigation keys manually
                KeyCode::Left | KeyCode::Char('h') => {
                    if self.paginator.page > 0 {
                        self.paginator.page -= 1;
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    let total_pages = self.items.len().div_ceil(self.paginator.per_page);
                    if self.paginator.page < total_pages - 1 {
                        self.paginator.page += 1;
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn view(&self) -> String {
        let mut output = String::new();

        // Only show title if we have enough vertical space
        if self.height > 10 {
            output.push_str("\n  Paginator Example\n\n");
        }

        // Get slice bounds for current page
        let (start, end) = self.paginator.get_slice_bounds(self.items.len());

        // Calculate available space for items (leave room for paginator and help)
        let available_height = if self.height > 10 {
            self.height.saturating_sub(6) // Title + paginator + help + margins
        } else {
            self.height.saturating_sub(4) // Just paginator + help + margins
        };

        // Display items for current page, but limit to available space
        let items_to_show = &self.items[start..end];
        let max_items = std::cmp::min(items_to_show.len(), available_height as usize / 2);

        for item in &items_to_show[..max_items] {
            output.push_str(&format!("  • {}\n", item));
        }

        // Add some spacing before paginator if we have room
        if self.height > 8 {
            output.push('\n');
        }

        // Display paginator
        output.push_str(&format!("  {}", self.paginator.view()));

        // Show help if we have room
        if self.height > 6 {
            output.push_str("\n\n  h/l ←/→ page • q: quit");
        }

        output
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Force TrueColor mode FIRST to ensure proper color rendering
    // Must be set before any lipgloss styling operations
    renderer::set_color_profile(ColorProfileKind::TrueColor);

    // Create program (no alt screen to match Go version behavior)
    let program = Program::<Model>::builder()
        .alt_screen(false)
        .signal_handler(true)
        .build()?;

    // Run the program
    let _result = program.run().await?;

    Ok(())
}
