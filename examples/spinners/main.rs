//! Spinner Example
//!
//! A simple program demonstrating spinner animations, matching the Go Bubble Tea 
//! spinner example with enhanced functionality.
//!
//! Features:
//! - Simple loading spinner with pink styling (matches Go version)
//! - Help menu to switch between different spinner types
//! - Clean interface matching the original Go version
//! - Multiple spinner styles with number key selection

use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};
use lipgloss_extras::lipgloss::{Color, Style};
use std::time::Duration;

/// Message for spinner animation ticks
#[derive(Debug)]
pub struct SpinnerTickMsg;

/// Available spinner types that can be selected
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinnerType {
    Line,
    Dot,
    MiniDot,
    Jump,
    Pulse,
    Points,
    Globe,
    Moon,
    Monkey,
}

impl SpinnerType {
    /// Get all available spinner types - matching Go spinners order
    fn all() -> &'static [SpinnerType] {
        &[
            SpinnerType::Line,
            SpinnerType::Dot,
            SpinnerType::MiniDot,
            SpinnerType::Jump,
            SpinnerType::Pulse,
            SpinnerType::Points,
            SpinnerType::Globe,
            SpinnerType::Moon,
            SpinnerType::Monkey,
        ]
    }

    /// Get the animation frames for this spinner type
    fn frames(self) -> &'static [&'static str] {
        match self {
            SpinnerType::Line => &["|", "/", "-", "\\"],
            SpinnerType::Dot => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerType::MiniDot => &["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"],
            SpinnerType::Jump => &["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"],
            SpinnerType::Pulse => &["█", "▉", "▊", "▋", "▌", "▍", "▎", "▏", "▎", "▍", "▌", "▋", "▊", "▉"],
            SpinnerType::Points => &["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"],
            SpinnerType::Globe => &["🌍", "🌎", "🌏"],
            SpinnerType::Moon => &["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
            SpinnerType::Monkey => &["🙈", "🙉", "🙊", "🐵"],
        }
    }

    /// Get the animation interval for this spinner type
    fn interval(self) -> Duration {
        match self {
            SpinnerType::Line => Duration::from_millis(150),
            SpinnerType::Dot => Duration::from_millis(100),
            SpinnerType::MiniDot => Duration::from_millis(120),
            SpinnerType::Jump => Duration::from_millis(130),
            SpinnerType::Pulse => Duration::from_millis(80),
            SpinnerType::Points => Duration::from_millis(400),
            SpinnerType::Globe => Duration::from_millis(500),
            SpinnerType::Moon => Duration::from_millis(200),
            SpinnerType::Monkey => Duration::from_millis(300),
        }
    }

}

/// The application model
#[derive(Debug)]
pub struct SpinnerModel {
    current_type: SpinnerType,
    current_frame: usize,
    quitting: bool,
    error: Option<String>,
}

impl SpinnerModel {
    fn new() -> Self {
        Self {
            current_type: SpinnerType::Line, // Default to Line like Go spinners version
            current_frame: 0,
            quitting: false,
            error: None,
        }
    }

    /// Get the current spinner frame as styled text
    fn current_spinner_frame(&self) -> String {
        let frames = self.current_type.frames();
        let frame = frames[self.current_frame % frames.len()];
        
        // Apply blue styling (#69) to match Go spinners version
        let style = Style::new().foreground(Color::from("69"));
        style.render(frame)
    }

    /// Advance to the next frame
    fn advance_frame(&mut self) {
        let frames = self.current_type.frames();
        self.current_frame = (self.current_frame + 1) % frames.len();
    }

    /// Move to previous spinner
    fn previous_spinner(&mut self) {
        let all_spinners = SpinnerType::all();
        let current_index = all_spinners.iter().position(|&s| s == self.current_type).unwrap_or(0);
        let new_index = if current_index == 0 {
            all_spinners.len() - 1
        } else {
            current_index - 1
        };
        self.current_type = all_spinners[new_index];
        self.current_frame = 0; // Reset animation
    }

    /// Move to next spinner  
    fn next_spinner(&mut self) {
        let all_spinners = SpinnerType::all();
        let current_index = all_spinners.iter().position(|&s| s == self.current_type).unwrap_or(0);
        let new_index = (current_index + 1) % all_spinners.len();
        self.current_type = all_spinners[new_index];
        self.current_frame = 0; // Reset animation
    }
}

impl Model for SpinnerModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = SpinnerModel::new();
        let interval = model.current_type.interval();

        // Start the spinner animation
        let cmd = tick(interval, |_| Box::new(SpinnerTickMsg) as Msg);
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle spinner tick messages
        if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
            if !self.quitting {
                self.advance_frame();
                let interval = self.current_type.interval();
                return Some(tick(interval, |_| Box::new(SpinnerTickMsg) as Msg));
            }
        }

        // Handle keyboard input
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Char('h') | KeyCode::Left => {
                    self.previous_spinner();
                    let interval = self.current_type.interval();
                    return Some(tick(interval, |_| Box::new(SpinnerTickMsg) as Msg));
                }
                KeyCode::Char('l') | KeyCode::Right => {
                    self.next_spinner();
                    let interval = self.current_type.interval();
                    return Some(tick(interval, |_| Box::new(SpinnerTickMsg) as Msg));
                }
                _ => {}
            }
        }

        None
    }

    fn view(&self) -> String {
        if let Some(error) = &self.error {
            return format!("Error: {}", error);
        }

        let mut s = String::new();
        
        // Determine gap spacing based on spinner type (like Go version)
        // In the Go version, index 1 (Dot) has no gap, all others have a space
        let gap = match self.current_type {
            SpinnerType::Dot => "",  // Dot spinner (index 1 in Go) needs no gap
            _ => " ",                // All other spinners need a space
        };
        
        // Main spinner display - matching Go spinners format exactly
        let text_style = Style::new().foreground(Color::from("252"));
        let spinning_text = text_style.render("Spinning...");
        
        s.push_str(&format!(
            "\n  {}{}{}\n\n",
            self.current_spinner_frame(),
            gap,
            spinning_text
        ));
        
        // Help text - matching Go format exactly
        let help_style = Style::new().foreground(Color::from("241"));
        s.push_str(&help_style.render("  h/l, ←/→: change spinner • q: exit\n"));

        if self.quitting {
            s.push('\n');
        }

        s
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting spinner example...");

    // Create and run the program
    let program = Program::<SpinnerModel>::builder()
        .alt_screen(true)
        .signal_handler(true)
        .build()?;

    // Run the program  
    program.run().await?;

    println!("Spinner example finished.");

    Ok(())
}