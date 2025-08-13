use std::env;

use bubbletea_rs::{
    KeyMsg, Msg, Model, Program, Cmd,
    enter_alt_screen, exit_alt_screen, quit,
};
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};

// Synthetic message used to trigger the initial render immediately after startup.
#[derive(Debug)]
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

#[derive(Debug)]
struct EditorFinishedMsg {
    err: Option<String>,
}

#[derive(Debug)]
pub struct KeyBindings {
    pub toggle_altscreen: Binding,
    pub open_editor: Binding,
    pub quit: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            toggle_altscreen: new_binding(vec![with_keys_str(&["a"]), with_help("a", "toggle altscreen")]),
            open_editor: new_binding(vec![with_keys_str(&["e"]), with_help("e", "open editor")]),
            quit: new_binding(vec![
                with_keys_str(&["q", "ctrl+c"]),
                with_help("q", "quit"),
            ]),
        }
    }
}

fn open_editor() -> Cmd {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    
    Box::pin(async move {
        use std::process::{Command, Stdio};
        
        // Create a command that inherits stdin/stdout/stderr to allow interactive editing
        let mut cmd = Command::new(&editor);
        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        
        // Execute the command and wait for it to complete
        match cmd.status() {
            Ok(status) if status.success() => {
                Some(Box::new(EditorFinishedMsg { err: None }) as Msg)
            }
            Ok(_) => {
                Some(Box::new(EditorFinishedMsg { 
                    err: Some("Editor exited with non-zero status".to_string())
                }) as Msg)
            }
            Err(e) => {
                Some(Box::new(EditorFinishedMsg { 
                    err: Some(format!("Failed to execute editor: {}", e))
                }) as Msg)
            }
        }
    })
}

#[derive(Default)]
struct ExecModel {
    altscreen_active: bool,
    err: Option<String>,
    keys: KeyBindings,
}

impl Model for ExecModel {
    fn init() -> (Self, Option<Cmd>) {
        (Self::default(), Some(init_render_cmd()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Initial render trigger is a no-op beyond causing the first render
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            return None;
        }

        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if self.keys.toggle_altscreen.matches(key_msg) {
                self.altscreen_active = !self.altscreen_active;
                return Some(if self.altscreen_active {
                    enter_alt_screen()
                } else {
                    exit_alt_screen()
                });
            }
            
            if self.keys.open_editor.matches(key_msg) {
                return Some(open_editor());
            }
            
            if self.keys.quit.matches(key_msg) {
                return Some(quit());
            }
        } else if let Some(editor_msg) = msg.downcast_ref::<EditorFinishedMsg>() {
            if let Some(err) = &editor_msg.err {
                self.err = Some(err.clone());
                return Some(quit());
            }
        }
        
        None
    }

    fn view(&self) -> String {
        if let Some(err) = &self.err {
            format!("Error: {}\n", err)
        } else {
            let mode = if self.altscreen_active {
                "ALTSCREEN MODE"
            } else {
                "INLINE MODE"
            };
            format!(
                "Current mode: {}\n\nPress 'e' to open your EDITOR.\nPress 'a' to toggle the altscreen\nPress 'q' to quit.\n",
                mode
            )
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<ExecModel>::builder()
        .signal_handler(true)
        .build()?;
    
    program.run().await?;
    
    Ok(())
}