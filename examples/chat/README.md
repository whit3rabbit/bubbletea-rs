# Chat Example (bubbletea-rs)

A Rust port of Bubble Tea’s chat example using a minimal viewport + textarea implementation. It demonstrates:

- A scroll-to-bottom message viewport
- A simple input line with prompt and character limit
- Width-aware wrapping on the viewport content
- Resize handling to keep layout consistent

## Run

From the repo root:

```bash
cargo run -p chat-example --bin chat
```

Quit with Esc or Ctrl+C (prints the current input before exiting, matching the Go example).

## Layout

The layout mirrors the Go example:

```
+-------------------- viewport (fills available height) --------------------+
| [wrapped messages, width-aware]                                           |
| ...                                                                       |
+---------------------------------------------------------------------------+


┃ [input line with placeholder or text]
```

- The viewport height is computed as: `window_height - textarea_height - gap_height`.
- `gap` is two newlines ("\n\n") between the viewport and the textarea.
- Width is shared: both viewport and textarea use the terminal width.

## Resizing

On startup, we request the terminal window size and then listen for `WindowSizeMsg` to recompute sizes:

1) Update viewport width and textarea width to `msg.width`.
2) Compute viewport height: `msg.height - textarea.height - gap_height`.
3) Re-wrap message content to the new viewport width.
4) Keep the viewport scrolled to the bottom.

This follows the Go example’s flow: adjusting widths and heights and then wrapping with Lip Gloss’ width setting before calling `viewport.SetContent`.

## Width-aware wrapping

In the Go example, wrapping is applied with `lipgloss.NewStyle().Width(vp.Width)`. In this Rust port we implement a simple word-wrapping helper that:

- Splits on whitespace
- Wraps at the current viewport width
- Hard-wraps long words that exceed the width
- Preserves line breaks in the source strings

We then take only the bottom-most lines that fit in the viewport height to emulate a "stick-to-bottom" behavior.

If you replace this wrapper with a preferred text layout library (or a Lip Gloss port), wire it into `recompute_viewport_content`.

## Input handling

- Enter sends the message as `"You: " + input`, appends to the model’s `messages`, re-wraps and scrolls to bottom.
- Backspace deletes a character.
- Ctrl+C or Esc: prints the current input and quits (like the Go example).

## Styling

This Rust example renders:

- A left prompt: `┃ ` (always visible)
- A placeholder: `Send a message...` shown when input is empty, styled in dark gray for subtlety
- The sender label `You:` in a bright pink approximation (magenta + bold) when messages are sent

Notes
- We currently use `crossterm` ANSI styling for colors.
- TODO(lipgloss): Port these styles to a Lip Gloss-compatible approach for exact parity with the Go example.

## Alt screen

We enable alt screen for a clean UI. If debugging, you can disable it in `main()` and use println! statements.

## Extending this example

- Styling: Add a Lip Gloss equivalent for `senderStyle` (magenta/purple) and apply it to the `"You: "` label.
- True viewport behavior: The current version truncates to the last N wrapped lines equal to the viewport height. You can store a larger buffer and render a slice based on scroll position to support "PageUp/PageDown" etc.
- Cursor/caret: Enhance the textarea to show a cursor and support multi-line input and keybindings, similar to Bubbles’ `textarea`.
- Tests: Extract wrap logic and add unit tests to validate edge cases (long words, unicode width, etc.).

---

## Using this example in your own app

This folder is self-contained. To adapt it:

1) Copy the minimal `TextArea` and `Viewport` structs from `main.rs` into your project.
2) Keep the layout math: render `viewport`, then a blank `GAP` (two newlines), then `textarea`.
3) Handle `WindowSizeMsg` to update widths/heights and re-wrap content.
4) Customize the placeholder, prompt, and colors (currently via `crossterm`).
5) Replace the simple wrapper with your preferred text layout if needed.

This README and the code in this directory only describe and implement this chat example.

---

## How it works (for new Rustaceans)

This example implements the classic Bubble Tea architecture in Rust with three core pieces:

1) Model (state)
- `ChatModel` holds the `Viewport`, `TextArea`, and a `Vec<String>` of messages.

2) Update (state transitions)
- The `update(&mut self, msg: Msg)` method handles:
  - Window resize (`WindowSizeMsg`): updates widths/heights and re-wraps the content.
  - Key input (`KeyMsg`): Enter to send, Backspace to delete, Esc/Ctrl+C to quit.
  - A periodic `BlinkMsg` to toggle the cursor visibility.

3) View (render)
- `view(&self)` returns a `String` that is printed to the terminal each frame.
- We compose: `viewport.view() + GAP + textarea.view()`.

Under the hood, `Program::<ChatModel>` drives the event loop: it dispatches terminal events as messages, calls `update`, and re-renders using `view`.

### The mini TextArea and Viewport

- `TextArea` is a focused single-line input with:
  - `placeholder: "Send a message..."` (shown in dark gray when empty)
  - `prompt: "┃ "` (always visible on the left)
  - `char_limit: 280`
  - A blinking block cursor when focused

- `Viewport` holds the wrapped chat messages. We wrap lines to the current width and keep the bottom-most lines visible to emulate a “stick to bottom” chat.

### Resizing and wrapping

On `WindowSizeMsg`, we:
1) Set `viewport.width = msg.width` and `textarea.set_width(msg.width)`
2) Compute `viewport.height = msg.height - textarea.height - GAP_height`
3) Re-wrap and keep the viewport at the bottom

### Styling

- We use `crossterm::style::Stylize` for simple colors:
  - Placeholder in dark gray
  - Sender label `"You: "` in a bright pink approximation (`magenta().bold()`).
- TODO(lipgloss): Replace ANSI styling with Lip Gloss-style theming to match the Go example exactly (e.g., `lipgloss.NewStyle().Foreground("5")`).

---

## Integrate this pattern into your app

1) Add dependencies (example):

```toml
[dependencies]
bubbletea-rs = { path = "../../" }
crossterm = "0.27"
tokio = { version = "1", features = ["full"] }
```

2) Define your model and implement `Model`:

```rust
use bubbletea_rs::{Model, Msg, Cmd};

struct MyModel { /* your fields */ }

impl Model for MyModel {
    fn init() -> (Self, Option<Cmd>) {
        (Self { /* init fields */ }, None)
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // match on WindowSizeMsg, KeyMsg, timers, network, etc.
        None
    }

    fn view(&self) -> String {
        // return the full screen as a String
        "Hello".into()
    }
}
```

3) Start the program (async main with Tokio):

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use bubbletea_rs::Program;
    let program = Program::<MyModel>::builder()
        .alt_screen(true)
        .signal_handler(true)
        .build()?;
    program.run().await?;
    Ok(())
}
```

4) Reuse pieces from the chat example
- Copy the simple `TextArea` and `Viewport` structs (or adapt them) into your project.
- Keep the `GAP` and layout math to split the screen between output and input.
- Wire up `WindowSizeMsg` to recompute widths/heights and update wrapped content.

5) Customize it
- Change `placeholder`, `prompt`, and `char_limit` as needed.
- Style with `crossterm` now; migrate to Lip Gloss later for richer theming (see TODO above).
- Extend `Viewport` to support real scrolling and history.

### Keybindings summary

- Enter: send the message (`"You: " + input`)
- Backspace: delete last char
- Esc or Ctrl+C: quit (prints current input before exit)

### Terminal support

The example uses ANSI styling via `crossterm`. Most modern terminals support it. If colors don’t show as expected, check your terminal’s color profile and ensure ANSI colors are enabled.
