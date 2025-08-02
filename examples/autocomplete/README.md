# Autocomplete (Rust)

This example demonstrates how to build an interactive autocomplete UI in the terminal using `bubbletea-rs`.

What it does:
- Fetches repository names from the Charmbracelet GitHub organization
- Shows a text input with a fixed prompt `charmbracelet/`
- Displays a navigable suggestions dropdown
- Allows accepting a suggestion with Tab
- Quits with Esc, Enter, or Ctrl+C

Controls:
- Type to filter suggestions
- Tab: accept the current suggestion
- Ctrl+N / Ctrl+P: move selection down/up
- Esc / Enter / Ctrl+C: quit

## GitHub API usage

This example fetches repository names from the GitHub API endpoint:

- `https://api.github.com/orgs/charmbracelet/repos`

By default, the request is unauthenticated. Unauthenticated requests are subject to lower rate limits, which can result in errors or an empty list if you exceed the limit. If that happens, wait and try again, or enable authentication.

Optional authentication with a GitHub token:
1) Create a classic Personal Access Token (no special scopes needed for public repos) from your GitHub settings.
2) Provide it to the example via an environment variable when running:

```bash
export GITHUB_TOKEN="<your-token>"
cargo run
```

3) To wire this into the example, add the following snippet where headers are set in `fetch_repos()` (look for the `HeaderMap` in `main.rs`):

```rust
use reqwest::header::AUTHORIZATION;
if let Ok(token) = std::env::var("GITHUB_TOKEN") {
    headers.insert(
        AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );
}
```

With a token, you’ll get higher rate limits and more reliable results.

---

## Run the example locally

You can run this example directly from the `examples/autocomplete` directory since it’s a standalone Cargo binary.

```bash
# From the repository root
cd examples/autocomplete
cargo run
```

Alternatively, if your editor or workspace is at the repository root and supports multiple Cargo projects, you can also run it via the binary name defined in `examples/autocomplete/Cargo.toml` (bin name: `autocomplete`).

---

## Integrate bubbletea-rs into your Rust project

This section is for users who are new to `bubbletea-rs` and want to integrate it into their own application.

### 1) Add dependencies

Depending on how you obtain `bubbletea-rs`, you have a few options for your `Cargo.toml`:

- If it’s published on crates.io (recommended when available):

```toml
[dependencies]
bubbletea-rs = "<latest>"          # e.g. "0.x"
```

- If you want to use the Git repository directly:

```toml
[dependencies]
# Replace the URL with the actual repository URL for bubbletea-rs
bubbletea-rs = { git = "https://github.com/<org-or-user>/bubbletea-rs" }
```

- If you’re working in a monorepo and want to use a local path:

```toml
[dependencies]
bubbletea-rs = { path = "../path/to/bubbletea-rs" }
```

In addition, you’ll typically need these supporting crates for TUI apps and async/networking (as used by this example):

```toml
[dependencies]
# Terminal backend
crossterm = "0.27"

# Async runtime
 tokio = { version = "1", features = ["full"] }

# Optional: HTTP client (if your app fetches data)
reqwest = { version = "0.11", features = ["json", "rustls-tls"], default-features = false }

# Optional: Serialization (for JSON APIs)
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Optional: For correct display width in terminals
unicode-width = "0.1"
```

Adjust versions as needed to match your project constraints.

### 2) Core concepts in bubbletea-rs

- Model: Implement the `Model` trait on your application state. It defines:
  - `init() -> (Self, Option<Cmd>)`: return initial state and optional startup command(s)
  - `update(&mut self, msg: Msg) -> Option<Cmd>`: handle input/events and optionally return commands to run
  - `view(&self) -> String`: render your UI to a `String`

- Msg: An “any”-style message. You can downcast to your own message types (e.g., key events, HTTP results).
- Cmd: An asynchronous command you can schedule to run; its result becomes a `Msg` on completion. Use `bubbletea_rs::command::batch` to run multiple commands at once.
- Program: The app runner. Create and configure with `Program::<YourModel>::builder()`, then `run().await`.

### 3) Minimal shape of a Bubble Tea app (Rust)

The rough flow looks like this:
1. Define your `Model` and implement `init`, `update`, and `view`.
2. In `main`, build a `Program` with options like `alt_screen(true)` and `signal_handler(true)`.
3. Call `program.run().await` inside a `tokio::main` async function (if you need async).

The `examples/autocomplete/main.rs` file shows:
- Starting an async HTTP request in `init`
- Downcasting to handle keyboard input via `KeyMsg`
- Updating state and re-rendering when messages arrive
- Returning `quit()` when it’s time to exit

---

## Adapting the autocomplete logic

Key building blocks you can reuse:
- A fixed prompt (e.g., `"charmbracelet/"`) and a mutable `input` string
- A vector of all suggestions (fetched or computed) and a filtered subset based on `input`
- A selected index to track which suggestion is active
- Keyboard handlers for:
  - Text entry (update `input` and re-filter)
  - Tab to accept the current suggestion
  - Ctrl+N / Ctrl+P to cycle through suggestions
  - Esc/Enter/Ctrl+C to quit

You can replace the GitHub fetch with your own data source and keep the filtering/selection logic.

---

## Troubleshooting

- GitHub API rate limits: Unauthenticated requests are limited. If you see errors or an empty list, try again later or add authentication headers.
- Terminal support: Ensure your terminal supports alternate screen and standard key events. If keybindings don’t behave as expected, check your terminal emulator and `crossterm` version.
- Windows/WSL: `crossterm` is cross-platform, but key handling can vary by environment. Test the example in your target environment.

---

## See also

- Source code: `examples/autocomplete/main.rs`
- Other examples in this repository for different patterns and features
