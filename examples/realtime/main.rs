//! Realtime Example
//!
//! Demonstrates:
//! - Real-time activity updates through channels and async commands
//! - Simulated external events at irregular intervals
//! - Channel-based communication for async activity
//! - Spinner animation combined with event counting
//! - Background task management and coordination
//!
//! This example shows how to send activity to Bubble Tea in real-time
//! through channels, simulating external events like chat messages,
//! network activity, or file system changes.

use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};
use std::time::Duration;
use tokio::sync::mpsc;

/// Message indicating that external activity has occurred
#[derive(Debug)]
pub struct ActivityMsg {
    pub event_id: u32,
}

/// Message for spinner animation ticks
#[derive(Debug)]
pub struct SpinnerTickMsg;

/// Different spinner styles for the loading indicator
#[derive(Debug, Clone, PartialEq)]
pub enum SpinnerStyle {
    Dots,
    Line,
    Arc,
}

impl SpinnerStyle {
    pub fn frames(&self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Dots => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerStyle::Line => &["|", "/", "-", "\\"],
            SpinnerStyle::Arc => &["◜", "◠", "◝", "◞", "◡", "◟"],
        }
    }

    pub fn interval(&self) -> Duration {
        Duration::from_millis(100)
    }
}

/// The application state
#[derive(Debug)]
pub struct RealtimeModel {
    pub events_received: u32,
    pub spinner_style: SpinnerStyle,
    pub spinner_frame: usize,
    pub quitting: bool,
    pub activity_receiver: Option<mpsc::UnboundedReceiver<ActivityMsg>>,
    pub last_event_id: u32,
    pub activity_started: bool,
}

impl RealtimeModel {
    pub fn new() -> (Self, mpsc::UnboundedSender<ActivityMsg>) {
        let (tx, rx) = mpsc::unbounded_channel();

        let model = Self {
            events_received: 0,
            spinner_style: SpinnerStyle::Dots,
            spinner_frame: 0,
            quitting: false,
            activity_receiver: Some(rx),
            last_event_id: 0,
            activity_started: false,
        };

        (model, tx)
    }

    pub fn current_spinner_frame(&self) -> &str {
        let frames = self.spinner_style.frames();
        frames[self.spinner_frame % frames.len()]
    }

    pub fn advance_spinner(&mut self) {
        let frames = self.spinner_style.frames();
        self.spinner_frame = (self.spinner_frame + 1) % frames.len();
    }

    pub fn record_activity(&mut self, event_id: u32) {
        self.events_received += 1;
        self.last_event_id = event_id;
    }
}

/// Simulates external activity by generating random events
/// This is a simplified version for demonstration purposes
fn simulate_activity() -> Cmd {
    Box::pin(async move {
        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};

        // Use a Send-safe RNG
        let mut rng = StdRng::from_entropy();
        let delay_ms = rng.gen_range(100..=1000);
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;

        let event_id = rng.gen_range(1..=999);
        Some(Box::new(ActivityMsg { event_id }) as Msg)
    })
}

impl Model for RealtimeModel {
    fn init() -> (Self, Option<Cmd>) {
        let (model, _tx) = RealtimeModel::new();

        // Start the spinner animation (activity will be triggered after first spinner tick)
        let spinner_cmd = tick(SpinnerStyle::Dots.interval(), |_| {
            Box::new(SpinnerTickMsg) as Msg
        });

        (model, Some(spinner_cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle spinner tick messages
        if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
            if !self.quitting {
                self.advance_spinner();
                let interval = self.spinner_style.interval();

                // Start activity simulation after first tick
                if !self.activity_started {
                    self.activity_started = true;
                    // Start the activity simulation - we'll get activity messages async
                    return Some(simulate_activity());
                }

                return Some(tick(interval, |_| Box::new(SpinnerTickMsg) as Msg));
            }
        }

        // Handle activity messages
        if let Some(activity) = msg.downcast_ref::<ActivityMsg>() {
            self.record_activity(activity.event_id);
            // Simulate next activity after a delay
            return Some(simulate_activity());
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
                KeyCode::Char(' ') => {
                    // Change spinner style
                    self.spinner_style = match self.spinner_style {
                        SpinnerStyle::Dots => SpinnerStyle::Line,
                        SpinnerStyle::Line => SpinnerStyle::Arc,
                        SpinnerStyle::Arc => SpinnerStyle::Dots,
                    };
                    self.spinner_frame = 0;
                }
                KeyCode::Char('r') => {
                    // Reset counter
                    self.events_received = 0;
                    self.last_event_id = 0;
                }
                _ => {}
            }
        }

        None
    }

    fn view(&self) -> String {
        let spinner_frame = self.current_spinner_frame();
        let style_name = match self.spinner_style {
            SpinnerStyle::Dots => "Dots",
            SpinnerStyle::Line => "Line",
            SpinnerStyle::Arc => "Arc",
        };

        let mut view = String::new();
        view.push_str(&format!(
            "\n {} Events received: {}\n\n",
            spinner_frame, self.events_received
        ));

        if self.events_received > 0 {
            view.push_str(&format!(" Last event ID: {}\n", self.last_event_id));
        }

        view.push_str(&format!(" Spinner: {} (space to change)\n", style_name));
        view.push_str(" Press 'r' to reset counter\n");
        view.push_str(" Press any other key to exit\n");

        if self.quitting {
            view.push('\n');
        }

        view
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting realtime example...");
    println!("This example simulates receiving events at random intervals.");

    // Create and run the program
    let program = Program::<RealtimeModel>::builder()
        .alt_screen(true) // Use alternate screen for cleaner display
        .signal_handler(true) // Enable Ctrl+C handling
        .build()?;

    // Run the program and get the final model state
    let final_model = program.run().await?;

    println!("Final event count: {}", final_model.events_received);
    if final_model.last_event_id > 0 {
        println!("Last event ID: {}", final_model.last_event_id);
    }

    Ok(())
}
