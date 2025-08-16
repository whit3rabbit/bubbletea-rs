# Window Size

A demonstration of terminal size detection and responsive layout handling, showing how to build applications that adapt to different terminal dimensions and respond to resize events in real-time.

## Features

- **Real-time Size Detection**: Shows current terminal width and height
- **Resize Event Handling**: Updates display immediately when terminal is resized
- **Initial Size Request**: Requests terminal dimensions on startup
- **Responsive Feedback**: Visual confirmation of dimension changes
- **Simple Exit**: Any key quits the application

## Running the Example

From the repository root:

```bash
cargo run --example window-size
```

**Testing:**
1. Run the example to see current terminal size
2. Resize your terminal window and watch dimensions update
3. Try different terminal sizes to see responsive behavior
4. Press any key to quit

## What this demonstrates

### Key Concepts for Beginners

**Responsive Terminal Applications**: This example shows fundamental patterns for:
1. Detecting initial terminal dimensions
2. Responding to terminal resize events
3. Adapting layout based on available space
4. Building applications that work well in different terminal sizes

**Window Size Events**: Understanding how terminal emulators report size changes and how applications can respond appropriately.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, window_size, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
```

- `window_size()`: Command to request current terminal dimensions
- `WindowSizeMsg`: Message sent when terminal size changes
- Automatic resize event handling by the framework

### Architecture Walkthrough

#### Model Structure
```rust
pub struct WindowSizeModel {
    pub width: u16,     // Terminal width in columns
    pub height: u16,    // Terminal height in rows  
    pub ready: bool,    // Whether initial size received
    pub keys: KeyBindings,
}
```

#### Initial Size Request
```rust
fn init() -> (Self, Option<Cmd>) {
    let model = WindowSizeModel {
        width: 0,
        height: 0,
        ready: false,  // Start with unknown size
        keys: KeyBindings::default(),
    };
    
    // Request current terminal size immediately
    let cmd = window_size();
    (model, Some(cmd))
}
```

#### Size Event Handling
```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    // Handle window size messages (initial and resize events)
    if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
        self.width = size_msg.width;
        self.height = size_msg.height;
        self.ready = true;  // Mark as ready after first size received
        return None;
    }
    
    // Any key press quits
    if let Some(_key_msg) = msg.downcast_ref::<KeyMsg>() {
        return Some(quit());
    }
    
    None
}
```

### Rust-Specific Patterns

**Window Size Message:**
```rust
if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
    self.width = size_msg.width;   // u16 - columns
    self.height = size_msg.height; // u16 - rows
}
```

**Readiness State:**
```rust
if self.ready {
    // Display size information
    format!("Terminal size: {}x{}", self.width, self.height)
} else {
    // Still waiting for initial size
    "Getting terminal size...".to_string()
}
```

**Any Key Quit Pattern:**
```rust
if let Some(_key_msg) = msg.downcast_ref::<KeyMsg>() {
    return Some(quit());  // Don't care which key
}
```

### Display Logic

```rust
fn view(&self) -> String {
    if !self.ready {
        return "Getting terminal size...".to_string();
    }
    
    format!(
        "Terminal size: {}×{} (width×height)\n\n\
         Try resizing your terminal to see this update in real time!\n\
         Press any key to quit.",
        self.width, self.height
    )
}
```

### Event Flow

1. **Program Start**: Model initialized with size 0×0, `window_size()` command sent
2. **Size Response**: Framework sends `WindowSizeMsg` with actual dimensions
3. **Model Update**: Width and height updated, `ready` flag set to true
4. **View Refresh**: Display shows current terminal size
5. **Resize Event**: User resizes terminal
6. **Auto-Detection**: Framework automatically sends new `WindowSizeMsg`
7. **Real-time Update**: Display immediately shows new dimensions

### Terminal Compatibility

**Size Detection Support:**
- ✅ **Modern terminals**: iTerm2, Alacritty, Windows Terminal, GNOME Terminal
- ✅ **Standard terminals**: Most xterm-compatible terminals
- ⚠️ **Limited**: Some very basic terminal emulators

**Resize Event Support:**
- ✅ **Graphical terminals**: Immediate resize detection
- ⚠️ **SSH sessions**: May have slight delays
- ❌ **TTY**: Limited resize support

### Responsive Design Patterns

**Width-based Layout:**
```rust
fn format_content(&self, content: &str) -> String {
    if self.width < 40 {
        // Narrow terminal - simple layout
        format!("Size: {}×{}\n{}", self.width, self.height, content)
    } else if self.width < 80 {
        // Medium terminal - more details
        format!("Terminal: {}×{} ({}×{} characters)\n{}", 
            self.width, self.height, self.width, self.height, content)
    } else {
        // Wide terminal - full layout with extras
        format!("Terminal Dimensions: {} columns × {} rows\n\
                 Total characters: {}\n{}", 
            self.width, self.height, self.width as u32 * self.height as u32, content)
    }
}
```

**Height-based Pagination:**
```rust
fn paginated_content(&self, items: &[String]) -> String {
    let available_height = self.height.saturating_sub(5) as usize; // Leave room for UI
    let visible_items = items.iter()
        .take(available_height)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");
    
    if items.len() > available_height {
        format!("{}\n... and {} more items", visible_items, items.len() - available_height)
    } else {
        visible_items
    }
}
```

### Real-world Applications

**Progress Bar Sizing:**
```rust
fn sized_progress_bar(&self, percent: f64) -> String {
    let bar_width = (self.width as f64 * 0.8) as usize; // 80% of terminal width
    let filled = ((bar_width as f64) * percent) as usize;
    let empty = bar_width - filled;
    
    format!("{}{}",
        "█".repeat(filled),
        "░".repeat(empty)
    )
}
```

**Responsive Tables:**
```rust
fn format_table(&self, data: &[TableRow]) -> String {
    if self.width < 60 {
        // Compact format
        data.iter().map(|row| format!("{}: {}", row.name, row.value)).collect()
    } else {
        // Full table format with borders
        create_full_table(data, self.width as usize)
    }
}
```

**Multi-column Layouts:**
```rust
fn layout_columns(&self, left: &str, right: &str) -> String {
    if self.width < 80 {
        // Stack vertically on narrow terminals
        format!("{}\n\n{}", left, right)
    } else {
        // Side-by-side on wide terminals
        let column_width = (self.width / 2) as usize;
        format_two_columns(left, right, column_width)
    }
}
```

### Performance Considerations

**Resize Frequency:**
- Resize events can fire frequently during window dragging
- Consider debouncing expensive layout recalculations
- Cache layout calculations when possible

**Layout Efficiency:**
```rust
// Cache expensive calculations
if self.last_width != size_msg.width {
    self.cached_layout = recalculate_layout(size_msg.width);
    self.last_width = size_msg.width;
}
```

### Testing Window Behavior

```rust
#[test]
fn test_window_size_handling() {
    let mut model = WindowSizeModel::new();
    
    // Simulate size message
    let size_msg = WindowSizeMsg { width: 80, height: 24 };
    model.update(Box::new(size_msg));
    
    assert_eq!(model.width, 80);
    assert_eq!(model.height, 24);
    assert!(model.ready);
}

#[test]
fn test_responsive_layout() {
    let mut model = WindowSizeModel::new();
    
    // Test narrow layout
    model.width = 40;
    assert!(model.view().len() < 100);
    
    // Test wide layout  
    model.width = 120;
    assert!(model.view().len() > 100);
}
```

## Related Examples

- **[progress-static](../progress-static/)** - Responsive progress bar sizing
- **[glamour](../glamour/)** - Responsive content display
- **[help](../help/)** - Responsive help text formatting

## Files

- `main.rs` — Complete window size detection and responsive handling
- `Cargo.toml` — Dependencies and build configuration
- `README.md` — This documentation

## Implementation Tips

- Always request initial window size in `init()`
- Handle both initial size and resize events the same way
- Design layouts that work well at different terminal sizes
- Test with very narrow (40 cols) and very wide (120+ cols) terminals
- Consider mobile/limited terminals in your responsive design
- Cache expensive layout calculations to handle frequent resize events efficiently