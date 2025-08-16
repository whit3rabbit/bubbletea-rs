# Progress Animated

<img width="1200" src="./progress-animated.gif" />

A smooth animated progress bar demonstration showing gradient colors, smooth animations, and responsive design patterns in bubbletea-rs applications.

## Features

- **Smooth Animation**: 60fps progress bar animation with exponential easing
- **Gradient Colors**: Beautiful color gradients matching Charm's default palette
- **25% Increments**: Progress increases in 25% steps every second
- **Responsive Design**: Progress bar width adjusts to window size changes
- **Automatic Completion**: Program exits automatically when reaching 100%
- **Custom Animation System**: Demonstrates building animations from scratch

## Running the Example

From the repository root:

```bash
cargo run --example progress-animated
```

For debug output showing animation frames:
```bash
BT_DEBUG=1 cargo run --example progress-animated
```

**Controls:**
- `q` / `Ctrl+C` - Quit early
- Wait for automatic completion at 100%

## What this demonstrates

### Key Concepts for Beginners

**Animation in TUI**: This example shows how to create smooth animations in terminal applications using:
1. High-frequency timer messages (16ms = ~60fps)
2. Exponential easing for smooth motion
3. State interpolation between current and target values
4. Efficient rendering to avoid flicker

**Responsive UI**: Demonstrates handling window resize events to maintain proper proportions.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{batch, quit, tick, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
```

- `tick()`: High-frequency timers for animation frames
- `batch()`: Combining multiple commands (progress + animation)
- `WindowSizeMsg`: Handling terminal resize events

**Gradient Rendering:**
```rust
use bubbletea_rs::gradient::gradient_filled_segment;
```

- `gradient_filled_segment()`: Built-in gradient rendering helper
- Matches Charm's default gradient colors (#FF7CCB → #FDFF8C)

### Architecture Walkthrough

#### Custom Progress Bar Component
```rust
pub struct AnimatedProgressBar {
    pub width: usize,         // Visual width in characters
    pub current_percent: f64, // Current animated position (0.0-1.0)
    pub target_percent: f64,  // Target position for animation
    pub animation_speed: f64, // Animation easing factor (0.1 = 10% per frame)
}
```

#### Animation System

The core animation uses exponential easing with minimum step guarantees:

```rust
pub fn update_animation(&mut self) -> Option<Cmd> {
    let diff = self.target_percent - self.current_percent;
    
    if diff.abs() > TOLERANCE {
        // Calculate step: exponential decay or minimum step
        let exponential_step = diff * self.animation_speed;  // Smooth easing
        let step = if exponential_step.abs() >= MIN_STEP {
            exponential_step
        } else {
            // Ensure progress when exponential becomes too small
            if diff > 0.0 { MIN_STEP } else { -MIN_STEP }
        };
        
        self.current_percent += step;
        
        // Continue animation at 60fps
        Some(tick(Duration::from_millis(16), |_| {
            Box::new(ProgressFrameMsg) as Msg
        }))
    } else {
        None  // Animation complete
    }
}
```

#### Progress Logic

The example increments progress in 25% steps:

```rust
// Every second, increase progress by 25%
if msg.downcast_ref::<ProgressTickMsg>().is_some() {
    if self.progress.target_percent < 1.0 {
        let progress_cmd = self.progress.incr_percent(0.25);
        
        // Schedule next progress update
        let next_tick = tick(Duration::from_secs(1), |_| {
            Box::new(ProgressTickMsg) as Msg
        });
        
        // Combine progress animation with next timer
        return Some(batch(vec![progress_cmd.unwrap_or_else(|| next_tick), next_tick]));
    } else {
        // 100% reached - exit after animation completes
        return Some(quit());
    }
}
```

### Rust-Specific Patterns

**Debug Logging:**
```rust
static DEBUG_ENABLED: OnceLock<bool> = OnceLock::new();
fn debug_enabled() -> bool {
    *DEBUG_ENABLED.get_or_init(|| std::env::var("BT_DEBUG").is_ok())
}

macro_rules! dlog {
    ($($arg:tt)*) => {
        if debug_enabled() { eprintln!($($arg)*); }
    }
}
```

Thread-safe lazy initialization for debug mode detection.

**Gradient Rendering:**
```rust
let filled = gradient_filled_segment(filled_width, '█');
let empty = "░".repeat(empty_width);
let bar = format!("{}{}", filled, empty);
```

Using the shared gradient helper for consistent colors.

**Responsive Width:**
```rust
// Handle window resize
if let Some(window_msg) = msg.downcast_ref::<WindowSizeMsg>() {
    // Adjust progress bar width based on terminal width
    let new_width = ((window_msg.width as f64 * 0.8) as usize).max(20).min(80);
    self.progress.width = new_width;
}
```

### Animation Mathematics

**Exponential Easing Formula:**
```
new_position = current + (target - current) * speed_factor
```

With `speed_factor = 0.1`, the animation covers:
- 10% of remaining distance each frame
- Starts fast, slows near target (natural motion)
- Always reaches target (with minimum step guarantee)

**Frame Rate Calculation:**
```
16ms per frame = 1000ms / 16 ≈ 62.5 fps
```

**Tolerance Handling:**
```rust
const TOLERANCE: f64 = 0.0001;  // Snap to target when very close
const MIN_STEP: f64 = 0.005;    // Minimum 0.5% progress per frame
```

### Performance Considerations

**Efficient Animation:**
- Only runs animation timers when needed (target ≠ current)
- Stops animation when reaching target (within tolerance)
- Uses single-shot `tick()` to avoid timer accumulation

**Batch Command Optimization:**
```rust
// GOOD: Batch related commands together
Some(batch(vec![animation_cmd, next_progress_cmd]))

// AVOID: Multiple separate command returns
```

### Window Resize Handling

The progress bar adapts to terminal size changes:

```rust
fn handle_window_resize(&mut self, window_msg: &WindowSizeMsg) {
    // Calculate new width: 80% of terminal width, clamped to 20-80 chars
    let new_width = ((window_msg.width as f64 * 0.8) as usize).max(20).min(80);
    self.progress.width = new_width;
}
```

## Related Examples

- **[progress-static](../progress-static/)** - Simpler progress bar without animation
- **[spinner](../spinner/)** - Another animation example with different patterns
- **[timer](../timer/)** - Timer-based applications using widgets

## Files

- `main.rs` — Complete animated progress implementation
- `Cargo.toml` — Dependencies and build configuration
- `progress-animated.gif` — Demo showing smooth animation
- `README.md` — This documentation

## Animation Tips

- Use 16ms (60fps) for smooth motion
- Implement exponential easing for natural feel
- Always provide minimum step to guarantee completion
- Batch animation commands with business logic
- Use tolerance values to avoid infinite loops
- Test with various terminal sizes for responsiveness