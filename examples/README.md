# bubbletea-rs Examples

This directory contains runnable examples for bubbletea-rs. A few tips will help you get the best experience across terminals.

## How to run

From the project root:

```bash
# List examples (cargo-style)
cargo run --example <name>

# Or run an example packaged as its own crate
cargo run -p <package> --bin <bin>

# Cellbuffer example in this repo
cargo run -p cellbuffer-example --bin cellbuffer
```

Most examples support quitting with one or more of: Esc, Enter, or Ctrl+C.

## Alternate screen (Alt Screen)

Many TUIs use the terminal's alternate screen buffer for a clean, full-screen UI that restores the original screen on exit.

- In bubbletea-rs, enable with `Program::builder().alt_screen(true)`.
- Some terminals only send full mouse reporting when in alt screen.
- If you are debugging display issues, temporarily set `.alt_screen(false)` to see printed debug output in the main buffer.

## Mouse support and motion modes

bubbletea-rs supports mouse input via crossterm. Examples that react to mouse motion should:

- Enable motion mode in the program builder: `builder.mouse_motion(MouseMotion::Cell)`.
- Handle `MouseMsg` in `update`:
  - Many terminals emit `MouseEventKind::Moved` for hover/move.
  - Some emit `MouseEventKind::Drag(_)` only while a button is pressed.
  - Some emit `MouseEventKind::Down(_)` on press without separate move events.

For best compatibility, examples handle Moved, Drag(_), and Down(_).

### Terminal-specific notes

- macOS Terminal and iTerm2
  - Generally work well with alt screen and cell-based mouse motion.
  - If you see no updates, ensure the app is running in a real terminal (not an embedded IDE console).

- Ghostty
  - Works best with alt screen enabled.
  - Ensure mouse reporting is enabled in preferences (e.g., "Report mouse movements" / "Report drag").
  - Some builds only send motion while dragging; our examples also listen for Drag/Down events.

- tmux
  - Enable mouse in tmux: add `set -g mouse on` to your `~/.tmux.conf` and reload.
  - Prefer a recent tmux version; older versions can have quirks with mouse tracking.

- SSH / Remote sessions
  - Mouse reporting depends on the remote terminal and multiplexer configuration.
  - Make sure `$TERM` is set to a terminfo that supports mouse (e.g., `xterm-256color`).

## Window size detection

Examples that need layout sizing typically request the window size on init and react to `WindowSizeMsg`:

- Use `command::window_size()` on startup.
- Update internal buffers on `WindowSizeMsg`.
- Some examples initialize with a safe default size (e.g., 80x24) so something renders immediately, then update when the real size arrives.

## Troubleshooting checklist

- Nothing displays:
  - Try `.alt_screen(false)` temporarily to see debug text.
  - Confirm you’re in a real terminal (Terminal/iTerm2/Ghostty), not an editor console.
  - Run `cargo check` to ensure the example compiles without errors.

- Mouse doesn’t move animations:
  - Enable alt screen.
  - Ensure terminal mouse reporting is enabled (see terminal-specific notes).
  - If in tmux, enable mouse and use a recent tmux.

- Layout looks wrong after resize:
  - Ensure your example listens to `WindowSizeMsg` and re-initializes any internal buffers.

## Contributing to examples

- Keep examples small, focused, and runnable with a single command.
- Favor idiomatic Rust and parity with upstream Bubble Tea examples when porting from Go.
- Add brief comments and a short usage section when behavior may be surprising (e.g., spring physics, mouse modes).
