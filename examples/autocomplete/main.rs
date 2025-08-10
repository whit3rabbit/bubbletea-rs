//! Autocomplete Example (Rust)
//!
//! Mirrors the Bubble Tea Go example at `bubbletea/examples/autocomplete`:
//! - Fetches Charmbracelet org repos from GitHub
//! - Text input with a fixed prompt `charmbracelet/`
//! - Suggestions dropdown navigable with Ctrl+N / Ctrl+P
//! - Tab to accept the current suggestion
//! - Esc/Enter/Ctrl+C to quit
//!
//! Components: `bubbletea-widgets::{textinput, key, help}`.
//!
//! Note: If typing appears to be ignored, ensure your workspace resolves to a
//! single instance of the `bubbletea-rs` crate. Because messages are carried
//! via `Box<dyn Any>`, downcasting (e.g., to `KeyMsg`) requires the exact same
//! concrete type from the same crate instance. Multiple versions/paths of the
//! crate at once will prevent `downcast_ref::<KeyMsg>()` from matching in
//! widget code. See this example's README “Troubleshooting” for details.

use bubbletea_rs::command::batch;
use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::{help, key, textinput};
use lipgloss_extras::lipgloss::{Color, Style};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use serde::Deserialize;

const REPOS_URL: &str = "https://api.github.com/orgs/charmbracelet/repos";

// Triggers an initial render immediately after startup to avoid a blank screen
// during early async work (e.g., HTTP fetch).
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

// Application key map. We primarily display these in the help view.
// Most editing/navigation keys are handled by the text input widget itself;
// here we include only the app-level bindings we want surfaced in help.
struct AppKeyMap {
    tab: key::Binding,
    next: key::Binding,  
    prev: key::Binding,
    quit: key::Binding,
}

impl Default for AppKeyMap {
    fn default() -> Self {
        Self {
            // These are handled by textinput but shown in help for discoverability
            tab: key::new_binding(vec![key::with_keys_str(&["tab"])]).with_help("tab", "complete"),
            next: key::new_binding(vec![key::with_keys_str(&["ctrl+n"])]).with_help("ctrl+n", "next"),
            prev: key::new_binding(vec![key::with_keys_str(&["ctrl+p"])]).with_help("ctrl+p", "prev"),
            // Quit is handled by our app after delegating the message to the input
            quit: key::new_binding(vec![key::with_keys_str(&["esc", "enter", "ctrl+c"])]).with_help("esc", "quit"),
        }
    }
}

impl help::KeyMap for AppKeyMap {
    fn short_help(&self) -> Vec<&key::Binding> {
        vec![&self.tab, &self.next, &self.prev, &self.quit]
    }
    
    fn full_help(&self) -> Vec<Vec<&key::Binding>> {
        vec![vec![&self.tab, &self.next, &self.prev, &self.quit]]
    }
}

pub struct AutocompleteModel {
    text_input: textinput::Model,
    help: help::Model,
    keymap: AppKeyMap,
    quitting: bool,
}

impl AutocompleteModel {
    fn new() -> Self {
        let mut text_input = textinput::new();
        text_input.set_placeholder("repository");
        text_input.prompt = "charmbracelet/".to_string();
        text_input.prompt_style = Style::new().foreground(Color::from("63"));
        text_input.cursor.style = Style::new().foreground(Color::from("63"));
        text_input.set_char_limit(50);
        text_input.set_width(20);
        
        Self {
            text_input,
            help: help::Model::new(),
            keymap: AppKeyMap::default(),
            quitting: false,
        }
    }
}

impl Model for AutocompleteModel {
    fn init() -> (Self, Option<Cmd>) {
        let mut model = Self::new();
        // Focus the input so it immediately receives typed characters
        let focus_cmd = model.text_input.focus();
        // Start HTTP request for repos and trigger an immediate first render
        (model, Some(batch(vec![fetch_repos(), init_render_cmd(), focus_cmd])))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Initial render trigger; no state change required.
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            return None;
        }

        // HTTP results: set suggestions on textinput
        if let Some(GotReposMsg(repos)) = msg.downcast_ref::<GotReposMsg>().cloned() {
            let suggestions: Vec<String> = repos.into_iter().map(|r| r.name).collect();
            self.text_input.set_suggestions(suggestions);
            return None;
        }
        
        if let Some(GotReposErrMsg(err)) = msg.downcast_ref::<GotReposErrMsg>().cloned() {
            // Keep running; just show no suggestions and append an error note in the view
            eprintln!("error fetching repos: {}", err);
            return None;
        }

        // Check if this is a quit key. We still delegate to the text input first
        // so the widget can consume it if needed (e.g., accept suggestion on Enter),
        // then we decide whether to quit.
        let should_quit = if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            self.keymap.quit.matches(key_msg)
        } else {
            false
        };

        // Delegate ALL messages to textinput so typing, navigation, and Tab
        // completion work correctly inside the widget.
        let text_input_result = self.text_input.update(msg);

        // If this was a quit key, quit after textinput handled it
        if should_quit {
            self.quitting = true;
            return Some(quit());
        }

        text_input_result
    }

    fn view(&self) -> String {
        if self.quitting {
            return String::from("");
        }

        let mut result = format!(
            "Pick a Charm™ repo:\n\n  {}",
            self.text_input.view()
        );

        // Show suggestions dropdown like the Go version. We highlight the
        // currently selected suggestion and cap the visible list to 8.
        let matched_suggestions = self.text_input.matched_suggestions();
        if !matched_suggestions.is_empty() {
            let max_show = 8.min(matched_suggestions.len());
            let current_index = self.text_input.current_suggestion_index();
            
            for (i, suggestion) in matched_suggestions.iter().take(max_show).enumerate() {
                if i == current_index {
                    // Highlight selected suggestion
                    let selected_style = Style::new().foreground(Color::from("205"));
                    result.push_str(&format!("\n> {}", selected_style.render(suggestion)));
                } else {
                    result.push_str(&format!("\n  {}", suggestion));
                }
            }
            
            if matched_suggestions.len() > max_show {
                result.push_str(&format!("\n  ... and {} more", matched_suggestions.len() - max_show));
            }
        }

        result.push_str(&format!("\n\n{}\n\n", self.help.view(&self.keymap)));
        result
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