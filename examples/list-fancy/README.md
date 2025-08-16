# List Fancy Example

<img width="1200" src="./list-fancy.gif" />

This example demonstrates advanced list functionality with the `bubbletea-widgets` List component, closely matching the Go Bubble Tea `list-fancy` example.

## Features

### Interactive Toggles
- **`s`** - Toggle spinner on/off
- **`T`** - Toggle title bar visibility (also toggles filtering)
- **`S`** - Toggle status bar visibility  
- **`P`** - Toggle pagination visibility
- **`H`** - Toggle help menu visibility

### Item Management
- **`a`** - Add a random grocery item to the top of the list
- **`Enter`** - Choose the selected item (shows status message)
- **`x` or `Backspace`** - Delete the selected item

### Navigation
- **`↑/↓` or `j/k`** - Navigate up/down through items
- **`/`** - Start filtering (when title bar is visible)
- **`Esc`** - Clear filter or quit if not filtering
- **`q`** - Quit application
- **`Ctrl+C`** - Force quit

## Visual Features

### Styling
- **Title**: White text on green background (#25A065)
- **Status messages**: Green text (#04B575)
- **App padding**: 1 unit vertical, 2 units horizontal

### Items Display
- Each grocery item shows both title and description
- Items are displayed as 2-line entries with spacing
- Selected item is highlighted with a bullet point

### Random Item Generator
- Cycles through 66+ grocery items and 30+ descriptive phrases
- Items are shuffled on startup for variety
- Combines titles and descriptions randomly

## Implementation Details

- Uses `bubbletea-widgets::list::List` with custom `ItemDelegate`
- Implements proper key binding system with help text
- Thread-safe random item generation using `Arc<Mutex<>>`
- Status messages with automatic timeout
- Proper window resizing support
- Toggle functionality matches Go version exactly

## Running

```bash
cargo run --example list-fancy
```

Or from the example directory:

```bash
cargo run
```

## Architecture

The example demonstrates:
- **Custom Item Types**: `GroceryItem` with title and description
- **Custom Delegates**: `FancyDelegate` with action handling
- **Key Binding Management**: Semantic key mappings with help text
- **State Management**: Toggle states and dynamic UI updates
- **Async Commands**: Status messages and item generation
- **Styling Integration**: lipgloss-extras for consistent theming

This example serves as a comprehensive reference for building feature-rich list interfaces with bubbletea-rs and bubbletea-widgets.