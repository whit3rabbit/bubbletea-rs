//! Progress Download Example
//!
//! Demonstrates:
//! - Animated progress bar with download simulation
//! - Progress updates from background async tasks
//! - Error handling during download process
//! - Real-time percentage display with smooth animations
//! - Window resize handling for progress bar sizing
//! - Completion detection with auto-quit after brief pause
//! - External message sending from background tasks
//!
//! This example simulates downloading a file with realistic download
//! patterns including variable speed, network delays, and potential
//! errors. It demonstrates how to integrate progress tracking with
//! background async operations in Bubble Tea applications.
//!
//! This is a faithful port of the Go Bubble Tea progress-download example,
//! but uses simulated downloads instead of real HTTP requests for
//! demonstration purposes and better testability.

use bubbletea_rs::gradient::gradient_filled_segment;
use bubbletea_rs::{batch, quit, tick, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
use rand::Rng;
use std::time::Duration;

/// Message containing progress update (0.0 to 1.0)
#[derive(Debug, Clone)]
pub struct ProgressMsg(pub f64);

/// Message for progress bar animation frames
#[derive(Debug)]
pub struct ProgressFrameMsg;

/// Message indicating download error
#[derive(Debug)]
pub struct ProgressErrMsg {
    pub error: String,
}

/// Message for final pause before quit
#[derive(Debug)]
pub struct FinalPauseMsg;

/// Message for download tick updates
#[derive(Debug)]
pub struct DownloadTickMsg;

/// Simulated download manager
#[derive(Debug)]
pub struct DownloadSimulator {
    pub file_name: String,
    pub total_size: u64,
    pub downloaded: u64,
    pub is_complete: bool,
    pub has_error: Option<String>,
    pub download_speed: u64, // bytes per tick
    pub error_chance: u32,   // 0-100, chance of error per tick
}

impl DownloadSimulator {
    pub fn new(file_name: String, size_mb: u64) -> Self {
        Self {
            file_name,
            total_size: size_mb * 1024 * 1024, // Convert MB to bytes
            downloaded: 0,
            is_complete: false,
            has_error: None,
            download_speed: 50 * 1024, // 50KB per tick (roughly 500KB/s at 100ms ticks)
            error_chance: 1,           // 1% chance of error per tick
        }
    }

    /// Get current progress ratio (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.total_size == 0 {
            0.0
        } else {
            (self.downloaded as f64 / self.total_size as f64).min(1.0)
        }
    }

    /// Simulate download progress for one tick
    pub fn tick(&mut self) -> Result<bool, String> {
        if self.is_complete || self.has_error.is_some() {
            return Ok(self.is_complete);
        }

        let mut rng = rand::thread_rng();

        // Simulate occasional network errors
        if rng.gen_range(0..100) < self.error_chance {
            let error = "Network timeout - connection lost".to_string();
            self.has_error = Some(error.clone());
            return Err(error);
        }

        // Simulate variable download speed (50% to 150% of base speed)
        let speed_variation = rng.gen_range(0.5..1.5);
        let chunk_size = (self.download_speed as f64 * speed_variation) as u64;

        let remaining = self.total_size - self.downloaded;
        let actual_chunk = chunk_size.min(remaining);

        self.downloaded += actual_chunk;

        if self.downloaded >= self.total_size {
            self.downloaded = self.total_size;
            self.is_complete = true;
        }

        Ok(self.is_complete)
    }
}

/// Animated progress bar with gradient
#[derive(Debug)]
pub struct AnimatedProgressBar {
    pub width: usize,
    pub current_percent: f64,
    pub target_percent: f64,
    pub filled_char: char,
    pub empty_char: char,
    pub animation_speed: f64,
}

impl AnimatedProgressBar {
    pub fn new() -> Self {
        Self {
            width: 40,
            current_percent: 0.0,
            target_percent: 0.0,
            filled_char: '█',
            empty_char: '░',
            animation_speed: 0.15, // Slightly faster for download feedback
        }
    }

    /// Set target percentage for animation
    pub fn set_percent(&mut self, percent: f64) -> Option<Cmd> {
        self.target_percent = percent.clamp(0.0, 1.0);

        // If we need to animate, start frame messages
        if (self.current_percent - self.target_percent).abs() > 0.001 {
            Some(tick(Duration::from_millis(16), |_| {
                Box::new(ProgressFrameMsg) as Msg
            })) // ~60fps
        } else {
            None
        }
    }

    /// Update animation frame
    pub fn update_animation(&mut self) -> Option<Cmd> {
        if (self.current_percent - self.target_percent).abs() > 0.001 {
            let diff = self.target_percent - self.current_percent;
            let step = diff * self.animation_speed;

            if step.abs() < 0.001 {
                self.current_percent = self.target_percent;
                None // Animation complete
            } else {
                self.current_percent += step;
                // Continue animation
                Some(tick(Duration::from_millis(16), |_| {
                    Box::new(ProgressFrameMsg) as Msg
                }))
            }
        } else {
            None // No animation needed
        }
    }

    /// Get current animated percentage
    pub fn percent(&self) -> f64 {
        self.current_percent
    }

    /// Render animated progress bar with gradient
    pub fn view(&self) -> String {
        let percent = self.current_percent.clamp(0.0, 1.0);
        let filled_width = (self.width as f64 * percent).round() as usize;
        let empty_width = self.width.saturating_sub(filled_width);

        // Use gradient for filled portion
        let filled_str = gradient_filled_segment(filled_width, self.filled_char);
        let empty_str = self.empty_char.to_string().repeat(empty_width);
        let bar = format!("{}{}", filled_str, empty_str);

        format!("{} {:5.1}%", bar, percent * 100.0)
    }
}

/// The application state
#[derive(Debug)]
pub struct ProgressDownloadModel {
    pub downloader: DownloadSimulator,
    pub progress: AnimatedProgressBar,
    pub error: Option<String>,
}

impl ProgressDownloadModel {
    pub fn new(file_name: String, size_mb: u64) -> Self {
        Self {
            downloader: DownloadSimulator::new(file_name, size_mb),
            progress: AnimatedProgressBar::new(),
            error: None,
        }
    }

    pub fn update_window_size(&mut self, width: u16, _height: u16) {
        // Match Go behavior: width - padding*2 - 4, max 80
        const PADDING: u16 = 2;
        const MAX_WIDTH: usize = 80;

        let available_width = width.saturating_sub(PADDING * 2).saturating_sub(4) as usize;
        self.progress.width = available_width.min(MAX_WIDTH);
    }

    /// Create final pause command before quitting
    fn final_pause() -> Cmd {
        tick(Duration::from_millis(750), |_| {
            Box::new(FinalPauseMsg) as Msg
        })
    }
}

impl Model for ProgressDownloadModel {
    fn init() -> (Self, Option<Cmd>) {
        // For demo purposes, simulate downloading a 10MB file
        let model = ProgressDownloadModel::new("example-file.zip".to_string(), 10);

        // Start the download simulation with timer ticks
        let tick_cmd = tick(Duration::from_millis(100), |_| {
            Box::new(DownloadTickMsg) as Msg
        });
        (model, Some(tick_cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle download tick updates
        if msg.downcast_ref::<DownloadTickMsg>().is_some() {
            match self.downloader.tick() {
                Ok(is_complete) => {
                    let mut cmds = Vec::new();

                    // Update progress bar with animation
                    let progress = self.downloader.progress();
                    if let Some(progress_cmd) = self.progress.set_percent(progress) {
                        cmds.push(progress_cmd);
                    }

                    if is_complete {
                        // Add final pause and quit
                        cmds.push(Self::final_pause());
                        cmds.push(quit());
                    } else {
                        // Continue download simulation
                        cmds.push(tick(Duration::from_millis(100), |_| {
                            Box::new(DownloadTickMsg) as Msg
                        }));
                    }

                    return if cmds.is_empty() {
                        None
                    } else {
                        Some(batch(cmds))
                    };
                }
                Err(error) => {
                    self.error = Some(error);
                    return Some(quit());
                }
            }
        }

        // Handle progress updates (for manual testing)
        if let Some(progress_msg) = msg.downcast_ref::<ProgressMsg>() {
            let mut cmds = Vec::new();

            // If download is complete, add final pause and quit
            if progress_msg.0 >= 1.0 {
                cmds.push(Self::final_pause());
                cmds.push(quit());
            }

            // Update progress bar with animation
            if let Some(progress_cmd) = self.progress.set_percent(progress_msg.0) {
                cmds.push(progress_cmd);
            }

            return if cmds.is_empty() {
                None
            } else {
                Some(batch(cmds))
            };
        }

        // Handle animation frame messages
        if msg.downcast_ref::<ProgressFrameMsg>().is_some() {
            return self.progress.update_animation();
        }

        // Handle download errors
        if let Some(err_msg) = msg.downcast_ref::<ProgressErrMsg>() {
            self.error = Some(err_msg.error.clone());
            return Some(quit());
        }

        // Handle final pause before quit
        if msg.downcast_ref::<FinalPauseMsg>().is_some() {
            return None; // Just pause, quit will come from the batched command
        }

        // Handle window size changes
        if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            self.update_window_size(size_msg.width, size_msg.height);
            return None;
        }

        // Handle keyboard input - ANY key quits
        if msg.downcast_ref::<KeyMsg>().is_some() {
            return Some(quit());
        }

        None
    }

    fn view(&self) -> String {
        const PADDING: &str = "  "; // 2 spaces padding

        // Show error if there was one
        if let Some(error) = &self.error {
            return format!("\n{}Error downloading: {}\n", PADDING, error);
        }

        format!(
            "\n{}Downloading {}...\n{}{}\n\n{}Press any key to quit",
            PADDING,
            self.downloader.file_name,
            PADDING,
            self.progress.view(),
            PADDING
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the program
    let program = Program::<ProgressDownloadModel>::builder().build()?;

    // Run the program
    program.run().await?;

    Ok(())
}
