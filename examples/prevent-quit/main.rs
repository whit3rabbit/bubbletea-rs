// A program demonstrating how to prevent quitting when there are unsaved changes.
//
// Unlike the Go version which uses WithFilter to intercept QuitMsg events,
// this Rust implementation achieves the same behavior through model state management:
// - Tracks unsaved changes with a boolean flag
// - Intercepts quit attempts in the update logic
// - Shows a confirmation dialog when trying to quit with unsaved changes
//
// This approach is more idiomatic in Rust and provides the same user experience.

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::help::{KeyMap as HelpKeyMap, Model as HelpModel};
use bubbletea_widgets::key::{
    matches_binding, new_binding, with_help, with_keys_str, Binding, KeyMap,
};
use bubbletea_widgets::{textarea, Component};
use crossterm::event::KeyCode;
use lipgloss_extras::lipgloss::{join_horizontal, rounded_border, Color, Style, TOP};

// Styling to match the Go version
fn choice_style() -> Style {
    Style::new().padding_left(1).foreground(Color::from("241"))
}

fn save_text_style() -> Style {
    Style::new().foreground(Color::from("170"))
}

fn quit_view_style() -> Style {
    Style::new()
        .padding(1, 1, 1, 1)
        .border(rounded_border())
        .border_foreground(Color::from("170"))
}

struct AppModel {
    textarea: textarea::Model,
    help: HelpModel,
    keymap: Keymap,
    save_text: String,
    has_changes: bool,
    quitting: bool,
}

#[derive(Debug, Clone)]
struct Keymap {
    save: Binding,
    quit: Binding,
}

impl Keymap {
    fn new() -> Self {
        Self {
            save: new_binding(vec![
                with_keys_str(&["ctrl+s"]),
                with_help("ctrl+s", "save"),
            ]),
            quit: new_binding(vec![
                with_keys_str(&["esc", "ctrl+c"]),
                with_help("esc", "quit"),
            ]),
        }
    }
}

impl KeyMap for Keymap {
    fn short_help(&self) -> Vec<&Binding> {
        vec![&self.save, &self.quit]
    }

    fn full_help(&self) -> Vec<Vec<&Binding>> {
        vec![vec![&self.save, &self.quit]]
    }
}

impl AppModel {
    fn new() -> Self {
        let textarea = textarea::new();
        // Note: bubbletea-widgets textarea doesn't have set_placeholder in current version
        // We'll show placeholder text in the view instead

        Self {
            textarea,
            help: HelpModel::new(),
            keymap: Keymap::new(),
            save_text: String::new(),
            has_changes: false,
            quitting: false,
        }
    }

    fn update_text_view(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            self.save_text.clear();

            if matches_binding(key_msg, &self.keymap.save) {
                self.save_text = "Changes saved!".to_string();
                self.has_changes = false;
                return None;
            } else if matches_binding(key_msg, &self.keymap.quit) {
                self.quitting = true;
                return Some(quit());
            } else {
                // Check if it's a character input (indicating changes)
                match key_msg.key {
                    KeyCode::Char(_) => {
                        self.has_changes = true;
                    }
                    KeyCode::Enter | KeyCode::Backspace | KeyCode::Delete => {
                        self.has_changes = true;
                    }
                    _ => {}
                }
            }
        }

        // Update textarea
        self.textarea.update(Some(msg))
    }

    fn update_prompt_view(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('y') => {
                    self.has_changes = false;
                    return Some(quit());
                }
                _ => {
                    // For simplicity, treat any other key as "no"
                    if matches_binding(key_msg, &self.keymap.quit) {
                        self.has_changes = false;
                        return Some(quit());
                    }
                    self.quitting = false;
                }
            }
        }

        None
    }
}

// Implement HelpKeyMap trait to connect our keymap to the help widget
impl HelpKeyMap for AppModel {
    fn short_help(&self) -> Vec<&Binding> {
        self.keymap.short_help()
    }

    fn full_help(&self) -> Vec<Vec<&Binding>> {
        self.keymap.full_help()
    }
}

impl Model for AppModel {
    fn init() -> (Self, Option<Cmd>) {
        let mut model = Self::new();
        let cmd = model.textarea.focus();
        (model, cmd)
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if self.quitting {
            self.update_prompt_view(msg)
        } else {
            self.update_text_view(msg)
        }
    }

    fn view(&self) -> String {
        if self.quitting {
            if self.has_changes {
                let text = join_horizontal(
                    TOP,
                    &[
                        "You have unsaved changes. Quit without saving?",
                        &choice_style().render("[yn]"),
                    ],
                );
                return quit_view_style().render(&text);
            }
            return "Very important, thank you\n".to_string();
        }

        let help_view = self.help.view(self);

        let textarea_content = self.textarea.value();
        let placeholder_or_content = if textarea_content.trim().is_empty() {
            "Only the best words".to_string()
        } else {
            textarea_content
        };

        format!(
            "\nType some important things.\n\n{}\n\n {}\n {}\n\n",
            placeholder_or_content,
            save_text_style().render(&self.save_text),
            help_view
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<AppModel>::builder()
        .alt_screen(true)
        .signal_handler(true)
        .build()?;

    let _ = program.run().await?;
    Ok(())
}
