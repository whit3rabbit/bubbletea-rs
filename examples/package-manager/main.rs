//! Package Manager Example
//!
//! A package installer simulation demonstrating advanced bubbletea-rs patterns:
//!
//! ## Key Components Demonstrated:
//! - **Custom Spinner**: Hand-built spinner component with lipgloss styling
//! - **Animated Progress Bar**: Custom progress with gradient rendering using bubbletea-rs::gradient
//! - **Dynamic List Building**: Maintaining completed items in model state (not printf)
//! - **Complex Layout**: Width-aware text truncation and gap calculation
//! - **Multi-Command Coordination**: Using `batch()` for concurrent commands
//! - **Timed Simulations**: Using `tick()` for realistic delays
//!
//! ## bubbletea-rs Patterns:
//! - Model state management for UI lists
//! - Custom message types for app-specific events
//! - Combining multiple visual components in a single view
//! - Visual vs string length calculations for ANSI-styled text
//!
//! This is a faithful port of the Go Bubble Tea package-manager example
//! with identical behavior, UI styling, and animation.
//!
//! Usage: cargo run

// bubbletea-rs core imports for MVU pattern
use bubbletea_rs::gradient::gradient_filled_segment; // Built-in gradient helper for progress bars
use bubbletea_rs::{batch, quit, tick, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};

// crossterm for keyboard input handling
use crossterm::event::{KeyCode, KeyModifiers};

// lipgloss-extras for terminal styling (colors, etc.)
use lipgloss_extras::lipgloss::{Color, Style};

// Standard library imports for randomization and timing
use rand::prelude::SliceRandom;
use rand::Rng;
use std::time::Duration;

// =============================================================================
// CUSTOM MESSAGE TYPES
// =============================================================================
// In bubbletea-rs, you define custom message types to communicate between
// async commands and your model. Each message type represents a specific
// event that can occur in your application.

/// Message indicating a package has been installed
/// This is sent by the simulated download command when a package completes
#[derive(Debug)]
pub struct InstalledPkgMsg(pub String);

/// Message for spinner animation ticks
/// Sent periodically to advance the spinner frame
#[derive(Debug)]
pub struct SpinnerTickMsg;

/// Message for progress bar animation frames
/// Sent at ~60fps to create smooth progress bar animations
#[derive(Debug)]
pub struct ProgressFrameMsg;

// =============================================================================
// CUSTOM SPINNER COMPONENT
// =============================================================================
// This demonstrates how to build a reusable UI component in bubbletea-rs.
// The component manages its own state and provides methods for updating
// and rendering itself.

/// Animated spinner with pink styling (matching Go version #63)
///
/// ## bubbletea-rs Pattern: Custom Components
/// Instead of using a pre-built spinner, this shows how to create your own
/// reusable component with:
/// - Internal state management (current_frame)
/// - Styling with lipgloss-extras
/// - Animation timing with tick() commands
/// - Clean separation of concerns
#[derive(Debug)]
pub struct Spinner {
    current_frame: usize,
}

impl Spinner {
    pub fn new() -> Self {
        Self { current_frame: 0 }
    }

    /// Get the dot spinner frames (matching Go bubbles)
    /// These Unicode Braille patterns create a smooth spinning effect
    fn frames() -> &'static [&'static str] {
        &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
    }

    /// Get the spinner interval - 100ms for smooth animation
    fn interval() -> Duration {
        Duration::from_millis(100)
    }

    /// Get the current spinner frame with color #63 styling
    ///
    /// ## bubbletea-rs Pattern: Styled Rendering
    /// Uses lipgloss-extras to apply consistent color styling.
    /// The Style::render() method applies ANSI color codes while
    /// keeping the visual appearance separate from the data.
    pub fn view(&self) -> String {
        let frames = Self::frames();
        let frame = frames[self.current_frame % frames.len()];

        // Apply color #63 styling to match Go version exactly
        let style = Style::new().foreground(Color::from("63"));
        style.render(frame)
    }

    /// Advance to the next frame
    /// Called when SpinnerTickMsg is received
    pub fn advance_frame(&mut self) {
        let frames = Self::frames();
        self.current_frame = (self.current_frame + 1) % frames.len();
    }

    /// Create spinner tick command
    ///
    /// ## bubbletea-rs Pattern: Async Commands with tick()
    /// tick() creates a one-shot timer that sends a message after a delay.
    /// This is perfect for animations - each tick advances the frame and
    /// schedules the next tick, creating a smooth animation loop.
    pub fn tick_cmd() -> Cmd {
        tick(Self::interval(), |_| Box::new(SpinnerTickMsg) as Msg)
    }
}

// =============================================================================
// CUSTOM PROGRESS BAR COMPONENT
// =============================================================================

/// Animated progress bar with gradient (matching Go bubbles)
///
/// ## bubbletea-rs Pattern: Smooth Animations
/// This demonstrates smooth percentage animations using:
/// - target_percent vs current_percent for tweening
/// - 60fps frame updates with ProgressFrameMsg
/// - Built-in gradient rendering with gradient_filled_segment()
/// - Non-blocking animation that doesn't interfere with other updates
#[derive(Debug)]
pub struct Progress {
    width: usize,
    current_percent: f64, // Currently displayed percentage (animated)
    target_percent: f64,  // Target percentage (set immediately)
    animation_speed: f64, // How fast to animate between current and target
}

impl Progress {
    pub fn new() -> Self {
        Self {
            width: 40,
            current_percent: 0.0,
            target_percent: 0.0,
            animation_speed: 0.15,
        }
    }

    /// Set target percentage for animation
    ///
    /// ## bubbletea-rs Pattern: Conditional Commands
    /// This method demonstrates how to conditionally return commands based on state.
    /// If animation is needed, it returns a tick() command to start the animation loop.
    /// If no animation is needed, it returns None to avoid unnecessary work.
    pub fn set_percent(&mut self, percent: f64) -> Option<Cmd> {
        self.target_percent = percent.clamp(0.0, 1.0);

        // If we need to animate, start frame messages
        if (self.current_percent - self.target_percent).abs() > 0.001 {
            Some(tick(Duration::from_millis(16), |_| {
                Box::new(ProgressFrameMsg) as Msg
            })) // ~60fps for smooth animation
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

    /// Render progress bar with gradient (without percentage, matching Go)
    ///
    /// ## bubbletea-rs Pattern: Built-in Gradient Helper
    /// Uses bubbletea-rs::gradient::gradient_filled_segment() to create
    /// the same gradient colors as Charm's default (pink to yellow).
    /// This ensures visual consistency across different examples.
    pub fn view(&self) -> String {
        let percent = self.current_percent.clamp(0.0, 1.0);
        let filled_width = (self.width as f64 * percent).round() as usize;
        let empty_width = self.width.saturating_sub(filled_width);

        // Use gradient for filled portion (matching Go's WithDefaultGradient)
        let filled_str = gradient_filled_segment(filled_width, '█');
        let empty_str = '░'.to_string().repeat(empty_width);
        format!("{}{}", filled_str, empty_str)
    }
}

// =============================================================================
// MAIN APPLICATION MODEL
// =============================================================================

/// The application model - the core state of your TUI application
///
/// ## bubbletea-rs Pattern: State Management
/// The model holds ALL application state. This includes:
/// - Business logic state (packages, index, done)
/// - UI component state (spinner, progress)
/// - Display state (completed_packages for the list)
/// - Terminal state (width, height for responsive layout)
///
/// ## Key Pattern: completed_packages vs printf()
/// Unlike the Go version which uses tea.Printf() to print above the UI,
/// we maintain the completed list in model state. This is because bubbletea-rs's
/// printf() doesn't support the same cursor positioning as Go's version.
/// This approach is actually cleaner - all state lives in the model!
#[derive(Debug)]
pub struct PackageManagerModel {
    // Business logic state
    packages: Vec<String>, // All packages to install
    index: usize,          // Current package being installed
    done: bool,            // Whether all packages are complete

    // UI component state
    spinner: Spinner,   // Custom spinner component
    progress: Progress, // Custom progress bar component

    // Display state
    completed_packages: Vec<String>, // Track completed packages for display

    // Terminal state for responsive layout
    width: usize,  // Terminal width for layout calculations
    height: usize, // Terminal height (not used but available)
}

impl PackageManagerModel {
    pub fn new() -> Self {
        Self {
            packages: get_packages(),
            index: 0,
            width: 80,
            height: 24,
            spinner: Spinner::new(),
            progress: Progress::new(),
            done: false,
            completed_packages: Vec::new(),
        }
    }

    /// Create a download and install command for a package
    ///
    /// ## bubbletea-rs Pattern: Async Simulation with tick()
    /// This demonstrates how to simulate async work (downloading, file I/O, etc.)
    /// using tick() with random delays. The closure captures the package name
    /// and sends an InstalledPkgMsg when the "work" is complete.
    ///
    /// In a real app, this would be an actual async operation using tokio.
    fn download_and_install(pkg: String) -> Cmd {
        // Simulate download/install time with random delay (matching Go)
        let delay = Duration::from_millis(rand::thread_rng().gen_range(100..=600));
        tick(delay, move |_| {
            Box::new(InstalledPkgMsg(pkg.clone())) as Msg
        })
    }
}

// =============================================================================
// MODEL-VIEW-UPDATE (MVU) IMPLEMENTATION
// =============================================================================

impl Model for PackageManagerModel {
    /// Initialize the model and return initial commands
    ///
    /// ## bubbletea-rs Pattern: Initial Commands with batch()
    /// The init() method can return commands to run immediately.
    /// Here we use batch() to run multiple commands concurrently:
    /// - Start downloading the first package
    /// - Start the spinner animation
    /// This demonstrates how to kick off multiple async processes.
    fn init() -> (Self, Option<Cmd>) {
        let model = Self::new();

        // Start with the first package installation and spinner
        let install_cmd = Self::download_and_install(model.packages[model.index].clone());
        let spinner_cmd = Spinner::tick_cmd();

        // batch() runs commands concurrently, not sequentially
        (model, Some(batch(vec![install_cmd, spinner_cmd])))
    }

    /// Handle messages and update model state
    ///
    /// ## bubbletea-rs Pattern: Message Handling with downcast_ref()
    /// Since Msg is a trait object, we use downcast_ref() to check the
    /// concrete message type. This is similar to pattern matching in Go,
    /// but uses Rust's type system for safety.
    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle window size changes for responsive layout
        if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            self.width = size_msg.width as usize;
            self.height = size_msg.height as usize;
            return None;
        }

        // Handle keyboard input - quit on q, esc, or ctrl+c
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('q') | KeyCode::Esc => {
                    return Some(quit());
                }
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit());
                }
                _ => return None,
            }
        }

        // Handle package installation completion
        //
        // ## bubbletea-rs Pattern: State Updates + Command Coordination
        // When a package completes:
        // 1. Update model state (add to completed list, advance index)
        // 2. Conditionally return commands based on new state
        // 3. Use batch() to coordinate multiple follow-up actions
        if let Some(installed_msg) = msg.downcast_ref::<InstalledPkgMsg>() {
            let pkg = installed_msg.0.clone();

            // Add to completed packages list (this replaces printf in Go version)
            self.completed_packages.push(pkg.clone());

            if self.index >= self.packages.len() - 1 {
                // Everything's been installed. We're done!
                self.done = true;
                return Some(quit());
            }

            // Update progress bar and continue with next package
            self.index += 1;
            let mut cmds = Vec::new();

            // Update progress percentage (may trigger animation)
            if let Some(progress_cmd) = self
                .progress
                .set_percent(self.index as f64 / self.packages.len() as f64)
            {
                cmds.push(progress_cmd);
            }

            // Start next download
            cmds.push(Self::download_and_install(
                self.packages[self.index].clone(),
            ));

            // batch() ensures all commands run concurrently
            return Some(batch(cmds));
        }

        // Handle spinner tick messages
        //
        // ## bubbletea-rs Pattern: Animation Loops
        // For continuous animations, each tick advances the state and
        // schedules the next tick. This creates a self-sustaining loop.
        if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
            if !self.done {
                self.spinner.advance_frame();
                return Some(Spinner::tick_cmd()); // Schedule next frame
            }
        }

        // Handle progress bar animation frames
        //
        // ## bubbletea-rs Pattern: Conditional Animation
        // The progress bar only animates when needed (when current != target).
        // This saves CPU when no animation is required.
        if msg.downcast_ref::<ProgressFrameMsg>().is_some() {
            return self.progress.update_animation();
        }

        None
    }

    /// Render the current state to a string
    ///
    /// ## bubbletea-rs Pattern: Stateful View Rendering
    /// The view() method renders ALL UI state, including:
    /// - The completed packages list (maintained in model state)
    /// - The current installation line with all components
    /// - Responsive layout based on terminal width
    ///
    /// ## Key Difference from Go: No printf() needed
    /// Instead of printing messages above the UI, we render everything
    /// as one cohesive view. This makes the UI more predictable and easier
    /// to test since all rendering is in one place.
    fn view(&self) -> String {
        let n = self.packages.len();
        let w = format!("{}", n).len();

        if self.done {
            // Show completed packages list and done message
            let mut result = String::new();

            // Add all completed packages with checkmarks
            for pkg in &self.completed_packages {
                let check_mark = Style::new().foreground(Color::from("42")).render("✓");
                result.push_str(&format!("{} {}\n", check_mark, pkg));
            }

            // Add done message with margin (matching Go's doneStyle)
            result.push_str(&format!("\nDone! Installed {} packages.\n", n));
            return result;
        }

        // Build the view with completed packages list above current installation
        let mut result = String::new();

        // Add completed packages with checkmarks
        for pkg in &self.completed_packages {
            let check_mark = Style::new().foreground(Color::from("42")).render("✓");
            result.push_str(&format!("{} {}\n", check_mark, pkg));
        }

        // Package count format (matching Go's printf format)
        let pkg_count = format!(" {:width$}/{:width$}", self.index, n, width = w);

        let spin = format!("{} ", self.spinner.view());
        let prog = self.progress.view();

        // ## bubbletea-rs Pattern: Responsive Layout Calculations
        // When dealing with styled text (ANSI escape codes), you must separate:
        // - Visual width (what the user sees)
        // - String length (includes ANSI codes)
        // This calculation uses visual width for layout, string data for rendering.

        let fixed_width = 2 + self.progress.width + pkg_count.len(); // spinner + prog + count
        let available_width = if self.width > fixed_width + 15 {
            // ensure minimum space
            self.width - fixed_width
        } else {
            15 // minimum space for "Installing..."
        };

        // Current package name with color #211 (matching Go's currentPkgNameStyle)
        let pkg_name = &self.packages[self.index];
        let pkg_name_styled = Style::new().foreground(Color::from("211")).render(pkg_name);

        // Create info text that fits in available space
        let full_info = format!("Installing {}", pkg_name_styled);
        let info_text = if available_width < 15 {
            "Installing...".to_string()
        } else {
            full_info
        };

        // ## Critical: Use visual length, not string length for gap calculation
        // The styled string contains ANSI escape codes, so info_text.len() would be wrong
        let info_visual_len = "Installing ".len() + pkg_name.len();
        let used_width = 2 + info_visual_len + self.progress.width + pkg_count.len();
        let gap = if self.width > used_width {
            " ".repeat(self.width - used_width)
        } else {
            " ".to_string()
        };

        // Add the current installation line
        result.push_str(&format!(
            "{}{}{}{}{}",
            spin, info_text, gap, prog, pkg_count
        ));

        result
    }
}

// =============================================================================
// SAMPLE DATA GENERATION
// =============================================================================

/// Get a randomized list of packages with version numbers (matching Go implementation)
///
/// This creates whimsical package names that demonstrate text width handling,
/// including very long package names that will test truncation logic.
fn get_packages() -> Vec<String> {
    let packages = vec![
        "vegeutils".to_string(),
        "libgardening".to_string(),
        "currykit".to_string(),
        "spicerack".to_string(),
        "fullenglish".to_string(),
        "eggy".to_string(),
        "bad-kitty".to_string(),
        "chai".to_string(),
        "hojicha".to_string(),
        "libtacos".to_string(),
        "babys-monads".to_string(),
        "libpurring".to_string(),
        "currywurst-devel".to_string(),
        "xmodmeow".to_string(),
        "licorice-utils".to_string(),
        "cashew-apple".to_string(),
        "rock-lobster".to_string(),
        "standmixer".to_string(),
        "coffee-CUPS".to_string(),
        "libesszet".to_string(),
        "zeichenorientierte-benutzerschnittstellen".to_string(), // Very long name!
        "schnurrkit".to_string(),
        "old-socks-devel".to_string(),
        "jalapeño".to_string(),
        "molasses-utils".to_string(),
        "xkohlrabi".to_string(),
        "party-gherkin".to_string(),
        "snow-peas".to_string(),
        "libyuzu".to_string(),
    ];

    let mut rng = rand::thread_rng();
    let mut shuffled = packages.clone();
    shuffled.shuffle(&mut rng);

    // Add random version numbers (matching Go implementation)
    shuffled
        .into_iter()
        .map(|pkg| {
            format!(
                "{}-{}.{}.{}",
                pkg,
                rng.gen_range(0..10),
                rng.gen_range(0..10),
                rng.gen_range(0..10)
            )
        })
        .collect()
}

// =============================================================================
// MAIN PROGRAM
// =============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ## bubbletea-rs Pattern: Program Builder
    // The Program::builder() provides a fluent API for configuration:
    // - signal_handler(true) enables Ctrl+C handling
    // - build() creates the program with our model type
    // - run() starts the event loop
    let program = Program::<PackageManagerModel>::builder()
        .signal_handler(true) // Enable graceful Ctrl+C handling
        .build()?;

    // Run the program and handle any errors
    if let Err(err) = program.run().await {
        println!("Error running program: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
