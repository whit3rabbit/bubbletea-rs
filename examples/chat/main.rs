use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::{textinput, viewport};
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};
use lipgloss_extras::lipgloss::{Color, Style};

// Port of Bubble Tea's chat example using bubbletea-widgets `viewport` and `textarea`.

const GAP: &str = "\n\n"; // matches the Go example (2 blank lines)
const WELCOME_TEXT: &str = "Welcome to the chat room!\nType a message and press Enter to send.";

// Key mappings for the chat example
#[derive(Debug)]
pub struct KeyBindings {
    pub quit: Binding,
    pub quit_alt: Binding,
    pub send: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![
                with_keys_str(&["esc"]),
                with_help("esc", "quit"),
            ]),
            quit_alt: new_binding(vec![
                with_keys_str(&["ctrl+c"]),
                with_help("ctrl+c", "quit"),
            ]),
            send: new_binding(vec![
                with_keys_str(&["enter"]),
                with_help("enter", "send message"),
            ]),
        }
    }
}

struct ChatModel {
    viewport: viewport::Model,
    textinput: textinput::Model,
    messages: Vec<String>,
    sender_style: Style,
    keys: KeyBindings,
    // Track terminal size for wrapping
    term_width: usize,
    term_height: usize,
    input_height: usize,
}

impl Model for ChatModel {
    fn init() -> (Self, Option<Cmd>) {
        let mut ti = textinput::new();
        ti.set_placeholder("Send a message...");
        ti.prompt = "┃ ".to_string();
        ti.set_width(30);
        ti.set_char_limit(280);
        // Focus so typing works immediately
        let _ = ti.focus();

        // Use reasonable defaults for initial terminal size - WindowSizeMsg will update these
        let (initial_width, initial_height) = (80usize, 24usize);

        ti.set_width((initial_width.saturating_sub(2)) as i32); // Account for prompt

        // Compute initial viewport height and initialize viewport content
        let gap_h = GAP.matches('\n').count();
        let input_height = 1; // textinput is single-line
        let vp_height = initial_height
            .saturating_sub(input_height)
            .saturating_sub(gap_h)
            .max(1);
        let mut vp = viewport::new(initial_width, vp_height);
        vp.set_content(WELCOME_TEXT);

        let sender_style = Style::new().foreground(Color::from("5"));

        // Also request an async window size update for environments where the
        // initial synchronous size isn't available or later changes occur.
        let window_size_cmd = bubbletea_rs::command::window_size();

        (
            Self {
                viewport: vp,
                textinput: ti,
                messages: vec![],
                sender_style,
                keys: KeyBindings::default(),
                term_width: initial_width,
                term_height: initial_height,
                input_height: 1,
            },
            Some(window_size_cmd),
        )
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Window size: update layout and recompute content wrapping
        if let Some(ws) = msg.downcast_ref::<bubbletea_rs::WindowSizeMsg>() {
            self.term_width = ws.width as usize;
            self.term_height = ws.height as usize;

            // Resize the textinput to full width minus prompt
            self.textinput.set_width((self.term_width.saturating_sub(2)) as i32);

            // Recreate viewport with new size: height = terminal - input height - gap
            let gap_h = GAP.matches('\n').count();
            let input_h = self.input_height;
            let vp_height = self
                .term_height
                .saturating_sub(input_h)
                .saturating_sub(gap_h);
            let mut new_vp = viewport::new(self.term_width, vp_height.max(1));
            // Keep welcome text until first message is sent; otherwise render wrapped messages
            if self.messages.is_empty() {
                new_vp.set_content(WELCOME_TEXT);
            } else {
                let content = self.compute_wrapped_content(self.term_width, vp_height.max(1));
                new_vp.set_content(&content);
            }
            new_vp.goto_bottom();
            self.viewport = new_vp;
            return None;
        }

        // Intercept app-level keys first
        if let Some(k) = msg.downcast_ref::<KeyMsg>() {
            if self.keys.quit.matches(k) || self.keys.quit_alt.matches(k) {
                println!("{}", self.textinput.value());
                return Some(quit());
            }
            
            if self.keys.send.matches(k) {
                let value = self.textinput.value();
                if !value.trim().is_empty() {
                    let sender = self.sender_style.render("You: ");
                    let line = format!("{}{}", sender, value);
                    self.messages.push(line);

                    // Re-wrap and update viewport content
                    let height = self.viewport_height_current();
                    let content = self.compute_wrapped_content(self.term_width, height);
                    self.viewport.set_content(&content);
                    self.viewport.goto_bottom();

                    // Clear textinput content
                    self.textinput.set_value("");
                }
                return None;
            }
        }

        // Delegate to textinput only; viewport content is managed by this model
        self.textinput.update(msg)
    }

    fn view(&self) -> String {
        format!("{}{}{}", self.viewport.view(), GAP, self.textinput.view())
    }
}

impl ChatModel {
    fn compute_wrapped_content(&self, width: usize, height: usize) -> String {
        let wrapped_lines: Vec<String> = self
            .messages
            .iter()
            .flat_map(|m| {
                wrap_text(m, width)
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .collect();
        let start = wrapped_lines.len().saturating_sub(height);
        let visible = &wrapped_lines[start..];
        visible.join("\n")
    }

    fn viewport_height_current(&self) -> usize {
        // Derive current viewport height from terminal and input
        let gap_h = GAP.matches('\n').count();
        self.term_height
            .saturating_sub(self.input_height)
            .saturating_sub(gap_h)
            .max(1)
    }
}

fn wrap_text(s: &str, width: usize) -> String {
    if width == 0 {
        return s.to_string();
    }
    let mut out = Vec::new();
    for raw_line in s.split('\n') {
        let mut line = String::new();
        for word in raw_line.split_whitespace() {
            if line.is_empty() {
                if word.len() > width {
                    // Hard wrap long words
                    let mut w = word;
                    while w.len() > width {
                        out.push(w[..width].to_string());
                        w = &w[width..];
                    }
                    line.push_str(w);
                } else {
                    line.push_str(word);
                }
            } else {
                if line.len() + 1 + word.len() > width {
                    out.push(std::mem::take(&mut line));
                    if word.len() > width {
                        let mut w = word;
                        while w.len() > width {
                            out.push(w[..width].to_string());
                            w = &w[width..];
                        }
                        line.push_str(w);
                    } else {
                        line.push_str(word);
                    }
                } else {
                    line.push(' ');
                    line.push_str(word);
                }
            }
        }
        out.push(line);
    }
    out.join("\n")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<ChatModel>::builder()
        // The Go example doesn't specify it, but Bubble Tea uses alt screen by default.
        // We enable it here for a cleaner UI.
        .alt_screen(true)
        .signal_handler(true)
        .build()?;

    program.run().await?;
    Ok(())
}
