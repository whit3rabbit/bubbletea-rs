use bubbletea_rs::{quit, BlurMsg, Cmd, FocusMsg, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};

/// Key bindings for the focus-blur example
#[derive(Debug)]
pub struct KeyBindings {
    pub quit: Binding,
    pub toggle: Binding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![
                with_keys_str(&["q", "ctrl+c"]),
                with_help("q/ctrl+c", "quit"),
            ]),
            toggle: new_binding(vec![
                with_keys_str(&["t"]),
                with_help("t", "toggle focus reporting"),
            ]),
        }
    }
}

#[derive(Debug)]
pub struct AppModel {
    focused: bool,
    reporting: bool,
    keys: KeyBindings,
}

impl Model for AppModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = AppModel {
            // assume we start focused...
            focused: true,
            reporting: true,
            keys: KeyBindings::default(),
        };
        (model, None)
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if msg.downcast_ref::<FocusMsg>().is_some() {
            self.focused = true;
        } else if msg.downcast_ref::<BlurMsg>().is_some() {
            self.focused = false;
        } else if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if self.keys.quit.matches(key_msg) {
                return Some(quit());
            }
            if self.keys.toggle.matches(key_msg) {
                self.reporting = !self.reporting;
            }
        }

        None
    }

    fn view(&self) -> String {
        let mut s = String::from("Hi. Focus report is currently ");
        if self.reporting {
            s.push_str("enabled");
        } else {
            s.push_str("disabled");
        }
        s.push_str(".\n\n");

        if self.reporting {
            if self.focused {
                s.push_str("This program is currently focused!");
            } else {
                s.push_str("This program is currently blurred!");
            }
        }

        s.push_str("\n\nTo quit sooner press ctrl-c, or t to toggle focus reporting...\n");
        s
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<AppModel>::builder()
        .report_focus(true) // Enable focus reporting
        .build()?;

    program.run().await?;

    Ok(())
}
