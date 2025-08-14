//! Simple Example
//!
//! A simple program that counts down from 5 and then exits.
//!
//! This is a faithful port of the Go Bubble Tea simple example, demonstrating:
//! - Basic countdown timer with custom tick messages  
//! - Direct keyboard input handling (q, Ctrl+C, Ctrl+Z)
//! - Simple integer model state
//! - Automatic program termination

use bubbletea_rs::{quit, suspend, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};
use std::time::Duration;

// A model can be more or less any type of data. It holds all the data for a
// program, so often it's a struct. For this simple example, however, all
// we'll need is a simple integer.
#[derive(Debug)]
struct SimpleModel(i32);

impl Model for SimpleModel {
    // Init optionally returns an initial command we should run. In this case we
    // want to start the timer.
    fn init() -> (Self, Option<Cmd>) {
        (SimpleModel(5), Some(tick()))
    }

    // Update is called when messages are received. The idea is that you inspect the
    // message and send back an updated model accordingly. You can also return
    // a command, which is a function that performs I/O and returns a message.
    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit());
                }
                KeyCode::Char('q') => {
                    return Some(quit());
                }
                KeyCode::Char('z') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(suspend());
                }
                _ => {}
            }
        }

        if let Some(_tick_msg) = msg.downcast_ref::<TickMsg>() {
            self.0 -= 1;
            if self.0 <= 0 {
                return Some(quit());
            }
            return Some(tick());
        }

        None
    }

    // View returns a string based on data in the model. That string which will be
    // rendered to the terminal.
    fn view(&self) -> String {
        format!(
            "Hi. This program will exit in {} seconds.\n\nTo quit sooner press ctrl-c, or press ctrl-z to suspend...\n",
            self.0
        )
    }
}

// Messages are events that we respond to in our Update function. This
// particular one indicates that the timer has ticked.
#[derive(Debug)]
struct TickMsg;

fn tick() -> Cmd {
    Box::pin(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        Some(Box::new(TickMsg) as Msg)
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize our program
    let program = Program::<SimpleModel>::builder()
        .signal_handler(true)
        .build()?;

    program.run().await?;

    Ok(())
}
