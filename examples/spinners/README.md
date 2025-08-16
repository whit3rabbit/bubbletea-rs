# Spinners

<img width="1200" src="./spinner.gif" />

A comprehensive demonstration of multiple spinner animation styles, showing different loading indicators, animation patterns, and interactive spinner selection for terminal applications.

## Features

- **Multiple Spinner Types**: 9 different spinner animations (line, dots, blocks, etc.)
- **Interactive Selection**: Number keys (1-9) switch between spinner styles
- **Consistent Styling**: Pink color scheme matching Charm's design system
- **Variable Animation Speeds**: Different spinners use appropriate frame rates
- **Live Preview**: See all spinner types in action with easy switching
- **Help Instructions**: Clear guidance for interaction

## Running the Example

From the repository root:

```bash
cargo run --example spinners
```

**Controls:**
- `1-9` - Switch between different spinner types
- `q` / `Ctrl+C` - Quit
- Watch different animation patterns and speeds

## What this demonstrates

### Key Concepts for Beginners

**Animation Variety**: This example shows how to:
1. Implement multiple animation styles in one application
2. Design different spinner patterns for various contexts
3. Handle user interaction to switch between animations
4. Create engaging loading indicators beyond basic dots

**State Management**: Demonstrates managing animation state, user selection, and smooth transitions between different visual patterns.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
```

- `tick()` for animation timing
- User input handling for interactive selection
- State management across different spinner types

**Styling System:**
```rust
use lipgloss_extras::lipgloss::{Color, Style};
```

- Consistent pink color scheme (#205)
- Professional spinner appearance

### Architecture Walkthrough

#### Spinner Type System

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinnerType {
    Line,     // | / - \
    Dot,      // Braille dot patterns  
    MiniDot,  // Smaller dot patterns
    Jump,     // Bouncing animation
    Pulse,    // Block fade effect
    Points,   // Multi-dot patterns
    Globe,    // Earth rotation
    Moon,     // Moon phases
    Monkey,   // Fun monkey animation
}
```

#### Animation Frame Definitions

```rust
fn frames(self) -> &'static [&'static str] {
    match self {
        SpinnerType::Line => &["|", "/", "-", "\\"],
        SpinnerType::Dot => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
        SpinnerType::MiniDot => &["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"],
        SpinnerType::Jump => &["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"],
        SpinnerType::Pulse => &["█", "▉", "▊", "▋", "▌", "▍", "▎", "▏", "▎", "▍", "▌", "▋", "▊", "▉"],
        SpinnerType::Points => &["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"],
        SpinnerType::Globe => &["🌍", "🌎", "🌏"],
        SpinnerType::Moon => &["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
        SpinnerType::Monkey => &["🙈", "🙉", "🙊"],
    }
}
```

Each spinner has its own character set optimized for that animation style.

#### Dynamic Animation Speed

```rust
fn interval(self) -> Duration {
    match self {
        SpinnerType::Line => Duration::from_millis(100),      // Fast classic
        SpinnerType::Dot | SpinnerType::MiniDot => Duration::from_millis(80),  // Smooth dots
        SpinnerType::Jump => Duration::from_millis(120),      // Bouncy timing
        SpinnerType::Pulse => Duration::from_millis(150),     // Slow fade
        SpinnerType::Points => Duration::from_millis(200),    // Deliberate progression
        SpinnerType::Globe => Duration::from_millis(300),     // Earth rotation
        SpinnerType::Moon => Duration::from_millis(400),      // Moon phases
        SpinnerType::Monkey => Duration::from_millis(500),    // Fun timing
    }
}
```

Different animations need different frame rates for optimal visual effect.

#### Interactive Selection

```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
        match key_msg.key {
            KeyCode::Char('1') => self.select_spinner(0),
            KeyCode::Char('2') => self.select_spinner(1),
            KeyCode::Char('3') => self.select_spinner(2),
            // ... up to '9'
            KeyCode::Char('q') => return Some(quit()),
            KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                return Some(quit());
            }
            _ => {}
        }
    }
    
    // Handle animation frames
    if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
        self.current_frame = (self.current_frame + 1) % self.current_spinner().frames().len();
        
        // Schedule next frame with appropriate timing
        return Some(tick(self.current_spinner().interval(), |_| {
            Box::new(SpinnerTickMsg) as Msg
        }));
    }
    
    None
}
```

### Rust-Specific Patterns

**Enum-Based Spinner Selection:**
```rust
impl SpinnerType {
    fn all() -> &'static [SpinnerType] {
        &[SpinnerType::Line, SpinnerType::Dot, /* ... */]
    }
    
    fn name(self) -> &'static str {
        match self {
            SpinnerType::Line => "Line",
            SpinnerType::Dot => "Dot", 
            // ... 
        }
    }
}
```

Type-safe spinner selection with compile-time guarantees.

**Frame Wrapping:**
```rust
self.current_frame = (self.current_frame + 1) % self.current_spinner().frames().len();
```

Safe array bounds with modulo arithmetic.

**Dynamic Interval Selection:**
```rust
tick(self.current_spinner().interval(), |_| {
    Box::new(SpinnerTickMsg) as Msg
})
```

Each spinner type controls its own animation speed.

**Static String Arrays:**
```rust
fn frames(self) -> &'static [&'static str] {
    // Compile-time constant arrays, efficient memory usage
}
```

### Spinner Design Principles

**Character Choice:**

**Line Spinner**: Classic ASCII characters `| / - \`
- Universal compatibility
- Clear rotation pattern
- Fast animation works well

**Dot Spinners**: Braille Unicode patterns
- Professional appearance
- Subtle animation
- Multiple dot variants available

**Block Spinners**: Block characters for fade effects
- `█▉▊▋▌▍▎▏` creates smooth fade
- Good for progress-like feedback
- Works well with slower animation

**Emoji Spinners**: Unicode emoji for fun effects  
- Globe: `🌍🌎🌏` shows world rotation
- Moon: `🌑🌒🌓🌔🌕🌖🌗🌘` shows phases
- Monkey: `🙈🙉🙊` adds personality

### Animation Timing

**Fast Animations (80-100ms):**
- Line and dot patterns
- High-frequency visual feedback
- Good for intensive operations

**Medium Animations (120-200ms):**
- Jump and points patterns  
- Balanced visual interest
- Good for moderate operations

**Slow Animations (300-500ms):**
- Globe, moon, emoji patterns
- Deliberate, calming effect
- Good for background operations

### Visual Display

```rust
fn view(&self) -> String {
    let style = Style::new().foreground(Color::from("205"));  // Pink
    
    let spinner_char = self.current_spinner().frames()[self.current_frame];
    let spinner_name = self.current_spinner().name();
    
    format!(
        "   {}  {}\n\n\
         Choose a spinner:\n\
         1. Line    2. Dot      3. MiniDot\n\
         4. Jump    5. Pulse    6. Points\n\
         7. Globe   8. Moon     9. Monkey\n\n\
         Press q to quit",
        style.render(spinner_char),
        spinner_name
    )
}
```

### Real-world Applications

**Context-Appropriate Spinners:**

**System Operations:**
```rust
// Fast, technical operations
SpinnerType::Dot     // File processing
SpinnerType::Line    // Network requests
SpinnerType::Jump    // Compilation
```

**User-Facing Operations:**
```rust
// Slower, user-friendly operations  
SpinnerType::Globe   // Data synchronization
SpinnerType::Points  // Search operations
SpinnerType::Pulse   // Loading content
```

**Fun Applications:**
```rust
// Entertainment or casual apps
SpinnerType::Moon    // Night mode operations
SpinnerType::Monkey  // Game loading
```

### Performance Characteristics

**Memory Usage:**
- Static frame arrays (minimal memory)
- Single animation state tracking
- No frame buffering or history

**CPU Usage:**
- Efficient timer-based animation
- Minimal computation per frame
- Frame rate optimized per spinner type

**Terminal Compatibility:**
- ASCII spinners: Universal support
- Unicode dots: Most modern terminals
- Emoji: Requires Unicode emoji support

### Spinner Selection Strategy

```rust
pub fn select_best_spinner_for_operation(operation: &str) -> SpinnerType {
    match operation {
        "compile" | "build" => SpinnerType::Jump,
        "download" | "upload" => SpinnerType::Dot,
        "search" | "filter" => SpinnerType::Points,
        "sync" | "backup" => SpinnerType::Globe,
        "process" | "transform" => SpinnerType::Pulse,
        _ => SpinnerType::Line,  // Safe default
    }
}
```

### Extension Ideas

**Custom Spinners:**
```rust
SpinnerType::Custom(CustomSpinner {
    frames: vec!["🚀", "✨", "⭐", "💫"],
    interval: Duration::from_millis(250),
    name: "Rocket",
})
```

**Color Variations:**
```rust
pub struct ColoredSpinner {
    spinner_type: SpinnerType,
    colors: Vec<Color>,
    color_index: usize,
}
```

**Progress Integration:**
```rust
pub struct ProgressSpinner {
    spinner: SpinnerType,
    progress: f64,  // 0.0 - 1.0
}
```

## Related Examples

- **[spinner](../spinner/)** - Single spinner implementation
- **[progress-animated](../progress-animated/)** - Progress bar animations
- **[realtime](../realtime/)** - Another real-time animation example

## Files

- `main.rs` — Complete multi-spinner implementation with selection
- `Cargo.toml` — Dependencies including lipgloss-extras
- `spinner.gif` — Demo showing all spinner types
- `README.md` — This documentation

## Implementation Tips

- Choose spinner styles appropriate for your application context
- Use faster animations for intensive operations, slower for background tasks
- Test spinner compatibility across different terminals
- Consider providing fallback ASCII spinners for compatibility
- Match spinner timing to perceived operation duration