//! Debounce Example
//!
//! This example illustrates how to debounce commands.
//!
//! When the user presses a key we increment the "tag" value on the model and,
//! after a short delay, we include that tag value in the message produced
//! by the Tick command.
//!
//! In a subsequent Update, if the tag in the Msg matches current tag on the
//! model's state we know that the debouncing is complete and we can proceed as
//! normal. If not, we simply ignore the inbound message.

use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use std::time::Duration;

/// Duration to wait for debouncing
const DEBOUNCE_DURATION: Duration = Duration::from_secs(1);

/// Custom message type for exit signals with tag
#[derive(Debug)]
pub struct ExitMsg(pub i32);

/// Synthetic message used to trigger the initial render immediately after startup
#[derive(Debug)]
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

/// The model represents our application state
#[derive(Debug)]
pub struct DebounceModel {
    pub tag: i32,
}

impl Model for DebounceModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = DebounceModel { tag: 0 };
        (model, Some(init_render_cmd()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle keyboard input
        if let Some(_key_msg) = msg.downcast_ref::<KeyMsg>() {
            // Increment the tag on the model...
            self.tag += 1;
            let current_tag = self.tag;

            // ...and schedule an exit message with a copy of that tag value
            return Some(tick(DEBOUNCE_DURATION, move |_| {
                Box::new(ExitMsg(current_tag)) as Msg
            }));
        }

        // Handle exit messages
        if let Some(exit_msg) = msg.downcast_ref::<ExitMsg>() {
            // If the tag in the message doesn't match the tag on the model then we
            // know that this message was not the last one sent and another is on
            // the way. If that's the case we know, we can ignore this message.
            // Otherwise, the debounce timeout has passed and this message is a
            // valid debounced one.
            if exit_msg.0 == self.tag {
                return Some(quit());
            }
        }

        // Handle initial render message (no-op, just triggers view)
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            // This message just triggers the initial render
            return None;
        }

        None
    }

    fn view(&self) -> String {
        format!(
            "Key presses: {}\nTo exit press any key, then wait for one second without pressing anything.",
            self.tag
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the program with default settings
    let program = Program::<DebounceModel>::builder()
        .signal_handler(true) // Enable Ctrl+C handling
        .alt_screen(false) // Match Go version - no alternate screen
        .build()?;

    program.run().await?;

    Ok(())
}
