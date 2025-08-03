//! Progress Static Example
//!
//! Demonstrates:
//! - Static progress bar rendering with percentage display
//! - Progress increments by 25% every second until completion
//! - Window resize handling for progress bar sizing
//! - Simple progress visualization without animation
//! - Automatic completion and exit when reaching 100%
//!
//! This example shows a basic progress bar that updates at regular intervals,
//! demonstrating how to create simple progress indicators for batch operations,
//! file transfers, or other time-based tasks.
//!
//! This is a faithful port of the Go Bubble Tea progress-static example,
//! maintaining the same behavior: increment by 25% every second, quit on
//! any key press, and automatically quit when reaching 100%.

use bubbletea_rs::gradient::gradient_filled_segment;
use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
use std::time::Duration;

/// Message for progress tick updates
#[derive(Debug)]
pub struct ProgressTickMsg;

/// Progress bar configuration matching Go example behavior
#[derive(Debug)]
pub struct ProgressBar {
    pub width: usize,
    pub filled_char: char,
    pub empty_char: char,
}

impl ProgressBar {
    pub fn new() -> Self {
        Self {
            width: 40,
            filled_char: '█',
            empty_char: '░',
        }
    }

    /// Render progress bar with percentage (matching Go's ViewAs method)
    pub fn view_as(&self, percent: f64) -> String {
        let percent = percent.clamp(0.0, 1.0);
        let filled_width = (self.width as f64 * percent).round() as usize;
        let empty_width = self.width.saturating_sub(filled_width);

        // Build filled portion with a horizontal color gradient (Charm default)
        let filled_str = gradient_filled_segment(filled_width, self.filled_char);

        let empty_str = self.empty_char.to_string().repeat(empty_width);
        let bar = format!("{}{}", filled_str, empty_str);

        format!("{} {:>3.0}%", bar, percent * 100.0)
    }
}

/// The application state - matching the Go model struct
#[derive(Debug)]
pub struct ProgressStaticModel {
    pub percent: f64,
    pub progress: ProgressBar,
}

impl ProgressStaticModel {
    pub fn new() -> Self {
        Self {
            percent: 0.0,
            progress: ProgressBar::new(),
        }
    }

    pub fn update_window_size(&mut self, width: u16, _height: u16) {
        // Match Go behavior: width - padding*2 - 4, max 80
        const PADDING: u16 = 2;
        const MAX_WIDTH: usize = 80;

        let available_width = width.saturating_sub(PADDING * 2).saturating_sub(4) as usize;
        self.progress.width = available_width.min(MAX_WIDTH);
    }
}

impl Model for ProgressStaticModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = ProgressStaticModel::new();

        // Start the progress updates (matching Go's tickCmd)
        let cmd = tick(Duration::from_secs(1), |_| Box::new(ProgressTickMsg) as Msg);
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle progress tick messages
        if msg.downcast_ref::<ProgressTickMsg>().is_some() {
            // Increment by 25% (matching Go behavior)
            self.percent += 0.25;
            if self.percent >= 1.0 {
                self.percent = 1.0;
                return Some(quit()); // Auto-quit when complete
            }
            // Schedule next progress update
            return Some(tick(Duration::from_secs(1), |_| {
                Box::new(ProgressTickMsg) as Msg
            }));
        }

        // Handle window size changes
        if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            self.update_window_size(size_msg.width, size_msg.height);
            return None;
        }

        // Handle keyboard input - ANY key quits (matching Go behavior)
        if msg.downcast_ref::<KeyMsg>().is_some() {
            return Some(quit());
        }

        None
    }

    fn view(&self) -> String {
        const PADDING: &str = "  "; // 2 spaces padding

        format!(
            "\n{}{}\n\n{}Press any key to quit",
            PADDING,
            self.progress.view_as(self.percent),
            PADDING
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the program (matching Go main function behavior)
    let program = Program::<ProgressStaticModel>::builder().build()?;

    // Run the program
    program.run().await?;

    Ok(())
}
