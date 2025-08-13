use bubbletea_rs::{quit, Cmd, KeyMsg, Model, MouseMotion, MouseMsg, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers, MouseEventKind};

// A simple program that opens the alternate screen buffer and displays mouse
// coordinates and events.

#[derive(Debug)]
pub struct MouseModel {
    mouse_events: Vec<String>,
}

impl Model for MouseModel {
    fn init() -> (Self, Option<Cmd>) {
        (
            MouseModel {
                mouse_events: Vec::new(),
            },
            None,
        )
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key) = msg.downcast_ref::<KeyMsg>() {
            match (key.key, key.modifiers) {
                (KeyCode::Char('q'), _) | (KeyCode::Esc, _) => return Some(quit()),
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Some(quit()),
                _ => {}
            }
        }

        if let Some(mouse) = msg.downcast_ref::<MouseMsg>() {
            let event_str = format_mouse_event(mouse);
            let event_line = format!("(X: {}, Y: {}) {}", mouse.x, mouse.y, event_str);

            self.mouse_events.push(event_line);

            // Keep only the last 10 events to prevent infinite scrolling
            if self.mouse_events.len() > 10 {
                self.mouse_events.remove(0);
            }
        }

        None
    }

    fn view(&self) -> String {
        let mut output = "Do mouse stuff. When you're done press q to quit.\n\n".to_string();

        if self.mouse_events.is_empty() {
            output.push_str("Move your mouse to see events...\n");
        } else {
            output.push_str("Recent mouse events:\n");
            for event in &self.mouse_events {
                output.push_str(&format!("{}\n", event));
            }
        }

        output
    }
}

fn format_mouse_event(mouse: &MouseMsg) -> String {
    let mut modifiers = Vec::new();
    if mouse.modifiers.contains(KeyModifiers::CONTROL) {
        modifiers.push("ctrl");
    }
    if mouse.modifiers.contains(KeyModifiers::ALT) {
        modifiers.push("alt");
    }
    if mouse.modifiers.contains(KeyModifiers::SHIFT) {
        modifiers.push("shift");
    }

    let modifier_str = if modifiers.is_empty() {
        String::new()
    } else {
        format!("{}+", modifiers.join("+"))
    };

    let event_type = match mouse.button {
        MouseEventKind::Down(button) => format!("{:?} press", button).to_lowercase(),
        MouseEventKind::Up(button) => format!("{:?} release", button).to_lowercase(),
        MouseEventKind::Drag(button) => format!("{:?} drag", button).to_lowercase(),
        MouseEventKind::Moved => "motion".to_string(),
        MouseEventKind::ScrollDown => "wheel down".to_string(),
        MouseEventKind::ScrollUp => "wheel up".to_string(),
        MouseEventKind::ScrollLeft => "wheel left".to_string(),
        MouseEventKind::ScrollRight => "wheel right".to_string(),
    };

    format!("{}{}", modifier_str, event_type)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<MouseModel>::builder()
        .alt_screen(true)
        .mouse_motion(MouseMotion::All)
        .build()?;

    program.run().await?;
    Ok(())
}
