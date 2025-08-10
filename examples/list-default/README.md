# List Default Example

A direct Rust port of the Go Bubble Tea `list-default` example, demonstrating the List component from `bubbletea-widgets`.

## Features

- **Interactive List**: Navigate through items with arrow keys or vim-like `j`/`k` keys
- **Filtering**: Press `/` to filter items in real-time
- **Clean Filter Clearing**: Single `Escape` press to clear filters (using v0.1.6 API)
- **Quit Options**: Press `q` or `Ctrl+C` to quit (when not filtering)
- **Responsive Design**: Automatically adapts to terminal size changes

## Usage

```bash
cargo run --bin list-default
```

### Key Bindings

- `↑`/`k` - Move up
- `↓`/`j` - Move down  
- `/` - Enter filter mode
- `Escape` - Clear filter (if active) or quit
- `q` - Quit (when not filtering)
- `Ctrl+C` - Force quit

## Implementation Notes

This example demonstrates the enhanced List API introduced in `bubbletea-widgets` v0.1.6:

- `list.is_filtering()` - Check if filtering is active
- `list.clear_filter()` - Programmatically clear filters
- `list.filter_state_info()` - Get detailed filter state information

The implementation prioritizes clean, maintainable code using the proper API instead of workarounds.

## Testing

Run the integration tests:

```bash
cargo test
```