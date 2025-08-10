//! Spinners Example
//!
//! Demonstrates:
//! - Multiple spinner styles and animations
//! - Navigation between different spinner types
//! - Professional spinner showcase with different characteristics
//! - Keyboard navigation to browse spinner gallery
//! - Different timing intervals for various spinner styles
//!
//! This example shows a collection of different spinner animations
//! that users can navigate through, demonstrating the variety of
//! loading indicators available in terminal applications.

use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};
use std::time::Duration;

/// Message for spinner animation ticks
#[derive(Debug)]
pub struct SpinnerTickMsg;

/// Different spinner styles available in the gallery
#[derive(Debug, Clone, PartialEq)]
pub enum SpinnerStyle {
    Line,
    Dots,
    MiniDots,
    Jump,
    Pulse,
    Points,
    Globe,
    Moon,
    Monkey,
    Arc,
    Bounce,
    Clock,
}

impl SpinnerStyle {
    /// Get all available spinner styles
    pub fn all() -> Vec<SpinnerStyle> {
        vec![
            SpinnerStyle::Line,
            SpinnerStyle::Dots,
            SpinnerStyle::MiniDots,
            SpinnerStyle::Jump,
            SpinnerStyle::Pulse,
            SpinnerStyle::Points,
            SpinnerStyle::Globe,
            SpinnerStyle::Moon,
            SpinnerStyle::Monkey,
            SpinnerStyle::Arc,
            SpinnerStyle::Bounce,
            SpinnerStyle::Clock,
        ]
    }

    /// Get the animation frames for this spinner style
    pub fn frames(&self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Line => &["|", "/", "-", "\\"],
            SpinnerStyle::Dots => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerStyle::MiniDots => &["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"],
            SpinnerStyle::Jump => &["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"],
            SpinnerStyle::Pulse => &[
                "█", "▉", "▊", "▋", "▌", "▍", "▎", "▏", "▎", "▍", "▌", "▋", "▊", "▉",
            ],
            SpinnerStyle::Points => &["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"],
            SpinnerStyle::Globe => &["🌍", "🌎", "🌏"],
            SpinnerStyle::Moon => &["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
            SpinnerStyle::Monkey => &["🙈", "🙉", "🙊", "🐵"],
            SpinnerStyle::Arc => &["◜", "◠", "◝", "◞", "◡", "◟"],
            SpinnerStyle::Bounce => &["⠁", "⠂", "⠄", "⠂"],
            SpinnerStyle::Clock => &[
                "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
            ],
        }
    }

    /// Get the animation interval for this spinner style
    pub fn interval(&self) -> Duration {
        match self {
            SpinnerStyle::Line => Duration::from_millis(150),
            SpinnerStyle::Dots => Duration::from_millis(100),
            SpinnerStyle::MiniDots => Duration::from_millis(120),
            SpinnerStyle::Jump => Duration::from_millis(130),
            SpinnerStyle::Pulse => Duration::from_millis(80),
            SpinnerStyle::Points => Duration::from_millis(400),
            SpinnerStyle::Globe => Duration::from_millis(500),
            SpinnerStyle::Moon => Duration::from_millis(200),
            SpinnerStyle::Monkey => Duration::from_millis(300),
            SpinnerStyle::Arc => Duration::from_millis(120),
            SpinnerStyle::Bounce => Duration::from_millis(300),
            SpinnerStyle::Clock => Duration::from_millis(500),
        }
    }

    /// Get the display name for this spinner style
    pub fn name(&self) -> &'static str {
        match self {
            SpinnerStyle::Line => "Line",
            SpinnerStyle::Dots => "Dots",
            SpinnerStyle::MiniDots => "Mini Dots",
            SpinnerStyle::Jump => "Jump",
            SpinnerStyle::Pulse => "Pulse",
            SpinnerStyle::Points => "Points",
            SpinnerStyle::Globe => "Globe",
            SpinnerStyle::Moon => "Moon",
            SpinnerStyle::Monkey => "Monkey",
            SpinnerStyle::Arc => "Arc",
            SpinnerStyle::Bounce => "Bounce",
            SpinnerStyle::Clock => "Clock",
        }
    }

    /// Get a description for this spinner style
    pub fn description(&self) -> &'static str {
        match self {
            SpinnerStyle::Line => "Classic rotating line",
            SpinnerStyle::Dots => "Braille dot pattern",
            SpinnerStyle::MiniDots => "Small braille dots",
            SpinnerStyle::Jump => "Jumping braille pattern",
            SpinnerStyle::Pulse => "Pulsing bar effect",
            SpinnerStyle::Points => "Three-dot sequence",
            SpinnerStyle::Globe => "Rotating earth emoji",
            SpinnerStyle::Moon => "Moon phase cycle",
            SpinnerStyle::Monkey => "See no evil monkeys",
            SpinnerStyle::Arc => "Curved arc rotation",
            SpinnerStyle::Bounce => "Bouncing dot",
            SpinnerStyle::Clock => "Clock face animation",
        }
    }
}

/// Key bindings for the spinners example
#[derive(Debug)]
pub struct KeyBindings {
    pub quit: Binding,
    pub quit_alt: Binding,
    pub left: Binding,
    pub right: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![
                with_keys_str(&["q", "esc"]),
                with_help("q", "quit"),
            ]),
            quit_alt: new_binding(vec![
                with_keys_str(&["ctrl+c"]),
                with_help("ctrl+c", "quit"),
            ]),
            left: new_binding(vec![
                with_keys_str(&["left", "h"]),
                with_help("h/←", "previous spinner"),
            ]),
            right: new_binding(vec![
                with_keys_str(&["right", "l"]),
                with_help("l/→", "next spinner"),
            ]),
        }
    }
}

/// The application state
#[derive(Debug)]
pub struct SpinnersModel {
    pub spinners: Vec<SpinnerStyle>,
    pub current_index: usize,
    pub current_frame: usize,
    pub quitting: bool,
    pub keys: KeyBindings,
}

impl SpinnersModel {
    pub fn new() -> Self {
        Self {
            spinners: SpinnerStyle::all(),
            current_index: 0,
            current_frame: 0,
            quitting: false,
            keys: KeyBindings::default(),
        }
    }

    pub fn current_spinner(&self) -> &SpinnerStyle {
        &self.spinners[self.current_index]
    }

    pub fn current_frame_text(&self) -> &str {
        let frames = self.current_spinner().frames();
        frames[self.current_frame % frames.len()]
    }

    pub fn advance_frame(&mut self) {
        let frames = self.current_spinner().frames();
        self.current_frame = (self.current_frame + 1) % frames.len();
    }

    pub fn previous_spinner(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        } else {
            self.current_index = self.spinners.len() - 1;
        }
        self.current_frame = 0; // Reset animation
    }

    pub fn next_spinner(&mut self) {
        self.current_index = (self.current_index + 1) % self.spinners.len();
        self.current_frame = 0; // Reset animation
    }
}

impl Model for SpinnersModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = SpinnersModel::new();
        let interval = model.current_spinner().interval();

        // Start the spinner animation
        let cmd = tick(interval, |_| Box::new(SpinnerTickMsg) as Msg);
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle spinner tick messages
        if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
            if !self.quitting {
                self.advance_frame();
                let interval = self.current_spinner().interval();
                return Some(tick(interval, |_| Box::new(SpinnerTickMsg) as Msg));
            }
        }

        // Handle keyboard input using key bindings
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if self.keys.quit.matches(key_msg) || self.keys.quit_alt.matches(key_msg) {
                self.quitting = true;
                return Some(quit());
            }
            if self.keys.left.matches(key_msg) {
                self.previous_spinner();
                // Restart animation with new spinner's interval
                let interval = self.current_spinner().interval();
                return Some(tick(interval, |_| Box::new(SpinnerTickMsg) as Msg));
            }
            if self.keys.right.matches(key_msg) {
                self.next_spinner();
                // Restart animation with new spinner's interval
                let interval = self.current_spinner().interval();
                return Some(tick(interval, |_| Box::new(SpinnerTickMsg) as Msg));
            }
        }

        None
    }

    fn view(&self) -> String {
        let spinner = self.current_spinner();
        let frame = self.current_frame_text();

        // Some spinners need no gap, others need a space
        let gap = match spinner {
            SpinnerStyle::Dots | SpinnerStyle::MiniDots => "",
            _ => " ",
        };

        let mut view = String::new();
        view.push_str(&format!("\n {}{}Spinning...\n\n", frame, gap));

        // Show spinner info
        view.push_str(&format!(
            " Style: {} ({}/{})\n",
            spinner.name(),
            self.current_index + 1,
            self.spinners.len()
        ));
        view.push_str(&format!(" Description: {}\n", spinner.description()));
        view.push_str(&format!(
            " Interval: {}ms\n",
            spinner.interval().as_millis()
        ));
        view.push_str(&format!(" Frames: {}\n\n", spinner.frames().len()));

        // Help text
        view.push_str(" h/l, ←/→: change spinner • q: exit\n");

        if self.quitting {
            view.push('\n');
        }

        view
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting spinners gallery...");
    println!("Navigate through different spinner styles!");

    // Create and run the program
    let program = Program::<SpinnersModel>::builder()
        .alt_screen(true) // Use alternate screen for cleaner display
        .signal_handler(true) // Enable Ctrl+C handling
        .build()?;

    // Run the program
    program.run().await?;

    println!("Spinners gallery closed.");

    Ok(())
}
