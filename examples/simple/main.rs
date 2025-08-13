//! Simple Example
//!
//! A basic countdown timer that demonstrates:
//! - Timer messages using the `every()` command
//! - Keyboard input handling (q, Ctrl+C, Ctrl+Z)
//! - Basic state management
//! - Automatic program termination
//!
//! This example counts down from 5 to 0 and then exits automatically.

use bubbletea_rs::{quit, suspend, tick, Cmd, KeyMsg, Model, Msg, Program, QuitMsg};
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};
use std::time::Duration;

/// Custom message type for timer ticks
#[derive(Debug)]
pub struct TickMsg;

/// Key bindings for the simple example
#[derive(Debug)]
pub struct KeyBindings {
    pub quit: Binding,
    pub suspend: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![with_keys_str(&["q"]), with_help("q", "quit")]),
            suspend: new_binding(vec![
                with_keys_str(&["ctrl+z"]),
                with_help("ctrl+z", "suspend"),
            ]),
        }
    }
}

/// The model represents our application state - just a simple counter
#[derive(Debug)]
pub struct SimpleModel {
    pub count: i32,
    pub keys: KeyBindings,
}

impl Model for SimpleModel {
    fn init() -> (Self, Option<Cmd>) {
        // Start with count of 5 and begin the timer
        let model = SimpleModel {
            count: 5,
            keys: KeyBindings::default(),
        };
        let cmd = tick(Duration::from_secs(1), |_| Box::new(TickMsg) as Msg);
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle keyboard input using key bindings
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if self.keys.quit.matches(key_msg) {
                return Some(quit());
            }
            if self.keys.suspend.matches(key_msg) {
                return Some(suspend());
            }
        }

        // Handle timer ticks
        if msg.downcast_ref::<TickMsg>().is_some() {
            self.count -= 1;

            // If countdown reaches 0, quit automatically
            if self.count <= 0 {
                return Some(quit());
            }

            // Re-arm next single-shot tick
            return Some(tick(Duration::from_secs(1), |_| Box::new(TickMsg) as Msg));
        }

        // Handle quit messages
        if msg.downcast_ref::<QuitMsg>().is_some() {
            // Program is quitting - no further commands needed
            return None;
        }

        None
    }

    fn view(&self) -> String {
        format!(
            "Hi. This program will exit in {} seconds.\n\nTo quit sooner press ctrl-c, or press ctrl-z to suspend...\n",
            self.count
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the program with default settings
    let program = Program::<SimpleModel>::builder()
        .signal_handler(true) // Enable Ctrl+C handling
        .alt_screen(false) // Match Go version - no alternate screen
        .build()?;

    program.run().await?;

    Ok(())
}
