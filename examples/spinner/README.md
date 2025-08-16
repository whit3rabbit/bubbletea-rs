# Spinner

<img width="1200" src="./spinner.gif" />

A simple loading spinner demonstration showing animated characters, timing control, and visual feedback patterns for indicating ongoing operations in terminal applications.

## Features

- **Dot Spinner Animation**: Classic Braille dot pattern rotating animation
- **100ms Frame Rate**: Smooth 10fps animation for optimal visual feedback  
- **Pink Styling**: Colored spinner matching Charm's design system
- **Keyboard Quit**: Exit with `q`, `Ctrl+C`, or `Esc`
- **Error Handling**: Demonstrates error message display patterns
- **Continuous Animation**: Runs indefinitely until user quits

## Running the Example

From the repository root:

```bash
cargo run --example spinner
```

**Controls:**
- `q` / `Ctrl+C` / `Esc` - Quit
- Spinner runs continuously until quit

## What this demonstrates

### Key Concepts for Beginners

**Loading Indicators**: Spinners provide visual feedback for:
- Long-running operations
- Network requests
- File processing
- Background tasks
- "Something is happening" feedback

**Animation Patterns**: Shows how to create smooth character-based animations using timer-driven frame updates.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
```

- `tick()`: Single-shot timers for animation frames
- Standard MVU pattern with custom timing
- Keyboard input handling patterns

**Styling:**
```rust
use lipgloss_extras::lipgloss::{Color, Style};
```

- `Color::from("#205")`: Hex color specification
- `Style::new().foreground()`: Text coloring

### Architecture Walkthrough

#### Model Structure
```rust
pub struct SpinnerModel {
    current_frame: usize,  // Current position in animation sequence
    quitting: bool,        // Exit state tracking
    err: Option<String>,   // Error message display
}
```

#### Animation System

The spinner uses a frame-based animation approach:

```rust
// Dot spinner frames (Braille Unicode characters)
fn frames() -> &'static [&'static str] {
    &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
}

// Animation timing
fn interval() -> Duration {
    Duration::from_millis(100)  // 10fps animation
}
```

#### Frame Update Logic

```rust
if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
    // Advance to next frame (wrap around at end)
    self.current_frame = (self.current_frame + 1) % Self::frames().len();
    
    // Schedule next frame update
    return Some(tick(Self::interval(), |_| {
        Box::new(SpinnerTickMsg) as Msg
    }));
}
```

#### Styled Rendering

```rust
fn current_frame(&self) -> String {
    let style = Style::new().foreground(Color::from("#205"));  // Pink color
    style.render(Self::frames()[self.current_frame])
}
```

### Rust-Specific Patterns

**Frame Wrapping:**
```rust
self.current_frame = (self.current_frame + 1) % Self::frames().len();
//                                             ^ prevents index out of bounds
```

**Static Frame Arrays:**
```rust
fn frames() -> &'static [&'static str] {
    &["⠋", "⠙", "⠹", ...] // Compile-time constant, efficient memory usage
}
```

**Multiple Quit Keybindings:**
```rust
match key_msg.key {
    KeyCode::Char('q') => return Some(quit()),
    KeyCode::Esc => return Some(quit()),
    KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
        return Some(quit());
    }
    _ => {}
}
```

**Single-Shot Timer Pattern:**
```rust
// GOOD: Re-arm timer each frame
tick(Duration::from_millis(100), |_| Box::new(SpinnerTickMsg) as Msg)

// AVOID: Continuous timer can accumulate
// every(Duration::from_millis(100), |_| ...)
```

### Animation Mathematics

**Frame Rate Calculation:**
```
100ms per frame = 10 frames per second = 10fps
Total cycle time = 10 frames × 100ms = 1 second per complete rotation
```

**Unicode Braille Dots:**
The spinner frames use Unicode Braille patterns (U+2800-U+28FF):
- `⠋` (U+280B): Dots 1,2,3,8
- `⠙` (U+2819): Dots 1,4,5,8  
- `⠹` (U+2839): Dots 1,4,5,6,8
- And so on...

These create a smooth rotating dot pattern when displayed in sequence.

### Error Handling Pattern

The example includes error display capability:

```rust
fn view(&self) -> String {
    let s = if let Some(err) = &self.err {
        format!("\nWe had some trouble: {}\n\n", err)
    } else if self.quitting {
        "".to_string()
    } else {
        format!("   {}  Loading forever...press q to quit", self.current_frame())
    };
    s
}
```

### Performance Considerations

**Efficient Animation:**
- Static frame array (no allocations during animation)
- Minimal state updates (just frame index)
- Single-shot timers prevent accumulation
- 100ms interval balances smoothness with performance

**Memory Usage:**
- Fixed frame count (10 frames)
- No frame history or interpolation
- Minimal model state (just current index)

### Visual Design Principles

**Character Choice:** Braille dots provide:
- Consistent character width (monospace-friendly)
- Subtle animation that doesn't distract
- Professional appearance
- Wide Unicode support

**Color Choice:** Pink (#205) provides:
- Good contrast on most terminal backgrounds
- Matches Charm's design system
- Not too aggressive or distracting

**Timing:** 100ms intervals provide:
- Smooth perceived motion
- Low CPU overhead
- Responsive feel without being jittery

### Common Usage Patterns

**In Real Applications:**
```rust
// Show spinner during async operations
if self.loading {
    format!("{}  Downloading file...", self.spinner.current_frame())
} else {
    "Download complete!".to_string()
}
```

**With Progress Information:**
```rust
format!("{}  Processing item {} of {}...", 
    self.spinner.current_frame(),
    current_item,
    total_items
)
```

## Related Examples

- **[spinners](../spinners/)** - Multiple spinner styles and patterns
- **[progress-animated](../progress-animated/)** - Progress indicators with animation
- **[realtime](../realtime/)** - Another real-time updating interface

## Files

- `main.rs` — Complete spinner implementation with styling
- `Cargo.toml` — Dependencies including lipgloss-extras
- `spinner.gif` — Demo showing animation frames
- `README.md` — This documentation

## Customization Ideas

- Different frame patterns (line spinner, clock, etc.)
- Variable speeds for different operations
- Color changes based on operation status
- Multiple simultaneous spinners
- Integration with progress percentages