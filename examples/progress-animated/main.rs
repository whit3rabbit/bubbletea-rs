//! Progress Animated Example
//!
//! Demonstrates:
//! - Animated progress bar with gradient colors using bubbletea-widgets styling
//! - Progress increments by 25% every second with smooth animation
//! - Window resize handling for progress bar sizing  
//! - Built-in progress bar animation system with frame messages
//! - Automatic completion and exit when reaching 100%
//! - Proper key binding management using bubbletea-widgets::key
//!
//! This example shows how to integrate bubbletea-widgets with custom progress
//! animation, creating visually appealing progress indicators that match
//! the Go Bubble Tea version's behavior and visual style.
//!
//! This is a faithful port of the Go Bubble Tea progress-animated example,
//! modernized to use bubbletea-widgets for key handling while maintaining
//! the custom animated progress implementation for precise control.

use bubbletea_rs::gradient::gradient_filled_segment;
use bubbletea_rs::{batch, quit, tick, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};
use std::sync::OnceLock;
use std::time::Duration;

static DEBUG_ENABLED: OnceLock<bool> = OnceLock::new();
fn debug_enabled() -> bool {
    *DEBUG_ENABLED.get_or_init(|| std::env::var("BT_DEBUG").is_ok())
}
macro_rules! dlog {
    ($($arg:tt)*) => {
        if debug_enabled() { eprintln!($($arg)*); }
    }
}

/// Message for progress tick updates
#[derive(Debug)]
pub struct ProgressTickMsg;

/// Message for progress bar animation frames
#[derive(Debug)]
pub struct ProgressFrameMsg;

/// Animated progress bar with smooth transitions using bubbletea-widgets styling
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
            animation_speed: 0.1, // Animation step size
        }
    }

    /// Set target percentage for animation (matching bubbletea-widgets progress API)
    pub fn set_percent(&mut self, percent: f64) -> Option<Cmd> {
        let old_target = self.target_percent;
        self.target_percent = percent.clamp(0.0, 1.0);

        dlog!(
            "set_percent: {:.3} -> {:.3}, current={:.3}",
            old_target,
            self.target_percent,
            self.current_percent
        );

        // If we need to animate, start frame messages
        if (self.current_percent - self.target_percent).abs() > 0.001 {
            dlog!(
                "set_percent: starting animation (diff={:.3})",
                (self.current_percent - self.target_percent).abs()
            );
            Some(tick(Duration::from_millis(16), |_| {
                Box::new(ProgressFrameMsg) as Msg
            })) // ~60fps
        } else {
            dlog!(
                "set_percent: no animation needed (diff={:.3})",
                (self.current_percent - self.target_percent).abs()
            );
            None
        }
    }

    /// Increment target percentage by amount (matching bubbletea-widgets progress API)
    pub fn incr_percent(&mut self, amount: f64) -> Option<Cmd> {
        self.set_percent(self.target_percent + amount)
    }

    /// Update animation frame
    pub fn update_animation(&mut self) -> Option<Cmd> {
        const MIN_STEP: f64 = 0.005; // Minimum step per frame (0.5%)
        const TOLERANCE: f64 = 0.0001; // When to snap to target

        let diff = self.target_percent - self.current_percent;

        if diff.abs() > TOLERANCE {
            // Calculate step: use larger of exponential decay or minimum step
            let exponential_step = diff * self.animation_speed;
            let step = if exponential_step.abs() >= MIN_STEP {
                exponential_step
            } else {
                // Use minimum step with correct sign when exponential becomes too small
                if diff > 0.0 {
                    MIN_STEP
                } else {
                    -MIN_STEP
                }
            };

            // Check if this step would overshoot the target
            if (self.current_percent + step - self.target_percent).abs() < TOLERANCE
                || (diff > 0.0 && step >= diff)
                || (diff < 0.0 && step <= diff)
            {
                // Snap to target to avoid overshoot
                self.current_percent = self.target_percent;
                dlog!("animation: snapped to target {:.3}", self.target_percent);
                None // Animation complete
            } else {
                // Apply step and continue animation
                self.current_percent += step;
                dlog!(
                    "animation: step {:.4}, now at {:.3}, target {:.3}",
                    step,
                    self.current_percent,
                    self.target_percent
                );
                // Always schedule next frame if not at target
                Some(tick(Duration::from_millis(16), |_| {
                    Box::new(ProgressFrameMsg) as Msg
                }))
            }
        } else {
            // Already at target within tolerance
            self.current_percent = self.target_percent;
            dlog!("animation: already at target {:.3}", self.target_percent);
            None // No animation needed
        }
    }

    /// Get current animated percentage (matching bubbletea-widgets progress API)
    pub fn percent(&self) -> f64 {
        self.current_percent
    }

    /// Render animated progress bar with gradient colors (matching Go version)
    pub fn view(&self) -> String {
        let percent = self.current_percent.clamp(0.0, 1.0);
        let filled_width = (self.width as f64 * percent).round() as usize;
        let empty_width = self.width.saturating_sub(filled_width);

        // Use bubbletea-rs gradient colors (matching Go's default gradient)
        let filled_str = gradient_filled_segment(filled_width, self.filled_char);

        let empty_str = self.empty_char.to_string().repeat(empty_width);
        let bar = format!("{}{}", filled_str, empty_str);

        let line = format!("{} {:5.1}%", bar, percent * 100.0);
        if debug_enabled() {
            format!(
                "{}\n[dbg] cur={:.3} tgt={:.3}",
                line, self.current_percent, self.target_percent
            )
        } else {
            line
        }
    }
}

/// The application state - using bubbletea-widgets key bindings
#[derive(Debug)]
pub struct ProgressAnimatedModel {
    pub progress: AnimatedProgressBar,
    pub quit_key_binding: Binding,
}

impl ProgressAnimatedModel {
    pub fn new() -> Self {
        // Set up proper key bindings using bubbletea-widgets
        let quit_key_binding = new_binding(vec![
            with_keys_str(&["q", "esc", "ctrl+c"]),
            with_help("any key", "quit"),
        ]);

        Self {
            progress: AnimatedProgressBar::new(),
            quit_key_binding,
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

impl Model for ProgressAnimatedModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = ProgressAnimatedModel::new();
        dlog!("init: starting 1s tick");
        // Start the progress updates (matching Go's tickCmd)
        let cmd = tick(Duration::from_secs(1), |_| Box::new(ProgressTickMsg) as Msg);
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle progress tick messages
        if msg.downcast_ref::<ProgressTickMsg>().is_some() {
            dlog!(
                "tick: current {:.3}, target {:.3}",
                self.progress.current_percent,
                self.progress.target_percent
            );

            // Check if target is already at 100% (matching Go's behavior exactly)
            if self.progress.target_percent >= 1.0 {
                dlog!("tick: target at 100%, quitting");
                return Some(quit()); // Auto-quit when complete
            }

            // Increment by 25% with animation (matching Go behavior)
            let old_target = self.progress.target_percent;
            let animation_cmd = self.progress.incr_percent(0.25);
            let new_target = self.progress.target_percent;

            dlog!("tick: target {:.3} -> {:.3}", old_target, new_target);

            // ALWAYS batch next tick with animation command (matching Go's tea.Batch)
            // The quit check happens at the beginning of the NEXT tick
            let next_tick = tick(Duration::from_secs(1), |_| Box::new(ProgressTickMsg) as Msg);
            match animation_cmd {
                Some(anim_cmd) => Some(batch(vec![next_tick, anim_cmd])),
                None => Some(next_tick),
            }
        }
        // Handle animation frame messages
        else if msg.downcast_ref::<ProgressFrameMsg>().is_some() {
            let before = self.progress.current_percent;
            let cmd = self.progress.update_animation();
            dlog!(
                "frame: {:.3} -> {:.3}, next_cmd? {}",
                before,
                self.progress.current_percent,
                cmd.is_some()
            );

            // Check for completion: only quit when current actually reaches 100%
            if self.progress.current_percent >= 1.0 {
                dlog!(
                    "frame: current reached 100% ({:.3}), quitting",
                    self.progress.current_percent
                );
                return Some(quit());
            }

            cmd
        }
        // Handle window size changes
        else if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            self.update_window_size(size_msg.width, size_msg.height);
            None
        }
        // Handle keyboard input - ANY key quits (matching Go behavior)
        else if msg.downcast_ref::<KeyMsg>().is_some() {
            Some(quit())
        } else {
            None
        }
    }

    fn view(&self) -> String {
        const PADDING: &str = "  "; // 2 spaces padding

        format!(
            "\n{}{}\n\n{}Press any key to quit",
            PADDING,
            self.progress.view(),
            PADDING
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the program (matching Go main function behavior)
    let program = Program::<ProgressAnimatedModel>::builder().build()?;

    // Run the program
    program.run().await?;

    Ok(())
}