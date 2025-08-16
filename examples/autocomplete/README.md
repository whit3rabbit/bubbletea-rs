# Autocomplete (Rust)

<img width="1200" src="./autocomplete.gif" />

Interactive autocomplete UI in the terminal using `bubbletea-rs`.

## What it does
- Fetches repository names from the Charmbracelet GitHub organization
- Shows a text input with a fixed prompt `charmbracelet/`
- Displays a navigable suggestions dropdown
- Accept the current suggestion with Tab
- Quit with Esc, Enter, or Ctrl+C

## How it works
At a high level this follows the Bubble Tea MVU pattern:

1) Model implements `init`, `update`, and `view`.
2) `init` fires three commands in a batch:
   - Fetch GitHub repos asynchronously
   - Focus the text input so typing works immediately
   - Emit a synthetic init-render message to paint the first frame
3) `update` handles three kinds of messages:
   - HTTP results: convert JSON payload into a suggestions list on the text input
   - Key input: we first detect app-level quit keys, but always delegate to the text input model so editing/navigation works, then quit if appropriate
   - Init-render: a no-op that just forces the first draw quickly
4) `view` renders the input and a capped list (up to 8) of matched suggestions, highlighting the selected row.

The text field, key bindings, and help are provided by `bubbletea-widgets`.

## Key bindings
- Type: filter suggestions based on your input
- Tab: accept the current suggestion
- Ctrl+N / Ctrl+P: move selection down/up
- Esc / Enter / Ctrl+C: quit

## Running locally

```bash
# From the repository root
cd examples/autocomplete
cargo run
```

## GitHub API usage
The example fetches from `https://api.github.com/orgs/charmbracelet/repos`.

Unauthenticated requests have lower rate limits. If you hit an error or get no suggestions, wait a bit and try again—or add a token:

```bash
export GITHUB_TOKEN="<your-token>"
cargo run
```

Then, in `fetch_repos()` you can add:

```rust
use reqwest::header::AUTHORIZATION;
if let Ok(token) = std::env::var("GITHUB_TOKEN") {
    headers.insert(
        AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );
}
```

## What we learned getting it working
- Single crate instance matters for downcasting: Messages use `Box<dyn Any>`, so `downcast_ref::<KeyMsg>()` only succeeds if the `KeyMsg` type comes from the exact same `bubbletea-rs` crate instance as the widget. If the workspace pulls in a separate `bubbletea-rs` (e.g., via a transitive dependency), typed input can appear to be ignored. We fixed this by pinning the workspace to a single crate instance using a `[patch.crates-io]` entry in the workspace `Cargo.toml`:

```toml
[patch.crates-io]
bubbletea-rs = { path = "." }
```

- Focus early: Call `text_input.focus()` in `init` so typing is immediately captured.
- Render quickly: Send a small synthetic message right after startup to force the first draw, preventing a blank screen while the HTTP request is in flight.

## Integrate into your project
Core pieces you’ll likely reuse:
- Fixed prompt and mutable `input` string
- Full list of suggestions + filtered list based on `input`
- Current suggestion index + Tab to accept
- Delegation of all messages to your input widget inside `update`

See `examples/autocomplete/main.rs` for a complete reference.
