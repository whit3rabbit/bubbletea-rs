use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};
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
            match key_msg.key {
                KeyCode::Char('q') | KeyCode::Char('c') | KeyCode::Esc => {
                    if key_msg.key == KeyCode::Char('c')
                        && !key_msg.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        return None;
                    }
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
            s.push_str(&format!("{} {}", status, status_text(status)));
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

// Simple status text mapping (subset of HTTP status codes)
fn status_text(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        304 => "Not Modified",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        _ => "Unknown",
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<HttpModel>::builder().build()?;
    program.run().await?;
    Ok(())
}
