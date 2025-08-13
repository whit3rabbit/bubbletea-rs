//! Realtime Example
//!
//! A simple example that shows how to send activity to Bubble Tea in real-time
//! through commands. This is a faithful port of the Go Bubble Tea realtime example.
//!
//! Demonstrates:
//! - Real-time activity simulation with random intervals
//! - Spinner animation using bubbletea-widgets
//! - Event counting and display
//! - Command batching for concurrent operations
//! - Proper key handling (any key quits)

use bubbletea_rs::{batch, quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use std::time::Duration;

/// A message used to indicate that activity has occurred
#[derive(Debug)]
pub struct ResponseMsg;

/// Message for spinner animation ticks  
#[derive(Debug)]
pub struct SpinnerTickMsg;

/// Simulate a process that sends events at an irregular interval in real time.
/// In this case, we'll send events at a random interval between 100 to 1000 milliseconds.
/// As a command, Bubble Tea will run this asynchronously.
fn listen_for_activity() -> Cmd {
    Box::pin(async move {
        use rand::Rng;
        loop {
            let delay_ms = rand::rng().random_range(100..=1000);
            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            // In a real application, this would send to a channel
            // For this example, we'll just return immediately
            return Some(Box::new(ResponseMsg) as Msg);
        }
    })
}

/// A command that waits for the next activity event
fn wait_for_activity() -> Cmd {
    listen_for_activity()
}

/// The application model
#[derive(Debug)]
pub struct RealtimeModel {
    spinner_frame: usize,
    responses: u32,
    quitting: bool,
}

impl RealtimeModel {
    pub fn new() -> Self {
        Self {
            spinner_frame: 0,
            responses: 0,
            quitting: false,
        }
    }

    /// Get the current spinner frame (matching Go bubble tea spinner widget)
    pub fn spinner_view(&self) -> String {
        // Use DOT spinner frames
        let frames = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let frame = frames[self.spinner_frame % frames.len()];
        
        // Apply no styling to match the Go version exactly
        frame.to_string()
    }

    /// Advance to the next spinner frame
    pub fn advance_spinner(&mut self) {
        let frames = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        self.spinner_frame = (self.spinner_frame + 1) % frames.len();
    }
}

impl Model for RealtimeModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = RealtimeModel::new();

        // Start spinner animation, activity generation, and waiting for activity (matching Go's tea.Batch)
        let spinner_tick = tick(Duration::from_millis(100), |_| Box::new(SpinnerTickMsg) as Msg);
        let cmd = batch(vec![
            spinner_tick,          // Start spinner animation
            listen_for_activity(), // Generate activity
            wait_for_activity(),   // Wait for activity
        ]);

        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle any key press - quit immediately (matching Go behavior)
        if msg.downcast_ref::<KeyMsg>().is_some() {
            self.quitting = true;
            return Some(quit());
        }

        // Handle response messages (activity events)
        if msg.downcast_ref::<ResponseMsg>().is_some() {
            self.responses += 1; // Record external activity
            return Some(wait_for_activity()); // Wait for next event
        }

        // Handle spinner tick messages
        if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
            if !self.quitting {
                self.advance_spinner();
                return Some(tick(Duration::from_millis(100), |_| Box::new(SpinnerTickMsg) as Msg));
            }
        }

        None
    }

    fn view(&self) -> String {
        let mut s = format!(
            "\n {} Events received: {}\n\n Press any key to exit\n",
            self.spinner_view(),
            self.responses
        );

        if self.quitting {
            s.push('\n');
        }

        s
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<RealtimeModel>::builder().build()?;

    program.run().await?;

    Ok(())
}
