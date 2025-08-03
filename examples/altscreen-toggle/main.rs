//! Alternate Screen Toggle Example
//!
//! This example mirrors the Go example `bubbletea/examples/altscreen-toggle`.
//! It toggles between the inline screen and the alternate screen with the
//! spacebar, supports suspend/resume with Ctrl+Z, and quits with q/esc.

use bubbletea_rs::{
    enter_alt_screen, exit_alt_screen, quit, suspend, Cmd, KeyMsg, Model, Msg, Program, QuitMsg,
    ResumeMsg,
};
use crossterm::event::{KeyCode, KeyModifiers};

// Synthetic message used to trigger the initial render immediately after startup.
#[derive(Debug)]
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

#[derive(Debug)]
pub struct AltScreenModel {
    pub altscreen: bool,
    pub quitting: bool,
    pub suspending: bool,
}

impl Model for AltScreenModel {
    fn init() -> (Self, Option<Cmd>) {
        (
            AltScreenModel {
                altscreen: false,
                quitting: false,
                suspending: false,
            },
            // Trigger an initial render so the view shows immediately on startup.
            Some(init_render_cmd()),
        )
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Initial render trigger is a no-op beyond causing the first render
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            return None;
        }

        // Resume clears suspending state
        if msg.downcast_ref::<ResumeMsg>().is_some() {
            self.suspending = false;
            return None;
        }

        // Keyboard handling
        if let Some(key) = msg.downcast_ref::<KeyMsg>() {
            match key.key {
                KeyCode::Char('q') => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Esc => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Char('z') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.suspending = true;
                    return Some(suspend());
                }
                KeyCode::Char(' ') => {
                    // Toggle alt screen
                    let cmd = if self.altscreen {
                        exit_alt_screen()
                    } else {
                        enter_alt_screen()
                    };
                    self.altscreen = !self.altscreen;
                    return Some(cmd);
                }
                _ => {}
            }
        }

        // Explicit quit message - nothing to do other than allow program to exit
        if msg.downcast_ref::<QuitMsg>().is_some() {
            return None;
        }

        None
    }

    fn view(&self) -> String {
        if self.suspending {
            return String::new();
        }
        if self.quitting {
            return "Bye!\n".to_string();
        }

        // TODO(UX): Use a lipgloss-like styling crate for Rust (when available)
        // to render the mode label with foreground/background colors and tint
        // the help text, matching the Go example:
        //   - keywordStyle: fg=204, bg=235 for the " altscreen mode "/" inline mode " label
        //   - helpStyle:    fg=241 for the help line
        // For now we keep plain text to avoid adding ad-hoc ANSI styling.
        let mode = if self.altscreen {
            " altscreen mode "
        } else {
            " inline mode "
        };

        format!(
            "\n\n  You're in {}\n\n\n{}\n",
            mode, "  space: switch modes • ctrl-z: suspend • q: exit"
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Print a demo command to the inline (main) screen buffer. When you toggle
    // to the alternate screen this line disappears; when you toggle back to
    // inline it shows again, highlighting the screen buffer switch.
    println!("$ ./altscreen-toggle");

    let program = Program::<AltScreenModel>::builder()
        .signal_handler(true)
        .alt_screen(false) // start in inline mode like the Go example
        .build()?;

    program.run().await?;
    Ok(())
}
