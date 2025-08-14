# Prevent Quit Example

A program demonstrating how to prevent users from quitting when there are unsaved changes.

## Features

- Multi-line text editing using `bubbletea-widgets::textarea`
- Unsaved changes detection 
- Quit confirmation dialog when attempting to exit with unsaved changes
- Save functionality that clears the "unsaved changes" flag
- Styled UI with lipgloss-extras matching the original Go example

## Key Bindings

- **Ctrl+S**: Save changes
- **Esc/Ctrl+C**: Attempt to quit (shows confirmation if changes exist)
- **Y**: Confirm quit when prompted
- **Any other key**: Cancel quit when prompted

## Running

```bash
cargo run --example prevent-quit
```

## Implementation Notes

This Rust implementation achieves quit prevention through model state management rather than the Go version's message filtering approach. The `quitting` boolean flag and `has_changes` tracking provide the same user experience while being more idiomatic in Rust.

The original Go version uses `tea.WithFilter()` to intercept `QuitMsg` events, while this Rust version handles quit attempts directly in the update logic by transitioning to a confirmation state.