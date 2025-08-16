use bubbletea_rs::{interrupt, quit, suspend, Cmd, KeyMsg, Model, Msg, Program, ResumeMsg};
use crossterm::event::{KeyCode, KeyModifiers};
use std::process;

struct SuspendModel {
    quitting: bool,
    suspending: bool,
}

// Synthetic message used to trigger the initial render immediately after startup.
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

impl Model for SuspendModel {
    fn init() -> (Self, Option<Cmd>) {
        (
            Self {
                quitting: false,
                suspending: false,
            },
            Some(init_render_cmd()),
        )
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            return None;
        }

        if msg.downcast_ref::<ResumeMsg>().is_some() {
            self.suspending = false;
            return None;
        }

        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quitting = true;
                    return Some(interrupt());
                }
                KeyCode::Char('z') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.suspending = true;
                    return Some(suspend());
                }
                _ => {}
            }
        }

        None
    }

    fn view(&self) -> String {
        if self.suspending || self.quitting {
            return String::new();
        }

        "\nPress ctrl-z to suspend, ctrl+c to interrupt, q, or esc to exit\n".to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<SuspendModel>::builder()
        .signal_handler(true)
        .build()?;

    if let Err(err) = program.run().await {
        eprintln!("Error running program: {}", err);
        match err {
            bubbletea_rs::Error::Interrupted => {
                process::exit(130);
            }
            _ => {
                process::exit(1);
            }
        }
    }

    Ok(())
}
