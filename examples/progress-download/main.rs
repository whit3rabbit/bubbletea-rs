//! Progress Download Example
//!
//! Demonstrates:
//! - Real HTTP file downloads with animated progress bars
//! - Background download tasks with progress reporting
//! - Error handling during download process
//! - Real-time percentage display with smooth animations
//! - Window resize handling for progress bar sizing
//! - Completion detection with auto-quit after brief pause
//! - Command-line argument parsing for download URLs
//!
//! This example downloads actual files from HTTP URLs and displays
//! real-time progress with an animated progress bar. It demonstrates
//! how to integrate progress tracking with background async operations
//! in Bubble Tea applications.
//!
//! This is a faithful port of the Go Bubble Tea progress-download example
//! with identical behavior, UI, and command-line interface.
//!
//! Usage: cargo run -- --url https://example.com/file.zip

use bubbletea_rs::gradient::gradient_filled_segment;
use bubbletea_rs::{batch, quit, sequence, tick, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
use clap::Parser;
use futures_util::StreamExt;
use lipgloss_extras::lipgloss::{Color, Style};
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(name = "progress-download")]
#[command(about = "Download a file with an animated progress bar")]
struct Args {
    /// URL of the file to download
    #[arg(long)]
    url: String,
}

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

/// Message for processing progress channel
#[derive(Debug)]
pub struct ProcessChannelMsg;

/// Progress writer that handles the actual download and progress reporting
pub struct ProgressWriter {
    total: u64,
    downloaded: u64,
    file: File,
    progress_sender: mpsc::UnboundedSender<Msg>,
}

impl ProgressWriter {
    pub fn new(total: u64, file: File, progress_sender: mpsc::UnboundedSender<Msg>) -> Self {
        Self {
            total,
            downloaded: 0,
            file,
            progress_sender,
        }
    }

    /// Start the download process
    pub async fn start(
        &mut self,
        response: reqwest::Response,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    self.file.write_all(&chunk).await?;
                    self.downloaded += chunk.len() as u64;

                    if self.total > 0 {
                        let progress = self.downloaded as f64 / self.total as f64;
                        let _ = self
                            .progress_sender
                            .send(Box::new(ProgressMsg(progress)) as Msg);
                    }
                }
                Err(e) => {
                    let _ = self.progress_sender.send(Box::new(ProgressErrMsg {
                        error: format!("Download error: {}", e),
                    }) as Msg);
                    return Err(Box::new(e));
                }
            }
        }

        Ok(())
    }
}

/// Animated progress bar with gradient (matching progress-animated example)
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
            animation_speed: 0.15,
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

/// Static channel for communicating with the model
static PROGRESS_CHANNEL: std::sync::OnceLock<mpsc::UnboundedSender<Msg>> =
    std::sync::OnceLock::new();

/// The application state
#[derive(Debug)]
pub struct ProgressDownloadModel {
    pub progress: AnimatedProgressBar,
    pub error: Option<String>,
    pub progress_receiver: mpsc::UnboundedReceiver<Msg>,
}

impl ProgressDownloadModel {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let _ = PROGRESS_CHANNEL.set(sender);

        Self {
            progress: AnimatedProgressBar::new(),
            error: None,
            progress_receiver: receiver,
        }
    }

    pub fn update_window_size(&mut self, width: u16, _height: u16) {
        // Match Go behavior: width - padding*2 - 4, max 80
        const PADDING: u16 = 2;
        const MAX_WIDTH: usize = 80;

        let available_width = width.saturating_sub(PADDING * 2).saturating_sub(4) as usize;
        self.progress.width = available_width.min(MAX_WIDTH);
    }

    /// Create final pause command before quitting (matching Go's 750ms)
    fn final_pause() -> Cmd {
        tick(Duration::from_millis(750), |_| {
            Box::new(FinalPauseMsg) as Msg
        })
    }
}

impl Model for ProgressDownloadModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = Self::new();
        // Start checking for progress messages
        let channel_cmd = tick(Duration::from_millis(10), |_| {
            Box::new(ProcessChannelMsg) as Msg
        });
        (model, Some(channel_cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle channel message processing
        if msg.downcast_ref::<ProcessChannelMsg>().is_some() {
            // Try to receive a message from the channel
            match self.progress_receiver.try_recv() {
                Ok(progress_msg) => {
                    // Process the received message and continue polling
                    let next_poll = tick(Duration::from_millis(10), |_| {
                        Box::new(ProcessChannelMsg) as Msg
                    });

                    // Forward the progress message to ourselves and continue polling
                    return Some(batch(vec![
                        Box::pin(async move { Some(progress_msg) }),
                        next_poll,
                    ]));
                }
                Err(mpsc::error::TryRecvError::Empty) => {
                    // No message available, continue polling
                    return Some(tick(Duration::from_millis(10), |_| {
                        Box::new(ProcessChannelMsg) as Msg
                    }));
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    // Channel closed, stop polling
                    return None;
                }
            }
        }
        // Handle progress updates
        if let Some(progress_msg) = msg.downcast_ref::<ProgressMsg>() {
            let mut cmds = Vec::new();

            // If download is complete, add final pause and quit using sequence
            if progress_msg.0 >= 1.0 {
                // Use sequence to ensure final pause happens before quit (matching Go)
                return Some(sequence(vec![Self::final_pause(), quit()]));
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
            return None; // Just pause, quit will come from the sequence command
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
        const PADDING: &str = "  "; // 2 spaces padding (matching Go)

        // Show error if there was one (matching Go format)
        if let Some(error) = &self.error {
            return format!("Error downloading: {}\n", error);
        }

        // Style help text with gray color (matching Go's lipgloss style #626262)
        let help_style = Style::new().foreground(Color::from("#626262"));
        let help_text = help_style.render("Press any key to quit");

        // Match Go's view format exactly
        format!(
            "\n{}{}\n\n{}{}",
            PADDING,
            self.progress.view(),
            PADDING,
            help_text
        )
    }
}

async fn get_response(url: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(format!("receiving status of {} for url: {}", response.status(), url).into());
    }

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.url.is_empty() {
        eprintln!("URL is required");
        std::process::exit(1);
    }

    // Get the response to check content length
    let response = match get_response(&args.url).await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("could not get response: {}", e);
            std::process::exit(1);
        }
    };

    // Don't add TUI if the header doesn't include content size
    // it's impossible to see progress without total (matching Go behavior)
    let content_length = match response.content_length() {
        Some(len) if len > 0 => len,
        _ => {
            eprintln!("can't parse content length, aborting download");
            std::process::exit(1);
        }
    };

    // Extract filename from URL
    let filename = Path::new(&args.url)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("downloaded_file");

    // Create the output file
    let file = match File::create(filename).await {
        Ok(f) => f,
        Err(e) => {
            eprintln!("could not create file: {}", e);
            std::process::exit(1);
        }
    };

    // Start the program first to initialize the channel
    let program = Program::<ProgressDownloadModel>::builder().build()?;

    // Get the progress sender from the static channel
    let progress_sender = PROGRESS_CHANNEL
        .get()
        .expect("Progress channel should be initialized")
        .clone();

    // Create progress writer and start download in background
    let mut progress_writer = ProgressWriter::new(content_length, file, progress_sender);

    // Spawn the download task
    tokio::spawn(async move {
        if let Err(e) = progress_writer.start(response).await {
            eprintln!("Download failed: {}", e);
        }
    });

    // Run the program
    program.run().await?;

    Ok(())
}
