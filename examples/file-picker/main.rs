//! File Picker Example (Rust, using bubbletea-widgets)
//!
//! Port of Bubble Tea's `file-picker` example using `bubbletea-widgets::filepicker`.
// TODO: Fix tree view for file-pcker

use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::filepicker;
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};
use std::time::Duration;

/// Message type for clearing error messages after a delay
#[derive(Debug)]
struct ClearErrorMsg;

/// Key bindings for the file picker example
#[derive(Debug)]
pub struct KeyBindings {
    pub quit: Binding,
    pub quit_alt: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![with_keys_str(&["q"]), with_help("q", "quit")]),
            quit_alt: new_binding(vec![
                with_keys_str(&["ctrl+c"]),
                with_help("ctrl+c", "quit"),
            ]),
        }
    }
}

/// Function to create a command that clears error after a delay
fn clear_error_after(duration: Duration) -> Cmd {
    tick(duration, |_| Box::new(ClearErrorMsg) as Msg)
}

pub struct FilePickerModel {
    filepicker: filepicker::Model,
    selected_file: String,
    quitting: bool,
    error: Option<String>,
    keys: KeyBindings,
}

impl FilePickerModel {
    #[allow(dead_code)]
    fn new() -> Self {
        // Note: This method is not used since we configure everything in init()
        Self {
            filepicker: filepicker::new(),
            selected_file: String::new(),
            quitting: false,
            error: None,
            keys: KeyBindings::default(),
        }
    }
}

impl Model for FilePickerModel {
    fn init() -> (Self, Option<Cmd>) {
        // Create and configure the filepicker directly
        let (mut fp_model, cmd) = filepicker::Model::init();

        // Configure the filepicker to match the Go version
        fp_model.file_allowed = true;
        fp_model.dir_allowed = false; // Can browse directories but not select them (like Go version)
        fp_model.allowed_types = vec![
            ".mod".to_string(),
            ".sum".to_string(),
            ".go".to_string(),
            ".txt".to_string(),
            ".md".to_string(),
        ];
        fp_model.show_hidden = false;
        fp_model.show_permissions = true;
        fp_model.show_size = true;

        // Set height to show multiple entries (like the Go version)
        fp_model.set_height(15);
        fp_model.auto_height = true;

        // Force re-read directory to properly initialize min/max based on height
        // The filepicker needs to populate its file list to set viewport correctly

        // Set starting directory to user's home directory
        if let Ok(home_dir) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
            if let Ok(_) = std::env::set_current_dir(&home_dir) {
                // Successfully changed to home directory
            }
        }

        let model = Self {
            filepicker: fp_model,
            selected_file: String::new(),
            quitting: false,
            error: None,
            keys: KeyBindings::default(),
        };

        // Send a window size command to initialize the viewport properly
        let window_size_cmd = bubbletea_rs::command::window_size();

        if let Some(init_cmd) = cmd {
            (
                model,
                Some(bubbletea_rs::batch(vec![init_cmd, window_size_cmd])),
            )
        } else {
            (model, Some(window_size_cmd))
        }
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle quit keys
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if self.keys.quit.matches(key_msg) || self.keys.quit_alt.matches(key_msg) {
                self.quitting = true;
                return Some(quit());
            }
        }

        // Handle clear error message
        if msg.downcast_ref::<ClearErrorMsg>().is_some() {
            self.error = None;
            return None;
        }

        // Forward window size messages to filepicker first
        if msg.downcast_ref::<bubbletea_rs::WindowSizeMsg>().is_some() {
            return self.filepicker.update(msg);
        }

        // Check if user selected a file (not directory)
        if let (true, path) = self.filepicker.did_select_file(&msg) {
            self.selected_file = path;
            return None; // Don't continue processing since we got a valid selection
        }

        // Check if user selected a disabled file
        if let (true, path) = self.filepicker.did_select_disabled_file(&msg) {
            self.error = Some(format!("{} is not valid.", path));
            self.selected_file = String::new();

            // Update the file picker and batch with clear error command
            let fp_cmd = self.filepicker.update(msg);
            if let Some(fp_cmd) = fp_cmd {
                return Some(bubbletea_rs::batch(vec![
                    fp_cmd,
                    clear_error_after(Duration::from_secs(2)),
                ]));
            } else {
                return Some(clear_error_after(Duration::from_secs(2)));
            }
        }

        // Update the file picker
        let cmd = self.filepicker.update(msg);

        cmd
    }

    fn view(&self) -> String {
        if self.quitting {
            return String::new();
        }

        let mut output = String::new();
        output.push_str("\n  ");

        if let Some(error) = &self.error {
            // Display error message with disabled file style
            output.push_str(error);
        } else if self.selected_file.is_empty() {
            output.push_str("Pick a file:");
        } else {
            output.push_str(&format!("Selected file: {}", self.selected_file));
        }

        output.push_str("\n\n");
        output.push_str(&self.filepicker.view());
        output.push('\n');

        output
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<FilePickerModel>::builder()
        .alt_screen(true)
        .signal_handler(true)
        .build()?;

    let final_model = program.run().await?;

    // Display final selected file like the Go version
    if !final_model.selected_file.is_empty() {
        println!("\n  You selected: {}\n", final_model.selected_file);
    }

    Ok(())
}
