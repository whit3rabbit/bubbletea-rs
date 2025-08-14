//! Sequence Example
//!
//! A simple example illustrating how to run a series of commands in order.
//!
//! This example demonstrates:
//! - Using `sequence()` to run commands in order
//! - Using `batch()` to run commands concurrently within a sequence
//! - Command composition and orchestration
//! - Automatic program termination after command completion

use bubbletea_rs::{batch, println, quit, sequence, Cmd, KeyMsg, Model, Msg, Program};

/// The application model - minimal like the Go version
#[derive(Debug)]
struct SequenceModel;

impl Model for SequenceModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = SequenceModel;

        // Create the sequence command exactly like the Go version:
        // 1. First run A, B, C concurrently via batch
        // 2. Then run Z after the batch completes
        // 3. Then quit the program
        let sequence_cmd = sequence(vec![
            batch(vec![
                println("A".to_string()),
                println("B".to_string()),
                println("C".to_string()),
            ]),
            println("Z".to_string()),
            quit(),
        ]);

        (model, Some(sequence_cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle keyboard input - any key quits (matching Go version)
        if let Some(_key_msg) = msg.downcast_ref::<KeyMsg>() {
            return Some(quit());
        }

        None
    }

    fn view(&self) -> String {
        // Empty view like the Go version - all output is via println commands
        String::new()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<SequenceModel>::builder().build()?;

    // Run the program
    program.run().await?;

    Ok(())
}
