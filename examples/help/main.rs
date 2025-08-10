//! Help Example
//!
//! Demonstrates:
//! - Key binding system with help text
//! - Toggle between short and full help modes
//! - Arrow key navigation with visual feedback
//! - Responsive help text formatting
//! - Styled output with colors
//!
//! This example shows how to implement a help system that displays
//! key bindings and can toggle between mini and full help views.

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
use crossterm::event::KeyCode;
use lipgloss_extras::lipgloss::{Color, Style};
use crossterm::terminal;
use std::env;
use unicode_width::UnicodeWidthStr;

// Synthetic message used to trigger the initial render immediately after startup.
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

/// Represents a key binding with its key, visual representation, and help text
#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub keys: Vec<KeyCode>,
    pub help_key: &'static str,
    pub help_desc: &'static str,
    pub symbol: &'static str,
}

impl KeyBinding {
    pub fn new(
        keys: Vec<KeyCode>,
        help_key: &'static str,
        help_desc: &'static str,
        symbol: &'static str,
    ) -> Self {
        Self {
            keys,
            help_key,
            help_desc,
            symbol,
        }
    }

    pub fn matches(&self, key: KeyCode) -> bool {
        self.keys.contains(&key)
    }
}

/// The help model containing key bindings and application state
#[derive(Debug)]
pub struct HelpModel {
    pub keys: Vec<KeyBinding>,
    pub help_expanded: bool,
    pub last_key: Option<String>,
    pub terminal_width: u16,
    pub quitting: bool,
}

impl HelpModel {
    pub fn new() -> Self {
        let keys = vec![
            KeyBinding::new(vec![KeyCode::Up, KeyCode::Char('k')], "↑/k", "move up", "↑"),
            KeyBinding::new(
                vec![KeyCode::Down, KeyCode::Char('j')],
                "↓/j",
                "move down",
                "↓",
            ),
            KeyBinding::new(
                vec![KeyCode::Left, KeyCode::Char('h')],
                "←/h",
                "move left",
                "←",
            ),
            KeyBinding::new(
                vec![KeyCode::Right, KeyCode::Char('l')],
                "→/l",
                "move right",
                "→",
            ),
            KeyBinding::new(vec![KeyCode::Char('?')], "?", "toggle help", ""),
            KeyBinding::new(
                vec![KeyCode::Char('q'), KeyCode::Esc, KeyCode::Char('c')],
                "q",
                "quit",
                "",
            ),
        ];

        Self {
            keys,
            help_expanded: false,
            last_key: None,
            terminal_width: 80,
            quitting: false,
        }
    }

    pub fn find_key_binding(&self, key: KeyCode) -> Option<&KeyBinding> {
        self.keys.iter().find(|binding| binding.matches(key))
    }

    pub fn short_help(&self) -> Vec<&KeyBinding> {
        self.keys
            .iter()
            .filter(|k| k.help_key == "?" || k.help_key == "q")
            .collect()
    }

    pub fn full_help(&self) -> (Vec<&KeyBinding>, Vec<&KeyBinding>) {
        let navigation: Vec<&KeyBinding> = self
            .keys
            .iter()
            .filter(|k| matches!(k.help_key, "↑/k" | "↓/j" | "←/h" | "→/l"))
            .collect();
        let actions: Vec<&KeyBinding> = self
            .keys
            .iter()
            .filter(|k| matches!(k.help_key, "?" | "q"))
            .collect();
        (navigation, actions)
    }

    pub fn format_help_line(bindings: &[&KeyBinding]) -> String {
        bindings
            .iter()
            .map(|binding| format!("{} {}", binding.help_key, binding.help_desc))
            .collect::<Vec<_>>()
            .join(" • ")
    }

    pub fn format_help_columns(col1: &[&KeyBinding], col2: &[&KeyBinding], width: u16) -> String {
        // Size columns based on content widths with truncation if needed, similar to Bubbles.
        let left_items: Vec<String> = col1
            .iter()
            .map(|b| format!("{} {}", b.help_key, b.help_desc))
            .collect();
        let right_items: Vec<String> = col2
            .iter()
            .map(|b| format!("{} {}", b.help_key, b.help_desc))
            .collect();

        let left_max = left_items
            .iter()
            .map(|s| UnicodeWidthStr::width(s.as_str()))
            .max()
            .unwrap_or(0);
        let right_max = right_items
            .iter()
            .map(|s| UnicodeWidthStr::width(s.as_str()))
            .max()
            .unwrap_or(0);

        let spacing = 2usize; // two spaces between columns
        let avail = width as usize;

        // Start with content-driven widths
        let mut left_w = left_max;
        let mut right_w = right_max;

        // If overflow, constrain to half-splits as an upper bound, then clamp
        if left_w + spacing + right_w > avail {
            let half = avail.saturating_sub(spacing) / 2;
            left_w = left_w.min(half);
            right_w = right_w.min(half);
            if left_w + spacing + right_w > avail {
                right_w = avail.saturating_sub(spacing + left_w);
            }
        }

        let mut lines = Vec::new();
        let max_len = col1.len().max(col2.len());

        for i in 0..max_len {
            let left = if i < left_items.len() {
                &left_items[i]
            } else {
                ""
            };
            let right = if i < right_items.len() {
                &right_items[i]
            } else {
                ""
            };

            // Truncate/pad left to left_w so right column starts uniformly
            let left_display = UnicodeWidthStr::width(left);
            let left_cell = if left_display > left_w {
                let mut acc = String::new();
                let mut w = 0;
                for ch in left.chars() {
                    let cw = UnicodeWidthStr::width(ch.to_string().as_str());
                    if w + cw + 3 > left_w {
                        break;
                    }
                    acc.push(ch);
                    w += cw;
                }
                acc.push_str("...");
                acc
            } else {
                let padding = left_w.saturating_sub(left_display);
                format!("{}{}", left, " ".repeat(padding))
            };

            // Truncate right if it exceeds its width; no need to pad at end
            let right_display = UnicodeWidthStr::width(right);
            let right_cell = if right_display > right_w {
                let mut acc = String::new();
                let mut w = 0;
                for ch in right.chars() {
                    let cw = UnicodeWidthStr::width(ch.to_string().as_str());
                    if w + cw + 3 > right_w {
                        break;
                    }
                    acc.push(ch);
                    w += cw;
                }
                acc.push_str("...");
                acc
            } else {
                right.to_string()
            };

            let line = if right_cell.is_empty() {
                left_cell
            } else {
                format!("{}{}{}", left_cell, " ".repeat(spacing), right_cell)
            };
            lines.push(line.trim_start().trim_end().to_string());
        }

        lines.join("\n")
    }

    /// Format an arbitrary list of bindings into a grid with `columns` columns,
    /// using Unicode-aware width calculations for alignment.
    pub fn format_help_grid(bindings: &[&KeyBinding], columns: usize, width: u16) -> String {
        let columns = columns.max(1);
        let total_padding = 2 /*left*/ + (columns.saturating_sub(1)) * 2; // two spaces between cols
        let col_width = ((width as usize).saturating_sub(total_padding)) / columns;
        let rows = (bindings.len() + columns - 1) / columns;

        let mut out = Vec::with_capacity(rows);
        for r in 0..rows {
            let mut line = String::new(); // no left margin; flush left
            for c in 0..columns {
                let idx = r + c * rows; // column-major to balance column heights
                let cell = if idx < bindings.len() {
                    format!("{} {}", bindings[idx].help_key, bindings[idx].help_desc)
                } else {
                    String::new()
                };

                // Unicode-aware padding/truncation per cell
                let cell_display_width = UnicodeWidthStr::width(cell.as_str());
                let cell_text = if cell_display_width > col_width {
                    let mut acc = String::new();
                    let mut w = 0;
                    for ch in cell.chars() {
                        let cw = UnicodeWidthStr::width(ch.to_string().as_str());
                        if w + cw + 3 > col_width {
                            // reserve for "..."
                            break;
                        }
                        acc.push(ch);
                        w += cw;
                    }
                    acc.push_str("...");
                    acc
                } else {
                    let padding = col_width.saturating_sub(cell_display_width);
                    format!("{}{}", cell, " ".repeat(padding))
                };

                line.push_str(&cell_text);
                if c + 1 < columns {
                    line.push_str("  ");
                }
            }
            out.push(line.trim_start().trim_end().to_string());
        }

        out.join("\n")
    }
}

impl Model for HelpModel {
    fn init() -> (Self, Option<Cmd>) {
        // Initialize model and set an initial terminal width if available.
        let mut model = Self::new();
        if let Ok((w, _h)) = terminal::size() {
            model.terminal_width = w;
        }

        // Emit a synthetic message immediately to trigger the first render.
        // This keeps the example framework-agnostic and ensures the UI is drawn
        // right away without waiting for the first input event.
        (model, Some(init_render_cmd()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if let Some(binding) = self.find_key_binding(key_msg.key) {
                match binding.help_key {
                    "↑/k" => self.last_key = Some("↑".to_string()),
                    "↓/j" => self.last_key = Some("↓".to_string()),
                    "←/h" => self.last_key = Some("←".to_string()),
                    "→/l" => self.last_key = Some("→".to_string()),
                    "?" => self.help_expanded = !self.help_expanded,
                    "q" => {
                        self.quitting = true;
                        return Some(quit());
                    }
                    _ => {}
                }
            }
        } else if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            self.terminal_width = size_msg.width;
        } else if msg.downcast_ref::<InitRenderMsg>().is_some() {
            // No-op: receiving this message merely triggers the initial render.
        }

        None
    }

    fn view(&self) -> String {
        if self.quitting {
            return "Bye!\n".to_string();
        }

        let status = if let Some(ref last_key) = self.last_key {
            format!(
                "You chose: {}",
                Style::new().foreground(Color::from("#FF75B7")).render(last_key)
            )
        } else {
            "Waiting for input...".to_string()
        };

        let help_view = if self.help_expanded {
            let (nav_keys, action_keys) = self.full_help();
            // Mirror Go example: two distinct columns (nav vs actions), flush-left
            Self::format_help_columns(&nav_keys, &action_keys, self.terminal_width)
        } else {
            Self::format_help_line(&self.short_help())
        };

        // Match upstream Go example: place help near the bottom of a fixed-height
        // region by inserting blank lines between the status and help. This keeps
        // everything flush-left while visually anchoring the help toward the bottom.
        let status_lines = status.matches('\n').count();
        let help_lines = help_view.matches('\n').count();
        // Upstream uses 8 rows for this demo block.
        let total_block_rows: isize = 8;
        let used_rows: isize = (status_lines as isize) + (help_lines as isize);
        let pad_rows = (total_block_rows - used_rows).max(0) as usize;
        let mut out = format!("\n{}{}{}", status, "\n".repeat(pad_rows), help_view);

        // Optional: add visible left guides when HELP_DEBUG_GUIDE is set
        if env::var("HELP_DEBUG_GUIDE").is_ok() {
            out = out
                .lines()
                .map(|l| format!("|{}|", l))
                .collect::<Vec<_>>()
                .join("\n");
        }
        out
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<HelpModel>::builder()
        .signal_handler(true)
        .alt_screen(true)
        .build()?;

    program.run().await?;
    Ok(())
}
