//! Views Example
//!
//! An example demonstrating an application with multiple views, matching the Go
//! Bubble Tea views example.
//!
//! This example shows:
//! - Multiple view states with transitions
//! - Menu selection with keyboard navigation
//! - Animated progress bar with custom gradient
//! - OutBounce easing animation
//! - Countdown timers and automatic progression
//!
//! Note: This example implements a custom progress bar with gradient colors,
//! similar to how the original Go example was written before the Bubbles
//! progress component was available.

use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};
use lipgloss_extras::lipgloss::{Color, Style};
use std::time::Duration;

/// Progress bar configuration constants
const PROGRESS_BAR_WIDTH: usize = 71;
const PROGRESS_FULL_CHAR: char = '█';
const PROGRESS_EMPTY_CHAR: char = '░';
const DOT_CHAR: &str = " • ";

/// Message for tick updates (1 second intervals)
#[derive(Debug)]
pub struct TickMsg;

/// Message for frame updates (60fps intervals)
#[derive(Debug)]
pub struct FrameMsg;

/// The application model
#[derive(Debug)]
pub struct ViewsModel {
    choice: usize,
    chosen: bool,
    ticks: i32,
    frames: i32,
    progress: f64,
    loaded: bool,
    quitting: bool,
}

impl ViewsModel {
    fn new() -> Self {
        Self {
            choice: 0,
            chosen: false,
            ticks: 10, // Start with 10-second countdown
            frames: 0,
            progress: 0.0,
            loaded: false,
            quitting: false,
        }
    }
}

impl Model for ViewsModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = ViewsModel::new();
        (
            model,
            Some(tick(Duration::from_secs(1), |_| Box::new(TickMsg) as Msg)),
        )
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle keyboard input - always check for quit keys first
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quitting = true;
                    return Some(quit());
                }
                _ => {}
            }
        }

        // Route to appropriate update function based on current state
        if !self.chosen {
            self.update_choices(msg)
        } else {
            self.update_chosen(msg)
        }
    }

    fn view(&self) -> String {
        if self.quitting {
            return "\n  See you later!\n\n".to_string();
        }

        let content = if !self.chosen {
            self.choices_view()
        } else {
            self.chosen_view()
        };

        // Apply main style with left margin
        let main_style = Style::new().margin_left(2);
        main_style.render(&format!("\n{}\n\n", content))
    }
}

impl ViewsModel {
    /// Update loop for the first view where you're choosing a task
    fn update_choices(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('j') | KeyCode::Down => {
                    if self.choice < 3 {
                        self.choice += 1;
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if self.choice > 0 {
                        self.choice -= 1;
                    }
                }
                KeyCode::Enter => {
                    self.chosen = true;
                    return Some(tick(Duration::from_millis(1000 / 60), |_| {
                        Box::new(FrameMsg) as Msg
                    }));
                }
                _ => {}
            }
        }

        if msg.downcast_ref::<TickMsg>().is_some() {
            if self.ticks == 0 {
                self.quitting = true;
                return Some(quit());
            }
            self.ticks -= 1;
            return Some(tick(Duration::from_secs(1), |_| Box::new(TickMsg) as Msg));
        }

        None
    }

    /// Update loop for the second view after a choice has been made
    fn update_chosen(&mut self, msg: Msg) -> Option<Cmd> {
        if msg.downcast_ref::<FrameMsg>().is_some() {
            if !self.loaded {
                self.frames += 1;
                self.progress = ease_out_bounce(self.frames as f64 / 100.0);
                if self.progress >= 1.0 {
                    self.progress = 1.0;
                    self.loaded = true;
                    self.ticks = 3;
                    return Some(tick(Duration::from_secs(1), |_| Box::new(TickMsg) as Msg));
                }
                return Some(tick(Duration::from_millis(1000 / 60), |_| {
                    Box::new(FrameMsg) as Msg
                }));
            }
        }

        if msg.downcast_ref::<TickMsg>().is_some() && self.loaded {
            if self.ticks == 0 {
                self.quitting = true;
                return Some(quit());
            }
            self.ticks -= 1;
            return Some(tick(Duration::from_secs(1), |_| Box::new(TickMsg) as Msg));
        }

        None
    }

    /// The first view, where you're choosing a task
    fn choices_view(&self) -> String {
        let mut content = String::new();
        content.push_str("What to do today?\n\n");

        // Add choices
        content.push_str(&format!(
            "{}\n{}\n{}\n{}\n\n",
            checkbox("Plant carrots", self.choice == 0),
            checkbox("Go to the market", self.choice == 1),
            checkbox("Read something", self.choice == 2),
            checkbox("See friends", self.choice == 3),
        ));

        // Add countdown
        let ticks_style = Style::new().foreground(Color::from("79"));
        content.push_str(&format!(
            "Program quits in {} seconds\n\n",
            ticks_style.render(&self.ticks.to_string())
        ));

        // Add help text
        let subtle_style = Style::new().foreground(Color::from("241"));
        let dot_style = Style::new().foreground(Color::from("236"));
        content.push_str(&format!(
            "{}{}{}{}{}",
            subtle_style.render("j/k, up/down: select"),
            dot_style.render(DOT_CHAR),
            subtle_style.render("enter: choose"),
            dot_style.render(DOT_CHAR),
            subtle_style.render("q, esc: quit")
        ));

        content
    }

    /// The second view, after a task has been chosen
    fn chosen_view(&self) -> String {
        let keyword_style = Style::new().foreground(Color::from("211"));
        let ticks_style = Style::new().foreground(Color::from("79"));

        let task_msg = match self.choice {
            0 => format!(
                "Carrot planting?\n\nCool, we'll need {} and {}...",
                keyword_style.render("libgarden"),
                keyword_style.render("vegeutils")
            ),
            1 => format!(
                "A trip to the market?\n\nOkay, then we should install {} and {}...",
                keyword_style.render("marketkit"),
                keyword_style.render("libshopping")
            ),
            2 => format!(
                "Reading time?\n\nOkay, cool, then we'll need a library. Yes, an {}.",
                keyword_style.render("actual library")
            ),
            _ => format!(
                "It's always good to see friends.\n\nFetching {} and {}...",
                keyword_style.render("social-skills"),
                keyword_style.render("conversationutils")
            ),
        };

        let label = if self.loaded {
            format!(
                "Downloaded. Exiting in {} seconds...",
                ticks_style.render(&self.ticks.to_string())
            )
        } else {
            "Downloading...".to_string()
        };

        format!(
            "{}\n\n{}\n{}%",
            task_msg,
            label,
            progress_bar(self.progress)
        )
    }
}

/// Create a checkbox display
fn checkbox(label: &str, checked: bool) -> String {
    let checkbox_style = Style::new().foreground(Color::from("212"));
    if checked {
        checkbox_style.render(&format!("[x] {}", label))
    } else {
        format!("[ ] {}", label)
    }
}

/// Create a progress bar with custom gradient
fn progress_bar(percent: f64) -> String {
    let percent = percent.clamp(0.0, 1.0);
    let width = PROGRESS_BAR_WIDTH as f64;

    let full_size = (width * percent).round() as usize;
    let empty_size = PROGRESS_BAR_WIDTH - full_size;

    // Create gradient ramp from #B14FFF to #00FFA3
    let gradient_ramp = make_gradient_ramp("#B14FFF", "#00FFA3", PROGRESS_BAR_WIDTH);

    let mut full_cells = String::new();
    for i in 0..full_size {
        full_cells.push_str(&gradient_ramp[i]);
    }

    let subtle_style = Style::new().foreground(Color::from("241"));
    let empty_cells = subtle_style.render(&PROGRESS_EMPTY_CHAR.to_string().repeat(empty_size));

    format!(
        "{}{} {:3.0}",
        full_cells,
        empty_cells,
        (percent * 100.0).round()
    )
}

/// Create a gradient ramp similar to the Go version's makeRampStyles
fn make_gradient_ramp(color_a: &str, color_b: &str, steps: usize) -> Vec<String> {
    let start_color = hex_to_rgb(color_a);
    let end_color = hex_to_rgb(color_b);

    let mut ramp = Vec::with_capacity(steps);

    for i in 0..steps {
        let t = if steps <= 1 {
            0.0
        } else {
            i as f64 / (steps - 1) as f64
        };

        let r =
            (start_color.0 as f64 + (end_color.0 as f64 - start_color.0 as f64) * t).round() as u8;
        let g =
            (start_color.1 as f64 + (end_color.1 as f64 - start_color.1 as f64) * t).round() as u8;
        let b =
            (start_color.2 as f64 + (end_color.2 as f64 - start_color.2 as f64) * t).round() as u8;

        let hex_color = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let style = Style::new().foreground(Color::from(hex_color.as_str()));
        ramp.push(style.render(&PROGRESS_FULL_CHAR.to_string()));
    }

    ramp
}

/// Convert hex color string to RGB tuple
fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    (r, g, b)
}

/// OutBounce easing function - matches the fogleman/ease OutBounce
fn ease_out_bounce(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);

    if t < (1.0 / 2.75) {
        7.5625 * t * t
    } else if t < (2.0 / 2.75) {
        let t = t - (1.5 / 2.75);
        7.5625 * t * t + 0.75
    } else if t < (2.5 / 2.75) {
        let t = t - (2.25 / 2.75);
        7.5625 * t * t + 0.9375
    } else {
        let t = t - (2.625 / 2.75);
        7.5625 * t * t + 0.984375
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<ViewsModel>::builder()
        .signal_handler(true)
        .build()?;

    program.run().await?;
    Ok(())
}
