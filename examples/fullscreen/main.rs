//! Fullscreen Example
//!
//! This example demonstrates how to use the alternate screen buffer (fullscreen mode)
//! in a bubbletea-rs application. Key features:
//!
//! - **Alternate Screen Buffer**: The application takes over the entire terminal screen
//!   and restores the original terminal content when it exits
//! - **Simple Countdown Timer**: Counts down from 5 to 0 using single-shot timers
//! - **Automatic Exit**: Program exits automatically when countdown reaches zero
//! - **Keyboard Input**: User can quit early with 'q', 'esc', or 'ctrl+c'
//!
//! The alternate screen buffer is useful for applications that need to temporarily
//! take over the entire terminal (like editors, games, or full-screen interfaces)
//! without affecting the user's existing terminal content.

use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};
use std::time::Duration;

/// Custom message type for timer ticks
/// This message is sent every second to decrement the countdown
#[derive(Debug)]
pub struct TickMsg;

/// Key bindings for the fullscreen example
/// Defines which keys can be used to quit the application
#[derive(Debug)]
pub struct KeyBindings {
    pub quit: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            // Allow multiple ways to quit: q, escape, or ctrl+c
            quit: new_binding(vec![
                with_keys_str(&["q", "esc", "ctrl+c"]),
                with_help("q/esc/ctrl+c", "quit"),
            ]),
        }
    }
}

/// The model represents our application state
/// Contains the countdown timer and key bindings
#[derive(Debug)]
pub struct FullscreenModel {
    /// Current countdown value (starts at 5, decrements to 0)
    pub count: i32,
    /// Key bindings for user input handling
    pub keys: KeyBindings,
}

impl Model for FullscreenModel {
    fn init() -> (Self, Option<Cmd>) {
        // Initialize the model with countdown starting at 5
        let model = FullscreenModel {
            count: 5,
            keys: KeyBindings::default(),
        };

        // Start the first timer tick - this will fire after 1 second
        let cmd = tick(Duration::from_secs(1), |_| Box::new(TickMsg) as Msg);
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle keyboard input - check if user wants to quit
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if self.keys.quit.matches(key_msg) {
                return Some(quit());
            }
        }

        // Handle timer tick messages
        if msg.downcast_ref::<TickMsg>().is_some() {
            // Decrement the countdown
            self.count -= 1;

            // If countdown reaches 0 or below, exit the program
            if self.count <= 0 {
                return Some(quit());
            }

            // Otherwise, schedule the next tick in 1 second
            // Using single-shot tick() rather than every() to avoid accumulating timers
            return Some(tick(Duration::from_secs(1), |_| Box::new(TickMsg) as Msg));
        }

        None
    }

    fn view(&self) -> String {
        // Simple view showing the countdown with some padding for center alignment
        format!(
            "\n\n     Hi. This program will exit in {} seconds...",
            self.count
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the program with alternate screen buffer enabled
    // The .alt_screen(true) setting is the key feature being demonstrated:
    // - It switches to the alternate screen buffer on startup
    // - The application takes over the full terminal screen
    // - When the program exits, the original terminal content is restored
    let program = Program::<FullscreenModel>::builder()
        .alt_screen(true) // Enable alternate screen buffer (fullscreen mode)
        .build()?;

    // Run the program - this will switch to fullscreen mode and start the countdown
    program.run().await?;

    // When we reach this point, the alternate screen has been disabled
    // and the user's original terminal content is restored
    Ok(())
}
