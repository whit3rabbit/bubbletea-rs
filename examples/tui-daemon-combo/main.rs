//! TUI Daemon Combo Example
//!
//! A program that can run either as a TUI application or as a daemon,
//! matching the Go Bubble Tea tui-daemon-combo example.
//!
//! Features:
//! - Dual mode: Interactive TUI mode or headless daemon mode
//! - TTY detection to automatically choose the appropriate mode
//! - Command-line flags: -d for daemon mode, -h for help
//! - Spinner animation with work simulation
//! - Rolling buffer of last 5 completed tasks with emojis
//! - Proper logging: stderr in daemon mode, discarded in TUI mode

use bubbletea_rs::{batch, quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use clap::{Arg, Command as ClapCommand};
use lipgloss_extras::lipgloss::{Color, Style};
use log::{info, warn};
use rand::Rng;
use std::io;
use std::os::unix::io::AsRawFd;
use std::time::Duration;

/// Message sent when a pretend process completes
#[derive(Debug)]
pub struct ProcessFinishedMsg {
    duration: Duration,
    emoji: String,
}

/// Message for spinner animation ticks
#[derive(Debug)]
pub struct SpinnerTickMsg;

/// A completed work result
#[derive(Debug, Clone)]
pub struct WorkResult {
    duration: Duration,
    emoji: String,
}

impl Default for WorkResult {
    fn default() -> Self {
        Self {
            duration: Duration::ZERO,
            emoji: String::new(),
        }
    }
}

/// The application model
#[derive(Debug)]
pub struct TuiDaemonModel {
    spinner_frame: usize,
    results: Vec<WorkResult>,
    quitting: bool,
}

impl TuiDaemonModel {
    fn new() -> Self {
        const SHOW_LAST_RESULTS: usize = 5;
        Self {
            spinner_frame: 0,
            results: vec![WorkResult::default(); SHOW_LAST_RESULTS],
            quitting: false,
        }
    }

    /// Get spinner frames (matching Go version)
    fn spinner_frames() -> &'static [&'static str] {
        &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
    }

    /// Get current spinner frame with pink styling
    fn current_spinner(&self) -> String {
        let frames = Self::spinner_frames();
        let frame = frames[self.spinner_frame % frames.len()];
        let style = Style::new().foreground(Color::from("206")); // Pink color like Go version
        style.render(frame)
    }

    /// Advance spinner to next frame
    fn advance_spinner(&mut self) {
        let frames = Self::spinner_frames();
        self.spinner_frame = (self.spinner_frame + 1) % frames.len();
    }

    /// Add a new result to the rolling buffer
    fn add_result(&mut self, result: WorkResult) {
        // Shift results left and add new one at the end (like Go version)
        for i in 1..self.results.len() {
            self.results[i - 1] = self.results[i].clone();
        }
        if let Some(last) = self.results.last_mut() {
            *last = result;
        }
    }
}

impl Model for TuiDaemonModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = TuiDaemonModel::new();

        info!("Starting work...");

        // Start both spinner and work process
        let cmds = vec![
            tick(Duration::from_millis(100), |_| {
                Box::new(SpinnerTickMsg) as Msg
            }),
            run_pretend_process(),
        ];

        (model, Some(batch(cmds)))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle key messages - quit on any key
        if msg.downcast_ref::<KeyMsg>().is_some() {
            self.quitting = true;
            return Some(quit());
        }

        // Handle spinner tick
        if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
            self.advance_spinner();
            return Some(tick(Duration::from_millis(100), |_| {
                Box::new(SpinnerTickMsg) as Msg
            }));
        }

        // Handle completed work
        if let Some(finished_msg) = msg.downcast_ref::<ProcessFinishedMsg>() {
            let result = WorkResult {
                duration: finished_msg.duration,
                emoji: finished_msg.emoji.clone(),
            };

            info!(
                "{} Job finished in {}ms",
                result.emoji,
                result.duration.as_millis()
            );

            self.add_result(result);
            return Some(run_pretend_process());
        }

        None
    }

    fn view(&self) -> String {
        let mut s = format!("\n{} Doing some work...\n\n", self.current_spinner());

        // Show results
        for result in &self.results {
            if result.duration == Duration::ZERO {
                s.push_str("........................\n");
            } else {
                s.push_str(&format!(
                    "{} Job finished in {}ms\n",
                    result.emoji,
                    result.duration.as_millis()
                ));
            }
        }

        // Help text
        let help_style = Style::new().foreground(Color::from("241"));
        s.push_str(&help_style.render("\nPress any key to exit\n"));

        if self.quitting {
            s.push('\n');
        }

        // Apply main style with left margin
        let main_style = Style::new().margin_left(1);
        main_style.render(&s)
    }
}

/// Simulate a pretend process that takes random time
fn run_pretend_process() -> Cmd {
    Box::pin(async {
        // Generate random values before the async block to avoid Send issues
        let pause_ms = {
            let mut rng = rand::thread_rng();
            rng.gen_range(100..=999) // 100-999ms like Go version
        };
        let emoji = random_emoji();
        let pause = Duration::from_millis(pause_ms);

        tokio::time::sleep(pause).await;

        Some(Box::new(ProcessFinishedMsg {
            duration: pause,
            emoji,
        }) as Msg)
    })
}

/// Get a random emoji from the selection
fn random_emoji() -> String {
    let emojis = [
        "🍦", "🧋", "🍡", "🤠", "👾", "😭", "🦊", "🐯", "🦆", "🥨", "🎏", "🍔", "🍒", "🍥", "🎮",
        "📦", "🦁", "🐶", "🐸", "🍕", "🥐", "🧲", "🚒", "🥇", "🏆", "🌽",
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..emojis.len());
    emojis[index].to_string()
}

/// Check if stdout is a TTY using libc::isatty
fn is_tty() -> bool {
    let stdout_fd = io::stdout().as_raw_fd();
    unsafe { libc::isatty(stdout_fd) == 1 }
}

/// Configure logging based on mode
fn setup_logging(daemon_mode: bool) {
    if daemon_mode {
        // In daemon mode, log to stderr
        env_logger::Builder::from_default_env()
            .target(env_logger::Target::Stderr)
            .init();
    } else {
        // In TUI mode, discard log output
        env_logger::Builder::from_default_env()
            .target(env_logger::Target::Pipe(Box::new(io::sink())))
            .init();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = ClapCommand::new("tui-daemon-combo")
        .version("0.1.0")
        .about("A TUI application that can also run as a daemon")
        .arg(
            Arg::new("daemon")
                .short('d')
                .long("daemon")
                .action(clap::ArgAction::SetTrue)
                .help("Run as a daemon"),
        )
        .get_matches();

    let daemon_mode = matches.get_flag("daemon");
    let force_daemon = daemon_mode || !is_tty();

    // Setup logging
    setup_logging(force_daemon);

    // Configure program based on mode
    let mut builder = Program::<TuiDaemonModel>::builder().signal_handler(true);

    if force_daemon {
        // If we're in daemon mode or not a TTY, don't render the TUI
        builder = builder.without_renderer();
    }

    let program = builder.build()?;

    // Run the program
    if let Err(err) = program.run().await {
        if force_daemon {
            warn!("Error running program: {}", err);
        } else {
            eprintln!("Error starting Bubble Tea program: {}", err);
        }
        std::process::exit(1);
    }

    Ok(())
}
