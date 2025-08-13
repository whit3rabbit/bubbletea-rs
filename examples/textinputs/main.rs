//! Text Inputs Example (Multiple Fields) using bubbletea-widgets
//!
//! Mirrors the Go Bubbles example with three inputs and a submit button.

use bubbletea_rs::command::batch;
use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::{cursor, key, textinput};
use lipgloss_extras::lipgloss::{Color, Style};

struct ModelTextInputs {
    focus_index: isize,
    inputs: Vec<textinput::Model>,
    cursor_mode: cursor::Mode,
    submit_focused: bool,
    keymap: AppKeyMap,
}

impl ModelTextInputs {
    fn new() -> Self {
        let focused_style = Style::new().foreground(Color::from("205"));

        let mut inputs = vec![textinput::new(), textinput::new(), textinput::new()];

        for (i, ti) in inputs.iter_mut().enumerate() {
            // Configure cursor properly:
            // cursor.style is for the cursor block when visible (gets reverse(true) applied)
            // cursor.text_style is for the character underneath when cursor is hidden
            ti.cursor.style = Style::new(); // Default cursor block style
            ti.cursor.text_style = Style::new(); // Default text style when cursor hidden
                                                 // Make sure cursor mode is set to blink
            let _ = ti.cursor.set_mode(cursor::Mode::Blink);
            ti.set_char_limit(32);
            // Explicitly set text_style to default to avoid conflicts
            ti.text_style = Style::new();
            match i {
                0 => {
                    ti.set_placeholder("Nickname");
                    let _ = ti.focus();
                    ti.prompt_style = focused_style.clone();
                }
                1 => {
                    ti.set_placeholder("Email");
                    ti.set_char_limit(64);
                }
                2 => {
                    ti.set_placeholder("Password");
                    // Password masking configuration omitted for compatibility across versions
                }
                _ => {}
            }
        }

        Self {
            focus_index: 0,
            inputs,
            cursor_mode: cursor::Mode::Blink,
            submit_focused: false,
            keymap: AppKeyMap::default(),
        }
    }
}

struct AppKeyMap {
    quit: key::Binding,
    toggle_cursor: key::Binding,
    next: key::Binding,
    prev: key::Binding,
    enter: key::Binding,
}

impl Default for AppKeyMap {
    fn default() -> Self {
        Self {
            quit: key::new_binding(vec![key::with_keys_str(&["esc", "ctrl+c"])]),
            toggle_cursor: key::new_binding(vec![key::with_keys_str(&["ctrl+r"])]),
            next: key::new_binding(vec![key::with_keys_str(&["tab", "down"])]),
            prev: key::new_binding(vec![key::with_keys_str(&["shift+tab", "up"])]),
            enter: key::new_binding(vec![key::with_keys_str(&["enter"])]),
        }
    }
}

impl Model for ModelTextInputs {
    fn init() -> (Self, Option<Cmd>) {
        let mut m = Self::new();
        // Focus on first input
        let cmd_focus = m.inputs[0].focus();
        (m, Some(cmd_focus))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            // Quit
            if self.keymap.quit.matches(key_msg) {
                return Some(quit());
            }

            // Change cursor mode
            if self.keymap.toggle_cursor.matches(key_msg) {
                self.cursor_mode = match self.cursor_mode {
                    cursor::Mode::Blink => cursor::Mode::Static,
                    cursor::Mode::Static => cursor::Mode::Hide,
                    cursor::Mode::Hide => cursor::Mode::Blink,
                };
                let cmds: Vec<Cmd> = self
                    .inputs
                    .iter_mut()
                    .filter_map(|i| i.cursor.set_mode(self.cursor_mode))
                    .collect();
                return Some(batch(cmds));
            }

            // Navigation and submit
            let is_back = self.keymap.prev.matches(key_msg);
            let is_forward =
                self.keymap.next.matches(key_msg) || self.keymap.enter.matches(key_msg);

            if is_back || is_forward {
                // Enter while submit focused quits
                if self.keymap.enter.matches(key_msg) && self.submit_focused {
                    return Some(quit());
                }

                // Compute next focus index
                if is_back {
                    if self.submit_focused {
                        self.submit_focused = false;
                        self.focus_index = (self.inputs.len() as isize) - 1;
                    } else {
                        self.focus_index -= 1;
                        if self.focus_index < 0 {
                            self.submit_focused = true;
                            self.focus_index = self.inputs.len() as isize;
                        }
                    }
                } else if is_forward {
                    if self.submit_focused {
                        self.submit_focused = false;
                        self.focus_index = 0;
                    } else {
                        self.focus_index += 1;
                        if self.focus_index >= self.inputs.len() as isize {
                            self.submit_focused = true;
                            self.focus_index = self.inputs.len() as isize;
                        }
                    }
                }

                // Apply focus/blur
                let mut cmds: Vec<Cmd> = Vec::new();
                if (self.focus_index as usize) == self.inputs.len() {
                    // Submit focused
                    self.submit_focused = true;
                    for i in self.inputs.iter_mut() {
                        i.blur();
                        i.prompt_style = Style::new();
                        i.text_style = Style::new();
                    }
                } else {
                    self.submit_focused = false;
                    let focused_style = Style::new().foreground(Color::from("205"));
                    let len = self.inputs.len();
                    for idx in 0..len {
                        let i = &mut self.inputs[idx];
                        if idx as isize == self.focus_index {
                            let c = i.focus();
                            cmds.push(c);
                            i.prompt_style = focused_style.clone();
                            i.text_style = Style::new(); // Keep text default white
                        } else {
                            i.blur();
                            i.prompt_style = Style::new();
                            i.text_style = Style::new();
                        }
                    }
                }
                return if cmds.is_empty() {
                    None
                } else {
                    Some(batch(cmds))
                };
            }
        }

        // Delegate other messages only to the focused input
        if !self.submit_focused {
            let idx = self.focus_index.max(0) as usize;
            return self.inputs[idx].update(msg);
        }
        None
    }

    fn view(&self) -> String {
        let focused_style = Style::new().foreground(Color::from("205"));
        let blurred_style = Style::new().foreground(Color::from("240"));
        let help_style = blurred_style.clone();
        let cursor_mode_help_style = Style::new().foreground(Color::from("244"));

        let mut out = String::new();
        for (_i, input) in self.inputs.iter().enumerate() {
            let line = input.view();
            out.push_str(&line);
            if _i < self.inputs.len() - 1 {
                out.push('\n');
            }
        }

        // Submit button
        let focused_button = focused_style.render("[ Submit ]");
        let blurred_button = format!("[ {} ]", blurred_style.render("Submit"));
        let button = if self.submit_focused {
            &focused_button
        } else {
            &blurred_button
        };
        out.push_str(&format!("\n\n{}\n\n", button));

        // Help
        out.push_str(&help_style.render("cursor mode is "));
        let mode_label = match self.cursor_mode {
            cursor::Mode::Blink => "blink",
            cursor::Mode::Static => "static",
            cursor::Mode::Hide => "hide",
        };
        out.push_str(&cursor_mode_help_style.render(mode_label));
        out.push_str(&help_style.render(" (ctrl+r to change style)"));

        out
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<ModelTextInputs>::builder()
        .alt_screen(true)
        .signal_handler(true)
        .build()?;
    let _ = program.run().await?;
    Ok(())
}
