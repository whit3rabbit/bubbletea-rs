# Progress Static

<img width="1200" src="./progress-static.gif" />

A simple static progress bar demonstration showing discrete progress updates, gradient colors, and responsive width handling for progress indicators in terminal applications.

## Features

- **Static Progress Updates**: Progress increases by 25% every second
- **Gradient Colors**: Beautiful gradient fill using Charm's default colors  
- **Integer Percentage Display**: Shows progress as whole numbers (25%, 50%, 75%, 100%)
- **Responsive Width**: Progress bar adjusts to terminal size changes
- **Keyboard Quit**: Exit early with any key press
- **Automatic Completion**: Program exits when reaching 100%

## Running the Example

From the repository root:

```bash
cargo run --example progress-static
```

**Controls:**
- Any key - Quit early
- Wait for automatic completion at 100%

## What this demonstrates

### Key Concepts for Beginners

**Progress Indicators**: This example shows the simplest form of progress visualization:
1. Discrete updates at regular intervals (no animation)
2. Visual representation using filled/empty characters
3. Percentage text display alongside visual bar
4. Responsive design for different terminal sizes

**Compared to progress-animated**: This version updates instantly rather than smoothly animating between values.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
```

- `tick()`: Single-shot timers for progress updates
- `WindowSizeMsg`: Handling terminal resize events
- Standard MVU pattern with custom progress component

**Gradient Rendering:**
```rust
use bubbletea_rs::gradient::gradient_filled_segment;
```

- Shared gradient helper for consistent colors across examples
- Matches Charm's default gradient (#FF7CCB → #FDFF8C)

**Styling:**
```rust
use lipgloss_extras::lipgloss::{Color, Style};
```

- `Style::new().foreground()`: Color styling for help text
- Matches Go version's muted color scheme

### Architecture Walkthrough

#### Progress Bar Component
```rust
pub struct ProgressBar {
    pub width: usize,      // Visual width in terminal columns
    pub filled_char: char, // Character for completed portion ('█')
    pub empty_char: char,  // Character for remaining portion ('░')
}
```

#### Simple Progress Logic
```rust
// Every second, increase by 25%
if msg.downcast_ref::<ProgressTickMsg>().is_some() {
    self.percent += 0.25;
    
    if self.percent >= 1.0 {
        return Some(quit());  // Exit when 100% reached
    }
    
    // Schedule next update in 1 second
    return Some(tick(Duration::from_secs(1), |_| {
        Box::new(ProgressTickMsg) as Msg
    }));
}
```

#### Rendering Implementation

The `view_as()` method creates the visual progress bar:

```rust
pub fn view_as(&self, percent: f64) -> String {
    let percent = percent.clamp(0.0, 1.0);
    let filled_width = (self.width as f64 * percent).round() as usize;
    let empty_width = self.width.saturating_sub(filled_width);
    
    // Use shared gradient helper
    let filled = gradient_filled_segment(filled_width, self.filled_char);
    let empty = self.empty_char.to_string().repeat(empty_width);
    
    // Format with integer percentage
    let label = format!("{:>3.0}%", percent * 100.0);
    format!("{}{} {}", filled, empty, label)
}
```

### Rust-Specific Patterns

**Percentage Formatting:**
```rust
let label = format!("{:>3.0}%", percent * 100.0);
//                    ^ right-align, 3 chars, no decimal places
```

Results in: "  0%", " 25%", " 50%", " 75%", "100%"

**Width Calculation:**
```rust
let filled_width = (self.width as f64 * percent).round() as usize;
let empty_width = self.width.saturating_sub(filled_width);
//                             ^ prevents underflow if filled_width > width
```

**Clamping Values:**
```rust
let percent = percent.clamp(0.0, 1.0);  // Ensure 0% ≤ percent ≤ 100%
```

**Window Resize Handling:**
```rust
if let Some(window_msg) = msg.downcast_ref::<WindowSizeMsg>() {
    // Calculate responsive width: 80% of terminal, between 10-80 chars
    let new_width = (window_msg.width as f64 * 0.8) as usize;
    self.progress_bar.width = new_width.max(10).min(80);
}
```

### Visual Design

**Color Gradient:**
Using the shared gradient helper ensures consistency:
- Start: `#FF7CCB` (Pink)
- End: `#FDFF8C` (Light Green)  
- Smooth interpolation across filled width

**Character Choice:**
```rust
filled_char: '█',   // Full block character - solid appearance
empty_char: '░',    // Light shade - subtle empty space
```

**Layout:**
```
████████████████████████████████████████ 100%
^-------- gradient filled portion ------^ ^-%^
                                           label
```

### Comparison with Go Version

**Maintained Compatibility:**
- Same 25% increment behavior
- Same automatic quit at 100%
- Same keyboard quit on any key
- Same percentage display format
- Same gradient colors (via shared helper)

**Rust Improvements:**
- Type-safe message handling
- Saturating arithmetic prevents underflow
- Responsive width with proper bounds checking
- Consistent styling via lipgloss-extras

### Performance Characteristics

**Efficient Updates:**
- No animation overhead (unlike progress-animated)
- Single render per second
- Minimal CPU usage between updates
- Instant visual feedback on progress changes

**Memory Usage:**
- Fixed-size progress bar component
- No animation state tracking
- Minimal string allocations

### Program Flow

1. **Initialization**: Start at 0%, schedule first tick
2. **Timer Loop**: Every second, increment by 25%
3. **Rendering**: Show gradient bar with percentage
4. **Completion**: Exit automatically at 100%
5. **Early Exit**: Any keypress quits immediately

### Responsive Design

The progress bar adapts to terminal size:

```rust
// Target 80% of terminal width, with reasonable bounds
let new_width = (window_width as f64 * 0.8) as usize;
let bounded_width = new_width.max(10).min(80);
```

This ensures:
- Small terminals: Minimum 10 characters wide
- Large terminals: Maximum 80 characters wide
- Medium terminals: 80% of available width

## Related Examples

- **[progress-animated](../progress-animated/)** - Smooth animated version
- **[progress-download](../progress-download/)** - Real download progress simulation
- **[simple](../simple/)** - Basic timer without progress visualization

## Files

- `main.rs` — Complete static progress implementation
- `Cargo.toml` — Dependencies including lipgloss-extras
- `progress-static.gif` — Demo showing step updates
- `README.md` — This documentation

## Use Cases

- File transfer progress
- Batch operation status
- Installation/setup progress
- Data processing indicators
- Any discrete progress updates

## Implementation Tips

- Use `tick()` for regular updates (not `every()` which can accumulate)
- Always clamp percentages to [0.0, 1.0] range
- Use saturating arithmetic to prevent underflow
- Consider responsive width for different terminal sizes
- Match user expectations with clear percentage labels