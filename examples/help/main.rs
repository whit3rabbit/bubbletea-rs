//! Help Example
//!
//! Demonstrates:
//! - Key binding system with help text using bubbletea-widgets
//! - Toggle between short and full help modes
//! - Arrow key navigation with visual feedback
//! - Responsive help text formatting
//! - Styled output with colors
//!
//! This example shows how to implement a help system that displays
//! key bindings and can toggle between mini and full help views.
//! Uses bubbletea-widgets help and key modules for proper integration.

use bubbletea_rs::{quit, Cmd, KeyMsg, Model as BubbleTeaModel, Msg, Program, WindowSizeMsg};
use bubbletea_widgets::help::{KeyMap as HelpKeyMap, Model as HelpModel};
use bubbletea_widgets::key::{
    matches_binding, new_binding, with_help, with_keys_str, Binding, KeyMap,
};
use crossterm::terminal;
use lipgloss_extras::lipgloss::{Color, Style};

// Synthetic message used to trigger the initial render immediately after startup.
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

/// KeyMap defines a set of keybindings. To work for help it must satisfy
/// key::KeyMap. It mirrors the Go example structure.
#[derive(Debug, Clone)]
pub struct Keymap {
    up: Binding,
    down: Binding,
    left: Binding,
    right: Binding,
    help: Binding,
    quit: Binding,
}

impl Keymap {
    fn new() -> Self {
        Self {
            up: new_binding(vec![
                with_keys_str(&["up", "k"]),
                with_help("↑/k", "move up"),
            ]),
            down: new_binding(vec![
                with_keys_str(&["down", "j"]),
                with_help("↓/j", "move down"),
            ]),
            left: new_binding(vec![
                with_keys_str(&["left", "h"]),
                with_help("←/h", "move left"),
            ]),
            right: new_binding(vec![
                with_keys_str(&["right", "l"]),
                with_help("→/l", "move right"),
            ]),
            help: new_binding(vec![with_keys_str(&["?"]), with_help("?", "toggle help")]),
            quit: new_binding(vec![
                with_keys_str(&["q", "esc", "ctrl+c"]),
                with_help("q", "quit"),
            ]),
        }
    }
}

/// ShortHelp returns keybindings to be shown in the mini help view. It's part
/// of the key::KeyMap interface.
impl KeyMap for Keymap {
    fn short_help(&self) -> Vec<&Binding> {
        vec![&self.help, &self.quit]
    }

    fn full_help(&self) -> Vec<Vec<&Binding>> {
        vec![
            vec![&self.up, &self.down, &self.left, &self.right], // first column
            vec![&self.help, &self.quit],                        // second column
        ]
    }
}

/// The help model containing key bindings and application state
pub struct Model {
    pub keys: Keymap,
    pub help: HelpModel,
    pub input_style: Style,
    pub last_key: String,
    pub quitting: bool,
}

impl Model {
    pub fn new() -> Self {
        Self {
            keys: Keymap::new(),
            help: HelpModel::new(),
            input_style: Style::new().foreground(Color::from("#FF75B7")),
            last_key: String::new(),
            quitting: false,
        }
    }
}

// Implement HelpKeyMap trait to connect our keymap to the help widget
impl HelpKeyMap for Model {
    fn short_help(&self) -> Vec<&Binding> {
        self.keys.short_help()
    }

    fn full_help(&self) -> Vec<Vec<&Binding>> {
        self.keys.full_help()
    }
}

impl BubbleTeaModel for Model {
    fn init() -> (Self, Option<Cmd>) {
        let mut model = Self::new();

        // Set initial terminal width if available
        if let Ok((w, _h)) = terminal::size() {
            model.help.width = w as usize;
        }

        // Emit a synthetic message immediately to trigger the first render
        (model, Some(init_render_cmd()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if matches_binding(key_msg, &self.keys.up) {
                self.last_key = "↑".to_string();
            } else if matches_binding(key_msg, &self.keys.down) {
                self.last_key = "↓".to_string();
            } else if matches_binding(key_msg, &self.keys.left) {
                self.last_key = "←".to_string();
            } else if matches_binding(key_msg, &self.keys.right) {
                self.last_key = "→".to_string();
            } else if matches_binding(key_msg, &self.keys.help) {
                self.help.show_all = !self.help.show_all;
            } else if matches_binding(key_msg, &self.keys.quit) {
                self.quitting = true;
                return Some(quit());
            }
        } else if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            // If we set a width on the help menu it can gracefully truncate
            // its view as needed.
            self.help.width = size_msg.width as usize;
        } else if msg.downcast_ref::<InitRenderMsg>().is_some() {
            // No-op: receiving this message merely triggers the initial render
        }

        None
    }

    fn view(&self) -> String {
        if self.quitting {
            return "Bye!\n".to_string();
        }

        let status = if self.last_key.is_empty() {
            "Waiting for input...".to_string()
        } else {
            format!("You chose: {}", self.input_style.render(&self.last_key))
        };

        let help_view = self.help.view(self);

        // Match upstream Go example: place help near the bottom of a fixed-height
        // region by inserting blank lines between the status and help
        let status_lines = status.matches('\n').count();
        let help_lines = help_view.matches('\n').count();
        let total_block_rows: isize = 8;
        let used_rows: isize = (status_lines as isize) + (help_lines as isize);
        let pad_rows = (total_block_rows - used_rows).max(0) as usize;

        format!("\n{}{}{}", status, "\n".repeat(pad_rows), help_view)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<Model>::builder()
        .signal_handler(true)
        .alt_screen(true)
        .build()?;

    program.run().await?;
    Ok(())
}
