//! Pipe Example (Rust, using bubbletea-widgets and lipgloss)
//!
//! Port of Bubble Tea's `pipe` example. This demonstrates how to pipe data into
//! a Bubble Tea application and handle non-TTY input scenarios.

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};
use bubbletea_widgets::textinput;
use std::io::{self, Read};

/// Key bindings for the pipe example
#[derive(Debug)]
pub struct KeyBindings {
    pub quit: Binding,
    pub quit_alt: Binding,
    pub quit_enter: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![with_keys_str(&["esc"]), with_help("esc", "quit")]),
            quit_alt: new_binding(vec![
                with_keys_str(&["ctrl+c"]),
                with_help("ctrl+c", "quit"),
            ]),
            quit_enter: new_binding(vec![with_keys_str(&["enter"]), with_help("enter", "quit")]),
        }
    }
}

pub struct PipeModel {
    user_input: textinput::Model,
    quitting: bool,
    keys: KeyBindings,
}

impl PipeModel {
    fn new(initial_value: String) -> Self {
        let mut ti = textinput::new();

        // Configure the textinput similar to the Go version
        ti.set_width(48);
        ti.set_value(&initial_value);
        ti.cursor_end();

        Self {
            user_input: ti,
            quitting: false,
            keys: KeyBindings::default(),
        }
    }
}

impl Model for PipeModel {
    fn init() -> (Self, Option<Cmd>) {
        // Read piped input from stdin
        let piped_input = read_piped_input().unwrap_or_else(|_| {
            eprintln!("Try piping in some text.");
            std::process::exit(1);
        });

        let mut model = Self::new(piped_input);
        let cmd = model.user_input.focus();
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle quit keys first
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if self.keys.quit.matches(key_msg)
                || self.keys.quit_alt.matches(key_msg)
                || self.keys.quit_enter.matches(key_msg)
            {
                self.quitting = true;
                return Some(quit());
            }
        }

        self.user_input.update(msg)
    }

    fn view(&self) -> String {
        if self.quitting {
            return String::new();
        }

        format!(
            "\nYou piped in: {}\n\nPress ^C to exit",
            self.user_input.view()
        )
    }
}

/// Read piped input from stdin, similar to the Go version
fn read_piped_input() -> Result<String, io::Error> {
    // Check if stdin is a pipe/redirected input
    use std::os::unix::io::AsRawFd;
    let stdin_fd = io::stdin().as_raw_fd();

    // Use libc to check if stdin is a TTY
    let is_tty = unsafe { libc::isatty(stdin_fd) } == 1;

    if is_tty {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No piped input detected",
        ));
    }

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<PipeModel>::builder()
        .alt_screen(false) // Don't use alt screen for pipe example
        .signal_handler(true)
        .build()?;

    let _ = program.run().await?;
    Ok(())
}
