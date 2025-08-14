//! File Picker Example (Rust, using bubbletea-widgets)
//!
//! Port of Bubble Tea's `file-picker` example using `bubbletea-widgets::filepicker`.
//! This example demonstrates the improved filepicker with proper message ordering,
//! robust viewport handling, and better user experience.

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
    pub quit_escape: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![with_keys_str(&["q"]), with_help("q", "quit")]),
            quit_alt: new_binding(vec![
                with_keys_str(&["ctrl+c"]),
                with_help("ctrl+c", "quit"),
            ]),
            quit_escape: new_binding(vec![with_keys_str(&["esc"]), with_help("esc", "quit")]),
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
        // Create and configure the filepicker using the standard init pattern
        let (mut fp_model, init_cmd) = filepicker::Model::init();

        // Configure the filepicker to match the Go version
        fp_model.file_allowed = true;
        fp_model.dir_allowed = false; // Can browse directories but not select them (like Go version)
        fp_model.allowed_types = vec![
            ".mod".to_string(),
            ".sum".to_string(),
            ".go".to_string(),
            ".txt".to_string(),
            ".md".to_string(),
            ".rs".to_string(),
            ".toml".to_string(),
        ];
        fp_model.show_hidden = false;
        fp_model.show_permissions = true;
        fp_model.show_size = true;

        // Set height to show multiple entries (like the Go version)
        fp_model.set_height(15);
        fp_model.auto_height = true;

        // Set the starting directory on the model itself (avoid changing process CWD)
        if let Ok(home_dir) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
            fp_model.current_directory = std::path::PathBuf::from(home_dir);
            // Re-read directory after changing the path
            fp_model.read_dir();
        }

        let model = Self {
            filepicker: fp_model,
            selected_file: String::new(),
            quitting: false,
            error: None,
            keys: KeyBindings::default(),
        };

        // Send a window size command to initialize the viewport properly
        let window_size_cmd = bubbletea_rs::window_size();

        if let Some(init_cmd) = init_cmd {
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
            if self.keys.quit.matches(key_msg)
                || self.keys.quit_alt.matches(key_msg)
                || self.keys.quit_escape.matches(key_msg)
            {
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

        // Check if user selected a file (not directory) - quit immediately like Go version
        if let (true, path) = self.filepicker.did_select_file(&msg) {
            self.selected_file = path;
            self.quitting = true;
            return Some(quit()); // Quit immediately upon selection
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
        self.filepicker.update(msg)
    }

    fn view(&self) -> String {
        if self.quitting {
            return String::new();
        }

        let mut output = String::new();
        output.push_str("\n  ");

        if let Some(error) = &self.error {
            // Style error message using filepicker's disabled file style for better UX
            let error_style = self.filepicker.styles.disabled_file.clone();
            output.push_str(&error_style.render(error));
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
