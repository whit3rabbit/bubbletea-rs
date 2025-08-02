use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_rs::command::{batch, tick, window_size};
use crossterm::style::Stylize; // for simple coloring; TODO(lipgloss): replace with lipgloss styles
use std::fmt::Write as _;

// Port of Bubble Tea's chat example.
// - Uses a viewport area for messages and a simple textarea for input.
// - TODO(lipgloss): Apply lipgloss styles (placeholder dark gray, sender label bright pink)
// - TODO(lipgloss): Respect dynamic width with style.Width(viewport_width) wrapping.

const GAP: &str = "\n\n"; // matches the Go example

// Cursor blink timing
const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(530);

#[derive(Debug, Clone)]
struct BlinkMsg;

use std::time::Duration;

// Error handling - matches Go version but not actively used in this example
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ErrMsg(String);

#[derive(Debug, Default, Clone)]
struct TextArea {
    value: String,
    placeholder: String,
    width: usize,
    height: usize,
    prompt: String,
    char_limit: usize,
    cursor_visible: bool,
    focused: bool,
}

impl TextArea {
    fn new() -> Self {
        Self {
            placeholder: "Send a message...".into(),
            width: 30,
            height: 3,
            prompt: "┃ ".into(),
            char_limit: 280,
            cursor_visible: true,
            focused: true,
            ..Default::default()
        }
    }
    fn set_width(&mut self, w: usize) { self.width = w; }
    fn set_height(&mut self, h: usize) { self.height = h; }
    fn reset(&mut self) { self.value.clear(); }
    fn view(&self) -> String {
        let mut out = String::new();
        // Show placeholder when input is empty (even if focused), like the Go example,
        // and render it in dark gray to appear subtle.
        if self.value.is_empty() {
            let _ = write!(&mut out, "{}{}", self.prompt, self.placeholder.clone().dark_grey());
        } else {
            let _ = write!(&mut out, "{}{}", self.prompt, self.value);
        }
        // Add cursor if focused and visible
        if self.focused && self.cursor_visible {
            out.push('█');
        }
        out
    }
}

#[derive(Debug, Default, Clone)]
struct Viewport {
    width: usize,
    height: usize,
    content: String,
    scroll: usize,
}

impl Viewport {
    fn new(width: usize, height: usize) -> Self { Self { width, height, ..Default::default() } }
    fn set_content(&mut self, s: String) { self.content = s; }
    fn goto_bottom(&mut self) { self.scroll = usize::MAX; }
    fn view(&self) -> String {
        // Just return the content; wrapping is handled by the model when setting content.
        self.content.clone()
    }
}

// (no custom WindowSizeMsg needed; we use bubbletea_rs::WindowSizeMsg)

#[derive(Debug)]
struct ChatModel {
    viewport: Viewport,
    textarea: TextArea,
    messages: Vec<String>,
    // TODO(lipgloss): senderStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("5"))
}

impl Model for ChatModel {
    fn init() -> (Self, Option<Cmd>) {
        let mut ta = TextArea::new();
        ta.set_width(30);
        ta.set_height(3);

        let mut vp = Viewport::new(30, 5);
        vp.set_content("Welcome to the chat room!\nType a message and press Enter to send.".into());

        (
            Self { viewport: vp, textarea: ta, messages: vec![] },
            Some(batch(vec![
                window_size(),
                tick(CURSOR_BLINK_INTERVAL, |_| Box::new(BlinkMsg) as Msg)
            ])),
        )
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        let mut cmds = Vec::new();
        
        // Handle blink message
        if msg.downcast_ref::<BlinkMsg>().is_some() {
            self.textarea.cursor_visible = !self.textarea.cursor_visible;
            cmds.push(tick(CURSOR_BLINK_INTERVAL, |_| Box::new(BlinkMsg) as Msg));
        }
        
        if let Some(ws) = msg.downcast_ref::<bubbletea_rs::WindowSizeMsg>() {
            self.viewport.width = ws.width as usize;
            self.textarea.set_width(ws.width as usize);
            // Height calculation: viewport fills remaining space above textarea and gap.
            let gap_h = GAP.matches('\n').count() + 1; // approx lipgloss.Height(gap)
            self.viewport.height = (ws.height as usize).saturating_sub(self.textarea.height).saturating_sub(gap_h);
            // Recompute wrapped viewport content at the new width
            self.recompute_viewport_content();
            self.viewport.goto_bottom();
            return if cmds.is_empty() { None } else { Some(batch(cmds)) };
        }

        if let Some(k) = msg.downcast_ref::<KeyMsg>() {
            use crossterm::event::{KeyCode, KeyModifiers};
            use crossterm::style::Stylize;
            match k.key {
                KeyCode::Esc => {
                    // Match Go example: print textarea value on quit
                    println!("{}", self.textarea.value);
                    return Some(quit());
                }
                KeyCode::Enter => {
                    // Colorize the "You:" label (bright pink approximation)
                    let sender = "You: ".magenta().bold().to_string();
                    let line = format!("{}{}", sender, self.textarea.value);
                    self.messages.push(line);
                    // Re-wrap to current viewport width
                    self.recompute_viewport_content();
                    self.textarea.reset();
                    self.viewport.goto_bottom();
                    return None;
                }
                KeyCode::Backspace => { self.textarea.value.pop(); return None; }
                KeyCode::Char(ch) => {
                    // Detect Ctrl+C (quit)
                    if ch == 'c' && k.modifiers.contains(KeyModifiers::CONTROL) {
                        println!("{}", self.textarea.value);
                        return Some(quit());
                    }
                    if self.textarea.value.len() < self.textarea.char_limit { self.textarea.value.push(ch); }
                    return None;
                }
                _ => {}
            }
        }
        
        if cmds.is_empty() { None } else { Some(batch(cmds)) }
    }

    fn view(&self) -> String {
        format!("{}{}{}", self.viewport.view(), GAP, self.textarea.view())
    }
}

impl ChatModel {
    fn recompute_viewport_content(&mut self) {
        let width = self.viewport.width.max(1);
        let wrapped_lines: Vec<String> = self
            .messages
            .iter()
            .flat_map(|m| wrap_text(m, width).lines().map(|s| s.to_string()).collect::<Vec<_>>())
            .collect();

        // Keep only the bottom-most lines that fit in height to mimic viewport bottom behavior
        let start = wrapped_lines.len().saturating_sub(self.viewport.height);
        let visible = &wrapped_lines[start..];
        self.viewport.set_content(visible.join("\n"));
    }
}

fn wrap_text(s: &str, width: usize) -> String {
    if width == 0 { return s.to_string(); }
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
