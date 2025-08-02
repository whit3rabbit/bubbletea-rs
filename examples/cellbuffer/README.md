# Cellbuffer Example (bubbletea-rs)

A Rust port of Bubble Tea’s cellbuffer example. It draws an animated ellipse on a simple cell buffer and uses a spring to smoothly follow the mouse position.

## Run

From the repo root:

```bash
cargo run -p cellbuffer-example --bin cellbuffer
```

Quit with Esc, Enter, or Ctrl+C.

## What it demonstrates
- A minimal cell-based drawing buffer rendered each frame
- A critically-damped spring integrator for smooth motion
- Mouse motion input (cell-based) to move the ellipse target
- Window-size handling to (re)initialize the buffer

## Terminal notes (Alt Screen and Mouse)
This example enables the terminal’s alternate screen buffer by default. Many terminals only provide full mouse reporting (including motion) while in the alternate screen.

- Alt screen is enabled in code via:
  ```rust
  Program::builder().alt_screen(true)
  ```
- If you need to debug visibility, you can temporarily set `.alt_screen(false)` and add `println!` logging.

### Mouse motion modes
The example enables cell-based mouse motion:
```rust
builder.mouse_motion(MouseMotion::Cell)
```
It listens for these events so it works across different terminals:
- `MouseEventKind::Moved`
- `MouseEventKind::Drag(_)`
- `MouseEventKind::Down(_)`

### Terminal-specific tips
- macOS Terminal / iTerm2
  - Usually work out-of-the-box with alt screen + cell mouse motion.
- Ghostty
  - Works best with alt screen enabled.
  - Ensure mouse reporting is enabled in preferences (e.g., “Report mouse movements” / “Report drag”).
  - If motion doesn’t update, try dragging while moving the pointer. This example also reacts to Drag / Down events.
- tmux
  - Add `set -g mouse on` to `~/.tmux.conf` and reload.
  - Use a recent tmux for best mouse support.

## Behavior details
- Initial center: The ellipse starts near the center. If the window size isn’t known immediately, the model uses a default (80×24) and re-centers after the first `WindowSizeMsg`.
- Spring physics: `frequency` and `damping` values are tuned to feel similar to the Go example.
- Frame rate: The example ticks at ~60 FPS.

## Troubleshooting
- “Nothing displays” or no motion:
  - Run in a real terminal (not an editor’s integrated console).
  - Ensure alt screen is enabled (default).
  - Make sure mouse reporting is enabled in your terminal settings.
  - In tmux, enable mouse support.
- “Layout looks wrong after resize”:
  - The buffer re-initializes on `WindowSizeMsg`; ensure your terminal sends resize events.

## Source
This example is a Rust port of the Bubble Tea Go example at:
`bubbletea/examples/cellbuffer`.
