use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};
use http::StatusCode;
use std::time::Duration;

// A simple program that makes a GET request and prints the response status.

const URL: &str = "https://charm.sh/";

#[derive(Debug, Clone)]
pub struct StatusMsg {
    pub status: u16,
}

#[derive(Debug, Clone)]
pub struct ErrMsg {
    pub error: String,
}

struct HttpModel {
    status: Option<u16>,
    error: Option<String>,
}

impl Model for HttpModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = Self {
            status: None,
            error: None,
        };

        (model, Some(check_server()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match (key_msg.key, key_msg.modifiers) {
                (KeyCode::Char('q'), _) | (KeyCode::Esc, _) => {
                    return Some(quit());
                }
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    return Some(quit());
                }
                _ => return None,
            }
        }

        if let Some(status_msg) = msg.downcast_ref::<StatusMsg>() {
            self.status = Some(status_msg.status);
            return Some(quit()); // Quit after receiving status, just like Go version
        }

        if let Some(err_msg) = msg.downcast_ref::<ErrMsg>() {
            self.error = Some(err_msg.error.clone());
            return None;
        }

        None
    }

    fn view(&self) -> String {
        let mut s = format!("Checking {}...", URL);

        if let Some(error) = &self.error {
            s.push_str(&format!("something went wrong: {}", error));
        } else if let Some(status) = self.status {
            let status_code = StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            s.push_str(&format!("{} {}", status, status_code.canonical_reason().unwrap_or("Unknown")));
        }

        s.push('\n');
        s
    }
}

fn check_server() -> Cmd {
    Box::pin(async move {
        let client = match reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
        {
            Ok(client) => client,
            Err(e) => {
                return Some(Box::new(ErrMsg {
                    error: e.to_string(),
                }) as Msg);
            }
        };

        match client.get(URL).send().await {
            Ok(response) => {
                let status = response.status().as_u16();
                Some(Box::new(StatusMsg { status }) as Msg)
            }
            Err(e) => Some(Box::new(ErrMsg {
                error: e.to_string(),
            }) as Msg),
        }
    })
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<HttpModel>::builder().build()?;
    program.run().await?;
    Ok(())
}
