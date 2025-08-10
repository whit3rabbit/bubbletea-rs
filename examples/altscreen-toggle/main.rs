//! Alternate Screen Toggle Example
//!
//! This example mirrors the Go example `bubbletea/examples/altscreen-toggle`.
//! It toggles between the inline screen and the alternate screen with the
//! spacebar, supports suspend/resume with Ctrl+Z, and quits with q/esc.

use bubbletea_rs::{
    enter_alt_screen, exit_alt_screen, quit, suspend, Cmd, KeyMsg, Model, Msg, Program, QuitMsg,
    ResumeMsg,
};
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};
use lipgloss_extras::lipgloss::{Color, Style};

// Synthetic message used to trigger the initial render immediately after startup.
#[derive(Debug)]
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

// Key mappings for the altscreen toggle example
#[derive(Debug)]
pub struct KeyBindings {
    pub quit: Binding,
    pub suspend: Binding,
    pub toggle: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![
                with_keys_str(&["q", "esc"]),
                with_help("q/esc", "quit"),
            ]),
            suspend: new_binding(vec![
                with_keys_str(&["ctrl+z"]),
                with_help("ctrl+z", "suspend"),
            ]),
            toggle: new_binding(vec![
                with_keys_str(&["space"]),
                with_help("space", "toggle mode"),
            ]),
        }
    }
}

#[derive(Debug)]
pub struct AltScreenModel {
    pub altscreen: bool,
    pub quitting: bool,
    pub suspending: bool,
    pub keys: KeyBindings,
}

impl Model for AltScreenModel {
    fn init() -> (Self, Option<Cmd>) {
        (
            AltScreenModel {
                altscreen: false,
                quitting: false,
                suspending: false,
                keys: KeyBindings::default(),
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

        // Keyboard handling using key bindings
        if let Some(key) = msg.downcast_ref::<KeyMsg>() {
            if self.keys.quit.matches(key) {
                self.quitting = true;
                return Some(quit());
            }
            
            if self.keys.suspend.matches(key) {
                self.suspending = true;
                return Some(suspend());
            }
            
            if self.keys.toggle.matches(key) {
                // Toggle alt screen
                let cmd = if self.altscreen {
                    exit_alt_screen()
                } else {
                    enter_alt_screen()
                };
                self.altscreen = !self.altscreen;
                return Some(cmd);
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

        // Create lipgloss-extras styles matching the Go example
        let keyword_style = Style::new()
            .foreground(Color::from("204"))
            .background(Color::from("235"));
        
        let help_style = Style::new()
            .foreground(Color::from("241"));
        
        let mode = if self.altscreen {
            " altscreen mode "
        } else {
            " inline mode "
        };

        format!(
            "\n\n  You're in {}\n\n\n{}\n",
            keyword_style.render(mode),
            help_style.render("  space: switch modes • ctrl-z: suspend • q: exit")
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
