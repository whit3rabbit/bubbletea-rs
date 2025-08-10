//! Window Size Example
//!
//! Demonstrates:
//! - Window size detection using the `window_size()` command
//! - Handling resize events through `WindowSizeMsg`
//! - Real-time terminal dimension updates
//! - Basic keyboard controls for quitting
//!
//! This example shows the current terminal dimensions and updates
//! them in real-time when the terminal is resized.

use bubbletea_rs::{quit, window_size, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};

/// Key bindings for the window-size example
#[derive(Debug)]
pub struct KeyBindings {
    pub quit: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![
                with_keys_str(&["q", "esc"]),
                with_help("any key", "quit"),
            ]),
        }
    }
}

/// The model holds the current terminal dimensions
#[derive(Debug)]
pub struct WindowSizeModel {
    pub width: u16,
    pub height: u16,
    pub ready: bool, // Whether we've received initial size
    pub keys: KeyBindings,
}

impl Model for WindowSizeModel {
    fn init() -> (Self, Option<Cmd>) {
        // Start with default dimensions and request the actual window size
        let model = WindowSizeModel {
            width: 0,
            height: 0,
            ready: false,
            keys: KeyBindings::default(),
        };

        // Immediately request the current window size
        let cmd = window_size();
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle keyboard input - any key quits
        if let Some(_key_msg) = msg.downcast_ref::<KeyMsg>() {
            return Some(quit());
        }

        // Handle window size messages (including resize events)
        if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            self.width = size_msg.width;
            self.height = size_msg.height;
            self.ready = true;
        }

        None
    }

    fn view(&self) -> String {
        if !self.ready {
            "Getting terminal dimensions...\n\nPress any key to quit.".to_string()
        } else {
            let total_cells = self.width as u32 * self.height as u32;

            format!(
                "Terminal Size Information\n\
                 =========================\n\n\
                 Width:  {} columns\n\
                 Height: {} rows\n\
                 Total:  {} cells\n\n\
                 Try resizing your terminal to see the values update!\n\n\
                 Press any key to quit.",
                self.width, self.height, total_cells
            )
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting window size example...");
    println!("Resize your terminal window to see the dimensions update in real-time!");

    // Create and run the program
    let program = Program::<WindowSizeModel>::builder()
        .alt_screen(true) // Use alternate screen for cleaner display
        .signal_handler(true) // Enable Ctrl+C handling
        .build()?;

    // Run the program and get the final model state
    let final_model = program.run().await?;

    println!(
        "Program finished. Final dimensions: {}x{}",
        final_model.width, final_model.height
    );

    Ok(())
}
