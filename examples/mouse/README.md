# Mouse Example

A simple program that demonstrates mouse event handling in Bubble Tea for Rust.

This example opens the alternate screen buffer and displays mouse coordinates and events as you interact with the terminal.

## Features

- Tracks all mouse motion events
- Displays mouse coordinates (X, Y)
- Shows mouse event types (press, release, drag, motion, scroll)
- Shows modifier keys (ctrl, alt, shift)
- Supports different mouse buttons (left, right, middle)

## Usage

```bash
cargo run --bin mouse
```

Once running:
- Move your mouse around the terminal to see motion events
- Click and drag to see press, drag, and release events
- Use the scroll wheel to see scroll events
- Hold modifier keys while clicking to see modifier combinations
- Press 'q', 'Esc', or 'Ctrl+C' to quit

## Implementation

This example is a Rust port of the original Go Bubble Tea mouse example. It demonstrates:

- Enabling mouse motion tracking with `MouseMotion::All`
- Handling `MouseMsg` events in the update function
- Formatting mouse events for display
- Using the `printf` command to display output without clearing the screen

The implementation closely mirrors the Go version while utilizing Rust's type safety and crossterm's mouse event types.