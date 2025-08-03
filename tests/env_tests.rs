use std::collections::HashMap;
use std::process::Command as StdCommand;

use bubbletea_rs::{command, Model, Msg, Program};

#[derive(Debug, Clone)]
struct DummyModel;

impl Model for DummyModel {
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) {
        (Self, None)
    }

    fn update(&mut self, _msg: Msg) -> Option<bubbletea_rs::Cmd> {
        None
    }

    fn view(&self) -> String {
        String::new()
    }
}

#[derive(Debug)]
struct EnvOut(String);

#[tokio::test]
async fn test_exec_process_uses_configured_environment() {
    // Arrange: build a Program that sets COMMAND_ENV
    let mut env = HashMap::new();
    env.insert("MY_VAR".to_string(), "hello-world".to_string());

    // Building is sufficient to set the global env; we don't need to run the program.
    let _program: Program<DummyModel> = Program::<DummyModel>::builder()
        .with_environment(env)
        .without_renderer()
        .build()
        .expect("program build");

    // Prepare a platform-appropriate command that echos the environment variable.
    #[cfg(windows)]
    let cmd = {
        let mut c = StdCommand::new("cmd");
        c.args(["/C", "echo %MY_VAR%"]);
        c
    };

    #[cfg(not(windows))]
    let cmd = {
        let mut c = StdCommand::new("sh");
        c.args(["-c", "printf %s \"$MY_VAR\""]);
        c
    };

    let fut = command::exec_process(cmd, |res| match res {
        Ok(output) => {
            let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Box::new(EnvOut(s)) as Msg
        }
        Err(e) => Box::new(EnvOut(format!("ERR:{e}"))) as Msg,
    });

    // Act: run the command
    let msg = fut.await.expect("command produced a message");

    // Assert: downcast and compare
    let out = msg.downcast_ref::<EnvOut>().expect("EnvOut msg");
    assert_eq!(out.0, "hello-world");
}
