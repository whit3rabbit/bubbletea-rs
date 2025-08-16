// Paginator Example - Matches the Go Bubble Tea paginator example
//
// A simple program demonstrating the paginator component from the bubbletea-widgets library.

use bubbletea_rs::{Cmd, KeyMsg, Model as BubbleTeaModel, Msg, Program};
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

    Model { paginator, items }
}

pub struct Model {
    items: Vec<String>,
    paginator: Paginator,
}

impl BubbleTeaModel for Model {
    fn init() -> (Self, Option<Cmd>) {
        let model = new_model();
        (model, Some(init_render_cmd()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle initial render message
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
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
                _ => {}
            }
        }

        // Update paginator - this handles h/l and arrow key navigation
        self.paginator.update(&msg);
        None
    }

    fn view(&self) -> String {
        let mut output = String::new();
        output.push_str("\n  Paginator Example\n\n");

        // Get slice bounds for current page
        let (start, end) = self.paginator.get_slice_bounds(self.items.len());

        // Display items for current page
        for item in &self.items[start..end] {
            output.push_str(&format!("  • {}\n\n", item));
        }

        // Display paginator
        output.push_str(&format!("  {}", self.paginator.view()));
        output.push_str("\n\n  h/l ←/→ page • q: quit\n");

        output
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Force TrueColor mode to ensure proper color rendering
    renderer::set_color_profile(ColorProfileKind::TrueColor);

    // Create program (no alt screen to match Go version behavior)
    let program = Program::<Model>::builder().build()?;

    // Run the program
    program.run().await?;

    Ok(())
}
