// A simple example that shows how to retrieve a value from a Bubble Tea
// program after the Bubble Tea has exited.

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::key::{matches_binding, new_binding, with_help, with_keys_str, Binding};

const CHOICES: &[&str] = &["Taro", "Coffee", "Lychee"];

// Synthetic message used to trigger the initial render immediately after startup.
#[derive(Debug, Clone)]
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

struct AppModel {
    cursor: usize,
    choice: String,
    keymap: KeyMap,
}

struct KeyMap {
    quit: Binding,
    enter: Binding,
    up: Binding,
    down: Binding,
}

impl KeyMap {
    fn new() -> Self {
        Self {
            quit: new_binding(vec![
                with_keys_str(&["ctrl+c", "q", "esc"]),
                with_help("q", "quit"),
            ]),
            enter: new_binding(vec![
                with_keys_str(&["enter"]),
                with_help("enter", "select"),
            ]),
            up: new_binding(vec![with_keys_str(&["up", "k"]), with_help("↑/k", "up")]),
            down: new_binding(vec![
                with_keys_str(&["down", "j"]),
                with_help("↓/j", "down"),
            ]),
        }
    }
}

impl Model for AppModel {
    fn init() -> (Self, Option<Cmd>) {
        (
            AppModel {
                cursor: 0,
                choice: String::new(),
                keymap: KeyMap::new(),
            },
            Some(init_render_cmd()),
        )
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle the initial render trigger message
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            // No-op: receiving this message merely triggers the initial render
            return None;
        }

        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if matches_binding(key_msg, &self.keymap.quit) {
                return Some(quit());
            } else if matches_binding(key_msg, &self.keymap.enter) {
                // Send the choice on the channel and exit.
                self.choice = CHOICES[self.cursor].to_string();
                return Some(quit());
            } else if matches_binding(key_msg, &self.keymap.down) {
                self.cursor += 1;
                if self.cursor >= CHOICES.len() {
                    self.cursor = 0;
                }
            } else if matches_binding(key_msg, &self.keymap.up) {
                if self.cursor == 0 {
                    self.cursor = CHOICES.len() - 1;
                } else {
                    self.cursor -= 1;
                }
            }
        }

        None
    }

    fn view(&self) -> String {
        let mut s = String::new();
        s.push_str("What kind of Bubble Tea would you like to order?\n\n");

        for (i, choice) in CHOICES.iter().enumerate() {
            if self.cursor == i {
                s.push_str("(•) ");
            } else {
                s.push_str("( ) ");
            }
            s.push_str(choice);
            s.push('\n');
        }
        s.push_str("\n(press q to quit)\n");

        s
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<AppModel>::builder()
        .alt_screen(true)
        .signal_handler(true)
        .build()?;

    // Run returns the model as a final result.
    let final_model = program.run().await?;

    // Print the choice if one was made.
    if !final_model.choice.is_empty() {
        println!("\n---\nYou chose {}!", final_model.choice);
    }

    Ok(())
}
