//! Autocomplete Example (Rust)
//!
//! Mirrors the Bubble Tea Go example at `bubbletea/examples/autocomplete`:
//! - Fetches Charmbracelet org repos from GitHub
//! - Text input with a fixed prompt `charmbracelet/`
//! - Suggestions dropdown that can be navigated with Ctrl+N / Ctrl+P
//! - Tab to accept the current suggestion
//! - Esc/Enter/Ctrl+C to quit
//!
//! Styling note: The Go example uses lipgloss (color 63) for prompt/cursor.
//! We add a comment here to capture that intent; terminal coloring is kept minimal.

use bubbletea_rs::command::batch;
use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::{Color, Stylize};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use serde::Deserialize;

const REPOS_URL: &str = "https://api.github.com/orgs/charmbracelet/repos";

// Synthetic message used to trigger the initial render immediately after startup.
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

#[derive(Debug, Clone, Deserialize)]
struct Repo {
    #[serde(rename = "name")]
    name: String,
}

#[derive(Debug, Clone)]
struct GotReposMsg(Vec<Repo>);

#[derive(Debug, Clone)]
struct GotReposErrMsg(String);

#[derive(Debug)]
pub struct AutocompleteModel {
    // Input
    input: String,
    prompt: String, // e.g., "charmbracelet/"

    // Suggestions
    all_suggestions: Vec<String>,
    filtered: Vec<String>,
    selected: usize, // index into filtered

    // Behavior flags
    show_suggestions: bool,
    quitting: bool,
}

impl AutocompleteModel {
    fn new() -> Self {
        Self {
            input: String::new(),
            prompt: "charmbracelet/".to_string(),
            all_suggestions: Vec::new(),
            filtered: Vec::new(),
            selected: 0,
            show_suggestions: true,
            quitting: false,
        }
    }

    fn apply_filter(&mut self) {
        let needle = self.input.to_lowercase();
        if needle.is_empty() {
            self.filtered = self.all_suggestions.clone();
        } else {
            self.filtered = self
                .all_suggestions
                .iter()
                .filter(|s| s.to_lowercase().starts_with(&needle))
                .cloned()
                .collect();
        }
        if self.selected >= self.filtered.len() {
            self.selected = 0;
        }
    }
}

impl Model for AutocompleteModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = Self::new();
        // Start HTTP request for repos and trigger an initial render immediately.
        (model, Some(batch(vec![fetch_repos(), init_render_cmd()])))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Initial render trigger; no state change required.
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            return None;
        }
        // Handle keyboard
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Enter | KeyCode::Esc => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Tab => {
                    if self.show_suggestions && !self.filtered.is_empty() {
                        // Accept current suggestion
                        self.input = self.filtered[self.selected].clone();
                        // After accepting, keep suggestions visible for continued navigation
                        self.apply_filter();
                    }
                    return None;
                }
                KeyCode::Char(c) => {
                    self.input.push(c);
                    self.apply_filter();
                    return None;
                }
                KeyCode::Backspace => {
                    self.input.pop();
                    self.apply_filter();
                    return None;
                }
                _ => {}
            }

            // Navigation keys (like Go example: ctrl+n/ctrl+p)
            if key_msg.modifiers.contains(KeyModifiers::CONTROL) {
                match key_msg.key {
                    KeyCode::Char('n') => {
                        if !self.filtered.is_empty() {
                            self.selected = (self.selected + 1) % self.filtered.len();
                        }
                        return None;
                    }
                    KeyCode::Char('p') => {
                        if !self.filtered.is_empty() {
                            if self.selected == 0 {
                                self.selected = self.filtered.len() - 1;
                            } else {
                                self.selected -= 1;
                            }
                        }
                        return None;
                    }
                    _ => {}
                }
            }
        }

        // HTTP results
        if let Some(GotReposMsg(repos)) = msg.downcast_ref::<GotReposMsg>().cloned() {
            self.all_suggestions = repos.into_iter().map(|r| r.name).collect();
            self.apply_filter();
            return None;
        }
        if let Some(GotReposErrMsg(err)) = msg.downcast_ref::<GotReposErrMsg>().cloned() {
            // Keep running; just show no suggestions and append an error note in the view
            eprintln!("error fetching repos: {}", err);
            return None;
        }

        None
    }

    fn view(&self) -> String {
        if self.quitting {
            return String::from("");
        }

        let title = "Pick a Charm™ repo:";

        // Style prompt similar to lipgloss color 63 (approx RGB). We'll use a blue-ish color.
        let prompt_styled = format!("{}", self.prompt.as_str().with(Color::Blue));
        let input_line = format!("  {}{}", prompt_styled, self.input.clone());

        // Suggestions list: show first up to 8 items
        let mut sug = String::new();
        if self.show_suggestions && !self.filtered.is_empty() {
            let max = 8.min(self.filtered.len());
            for (i, s) in self.filtered.iter().take(max).enumerate() {
                if i == self.selected {
                    let selected_styled = format!("{}", s.as_str().with(Color::Blue));
                    sug.push_str(&format!("\n> {}", selected_styled));
                } else {
                    sug.push_str(&format!("\n  {}", s));
                }
            }
        }

        // Help line at bottom
        let help = "tab complete • ctrl+n next • ctrl+p prev • esc quit";

        format!("{}\n\n{}{}\n\n{}\n\n", title, input_line, sug, help)
    }
}

fn fetch_repos() -> Cmd {
    Box::pin(async move {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            HeaderValue::from_static("2022-11-28"),
        );
        // Add a UA as GitHub may reject requests without it
        headers.insert(
            "User-Agent",
            HeaderValue::from_static("bubbletea-rs-autocomplete-example"),
        );

        let client = match reqwest::Client::builder().default_headers(headers).build() {
            Ok(c) => c,
            Err(e) => return Some(Box::new(GotReposErrMsg(e.to_string())) as Msg),
        };

        match client.get(REPOS_URL).send().await {
            Ok(resp) => match resp.json::<Vec<Repo>>().await {
                Ok(repos) => Some(Box::new(GotReposMsg(repos)) as Msg),
                Err(e) => Some(Box::new(GotReposErrMsg(e.to_string())) as Msg),
            },
            Err(e) => Some(Box::new(GotReposErrMsg(e.to_string())) as Msg),
        }
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<AutocompleteModel>::builder()
        .signal_handler(true)
        .alt_screen(true)
        .build()?;

    program.run().await?;
    Ok(())
}
