# Table Resize Example

This example demonstrates how to create a responsive table with proper viewport management using Bubble Tea and lipgloss in Rust. It solves common table rendering issues where headers get cut off or tables don't display properly in constrained terminal sizes.

## What This Example Shows

- ✅ **Always-visible headers**: Table headers remain at the top regardless of terminal size
- ✅ **Responsive design**: Table adapts to terminal resizing without breaking
- ✅ **Scroll navigation**: Navigate through large datasets with keyboard controls
- ✅ **Proper viewport management**: Manual control over which rows are visible
- ✅ **Status indicators**: Shows current position and navigation help

## The Problem We Solved

### Original Issues
When working with tables in terminal UIs, you often encounter these problems:

1. **Header Cut-off**: Headers disappear when terminal is too small
2. **Bottom-first Rendering**: Table shows bottom rows instead of top rows
3. **No Navigation**: Can't scroll through data that doesn't fit
4. **Poor Responsiveness**: Table breaks when terminal is resized

### Common Failed Approaches
```rust
// ❌ This doesn't work reliably
.height(terminal_height - 2)  // Headers still get cut off
.offset(0)                    // Doesn't guarantee top visibility
```

## Our Solution: Manual Viewport Management

Instead of relying on lipgloss's automatic height management, we implement manual viewport control:

```rust
// Calculate available space
let available_height = (self.height - 4).max(3) as usize;
let max_visible_rows = available_height.saturating_sub(1);

// Manually slice the data
let visible_rows = if self.rows.len() <= max_visible_rows {
    self.rows.clone()  // Show all if it fits
} else {
    // Show subset based on scroll position
    let start_idx = self.scroll_offset;
    let end_idx = (start_idx + max_visible_rows).min(self.rows.len());
    self.rows[start_idx..end_idx].to_vec()
};
```

## Key Technologies

### Bubble Tea (bubbletea-rs)
Bubble Tea is a TUI framework based on The Elm Architecture, using the Model-View-Update pattern:

- **Model**: Application state (our `AppModel` struct)
- **View**: Renders the current state to a string 
- **Update**: Processes messages and updates state

```rust
impl Model for AppModel {
    fn init() -> (Self, Option<Cmd>) { /* Initialize */ }
    fn update(&mut self, msg: Msg) -> Option<Cmd> { /* Handle events */ }
    fn view(&self) -> String { /* Render UI */ }
}
```

### lipgloss-extras
lipgloss is a library for styling terminal layouts with a declarative API:

```rust
use lipgloss_extras::lipgloss::{Color, Style, thick_border};
use lipgloss_extras::table::{Table, HEADER_ROW};

// Create styled table
let table = Table::new()
    .headers(vec!["Column 1", "Column 2"])
    .rows(data)
    .border(thick_border())
    .width(80)
    .style_func_boxed(Box::new(|row, col| {
        if row == HEADER_ROW {
            Style::new().bold(true).foreground(Color::from("252"))
        } else {
            Style::new().foreground(Color::from("245"))
        }
    }));
```

## Implementation Details

### 1. State Management
```rust
struct AppModel {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    width: i32,
    height: i32,
    scroll_offset: usize,  // 🔑 Key addition for viewport control
    // ... styling data
}
```

### 2. Keyboard Navigation
```rust
match key_msg.key {
    KeyCode::Up | KeyCode::Char('k') => {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }
    KeyCode::Down | KeyCode::Char('j') => {
        // Scroll down with bounds checking
    }
    KeyCode::Home => self.scroll_offset = 0,
    KeyCode::End => /* Jump to bottom */,
}
```

### 3. Responsive Height Calculation
```rust
// Account for:
// - Top border (1 line)
// - Header row (1 line) 
// - Bottom border (1 line)
// - Status line (1 line)
let available_height = (self.height - 4).max(3) as usize;
let max_visible_rows = available_height.saturating_sub(1);
```

### 4. Dynamic Row Selection
Instead of showing all data and letting lipgloss handle truncation:

```rust
// ❌ Old approach - unreliable
.rows(all_data)
.height(some_height)

// ✅ New approach - manual control  
let visible_rows = rows[scroll_offset..scroll_offset + max_visible];
.rows(visible_rows)
// No .height() constraint
```

## Running the Example

```bash
cd examples/table-resize
cargo run
```

### Controls
- **↑** or **k**: Scroll up
- **↓** or **j**: Scroll down  
- **Home**: Jump to top
- **End**: Jump to bottom
- **q** or **Esc**: Quit

## Why This Approach Works

1. **Predictable Behavior**: We control exactly which rows are visible
2. **Header Guarantee**: Header is always in the visible row set
3. **Bounds Safety**: Scroll position is always within valid ranges
4. **Terminal Agnostic**: Works regardless of terminal size
5. **Performance**: Only renders visible rows, not entire dataset

## Styling Features Demonstrated

### Color-coded Type Columns
```rust
// Different colors for Pokemon types
type_colors.insert("Fire".to_string(), Color::from("#FF7698"));
type_colors.insert("Water".to_string(), Color::from("#00E2C7"));
type_colors.insert("Grass".to_string(), Color::from("#75FBAB"));
```

### Row Alternation
```rust
let even = (row + 1) % 2 == 0;
if even {
    Style::new().foreground(Color::from("245"))  // Dimmer
} else {
    Style::new().foreground(Color::from("252"))  // Brighter
}
```

### Special Highlighting
```rust
// Highlight Pikachu row
if rows[row_index][1] == "Pikachu" {
    return Style::new()
        .foreground(Color::from("#01BE85"))
        .background(Color::from("#00432F"));
}
```

## Common Gotchas

### 1. Height Calculation
```rust
// ❌ Don't forget border overhead
.height(terminal_height)

// ✅ Account for all UI elements
let available = (terminal_height - borders - status_line).max(minimum)
```

### 2. Empty Data Handling
```rust
// ❌ Can panic on empty data
let visible = data[start..end];

// ✅ Safe bounds checking
let end = (start + count).min(data.len());
let visible = &data[start..end];
```

### 3. Style Function Scope
```rust
// ❌ Referencing wrong data in closure
.style_func_boxed(Box::new(move |row, col| {
    // This might reference original data, not visible subset
}))

// ✅ Use the same data slice for styling
let visible_rows = filtered_data.clone();
.rows(visible_rows.clone())
.style_func_boxed({
    let rows = visible_rows.clone();  // Same data reference
    Box::new(move |row, col| { /* style logic */ })
})
```

## Learn More

- **Bubble Tea Rust**: [Documentation](https://docs.rs/bubbletea-rs)
- **lipgloss-extras**: [API Reference](https://docs.rs/lipgloss-extras)
- **Original Go Bubble Tea**: [GitHub](https://github.com/charmbracelet/bubbletea)
- **Terminal Styling**: [lipgloss Go docs](https://github.com/charmbracelet/lipgloss)

This example provides a robust foundation for building responsive terminal tables that work reliably across different terminal sizes and handle large datasets gracefully.