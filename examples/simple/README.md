# Simple

<img width="1200" src="./simple.gif" />

The foundational example demonstrating core bubbletea-rs concepts with a 5-second countdown timer, keyboard input handling, and the essential Model-View-Update (MVU) architecture pattern.

## Features

- **5-Second Countdown**: Automatic timer that counts down from 5 to 0
- **Multiple Exit Options**: Quit with `q`, `Ctrl+C`, or wait for countdown completion
- **Process Suspension**: Suspend with `Ctrl+Z` and resume with shell commands
- **Automatic Termination**: Program exits when timer reaches zero
- **Clean Architecture**: Demonstrates the fundamental MVU pattern

## Running the Example

From the repository root:

```bash
cargo run --example simple
```

**Controls:**
- `q` - Quit immediately
- `Ctrl+C` - Quit immediately  
- `Ctrl+Z` - Suspend process (use `fg` to resume)
- Wait 5 seconds for automatic exit

## What this demonstrates

### Key Concepts for Beginners

**Model-View-Update (MVU) Architecture**: This example is the perfect introduction to bubbletea-rs because it demonstrates all three core components:

1. **Model**: Application state (`SimpleModel(i32)` - just a countdown number)
2. **View**: UI rendering (`view()` method returns display string)
3. **Update**: Event handling (`update()` method processes messages and updates state)

**Essential Patterns**: Every bubbletea-rs application uses these patterns shown here.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, suspend, Cmd, KeyMsg, Model, Msg, Program};
```

- `Model` trait: The foundation of all bubbletea-rs applications
- `KeyMsg`: Keyboard input events
- `Cmd`: Asynchronous commands (like timers)
- `quit()`: Graceful program termination
- `suspend()`: Process suspension support

**Crossterm Integration:**
```rust
use crossterm::event::{KeyCode, KeyModifiers};
```

- `KeyCode::Char('q')`: Character key detection
- `KeyModifiers::CONTROL`: Modifier key combinations

### Architecture Walkthrough

#### Model Definition
```rust
#[derive(Debug)]
struct SimpleModel(i32);  // Tuple struct holding countdown value
```

**Why a tuple struct?** 
This is the simplest possible model - just a number. Real applications typically use regular structs with multiple fields.

#### Model Implementation

**Initialization:**
```rust
fn init() -> (Self, Option<Cmd>) {
    (SimpleModel(5), Some(tick()))  // Start at 5, begin countdown
}
```

**The init pattern:** Every application starts here, optionally scheduling initial commands.

**Update Logic:**
```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    // Handle keyboard input
    if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
        match key_msg.key {
            KeyCode::Char('q') => return Some(quit()),
            KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                return Some(quit());
            }
            KeyCode::Char('z') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                return Some(suspend());
            }
            _ => {} // Ignore other keys
        }
    }

    // Handle timer ticks
    if let Some(_tick_msg) = msg.downcast_ref::<TickMsg>() {
        self.0 -= 1;  // Decrement counter
        if self.0 <= 0 {
            return Some(quit());  // Exit when reaching zero
        }
        return Some(tick());  // Schedule next tick
    }

    None  // No command to run
}
```

**View Rendering:**
```rust
fn view(&self) -> String {
    format!(
        "Hi. This program will exit in {} seconds.\n\n\
         To quit sooner press ctrl-c, or press ctrl-z to suspend...\n",
        self.0
    )
}
```

**The view pattern:** Always returns a string representation of current state.

### Rust-Specific Patterns

**Message Type Checking:**
```rust
if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
    // We now know msg is a KeyMsg and can access its fields
}
```

Unlike Go's type switches, Rust uses `downcast_ref()` for runtime type checking.

**Custom Message Types:**
```rust
#[derive(Debug)]
struct TickMsg;  // Unit struct for timer events
```

**Async Command Creation:**
```rust
fn tick() -> Cmd {
    Box::pin(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        Some(Box::new(TickMsg) as Msg)  // Return message after delay
    })
}
```

**The command pattern:** Commands are `Pin<Box<dyn Future<...>>>` - async functions that optionally return messages.

**Program Builder:**
```rust
let program = Program::<SimpleModel>::builder()
    .signal_handler(true)  // Enable Ctrl+C and Ctrl+Z handling
    .build()?;
```

**Type Parameters:** `Program<SimpleModel>` ensures type safety throughout.

### MVU Flow Diagram

```
┌─────────────┐    User Input     ┌─────────────┐    State Change    ┌─────────────┐
│    View     │ ◄───────────────── │   Update    │ ◄────────────────── │    Model    │
│             │                   │             │                   │             │
│ "Exit in 3" │                   │ handle 'q'  │                   │ count: 3    │
│ "seconds"   │                   │ handle tick │                   │             │
└─────────────┘                   └─────────────┘                   └─────────────┘
      │                                   │                                 ▲
      │            Display               │ Optional                       │
      ▼            String               ▼ Command                        │
┌─────────────┐                   ┌─────────────┐     Message          │
│  Terminal   │                   │   Runtime   │ ─────────────────────┘
│   Display   │                   │   System    │
└─────────────┘                   └─────────────┘
```

### Error Handling Pattern

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<SimpleModel>::builder()
        .signal_handler(true)
        .build()?;  // Builder can fail

    program.run().await?;  // Program execution can fail

    Ok(())
}
```

**Why `Box<dyn std::error::Error>`?**
- Allows any error type to be returned
- Common pattern for `main()` functions
- Simplifies error handling for beginners

### Common Beginner Patterns

**State Mutation:**
```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    //      ^^^^ mutable reference allows state changes
    self.0 -= 1;  // Modify the model state
}
```

**Pattern Matching:**
```rust
match key_msg.key {
    KeyCode::Char('q') => return Some(quit()),
    KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
        return Some(quit());
    }
    _ => {} // Always include catch-all for unhandled keys
}
```

**Option Handling:**
```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    // ...
    None  // Return None when no command needs to run
}
```

### Timer Implementation Details

**Why single-shot timers?**
```rust
return Some(tick());  // Schedule ONE more tick
```

This prevents timer accumulation - each tick schedules exactly one more.

**Timer Precision:**
```rust
tokio::time::sleep(Duration::from_secs(1)).await;
```

Uses tokio's high-precision timer system.

### Signal Handling

**Process Suspension:**
```rust
KeyCode::Char('z') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
    return Some(suspend());
}
```

**Signal handler requirement:**
```rust
.signal_handler(true)  // Must enable for Ctrl+C and Ctrl+Z
```

### Testing the Example

```bash
# Test normal countdown
cargo run --example simple
# Wait 5 seconds or press keys

# Test suspension
cargo run --example simple
# Press Ctrl+Z
fg  # Resume from shell

# Test early quit
cargo run --example simple
# Press 'q' or Ctrl+C
```

### Extending the Example

**Add more state:**
```rust
#[derive(Debug)]
struct ExtendedModel {
    count: i32,
    message: String,
    paused: bool,
}
```

**Handle more keys:**
```rust
KeyCode::Char(' ') => {
    self.paused = !self.paused;
    if !self.paused {
        return Some(tick());
    }
}
```

**Add colors:**
```rust
use lipgloss_extras::lipgloss::{Color, Style};

fn view(&self) -> String {
    let style = Style::new().foreground(Color::from("205"));
    style.render(&format!("Exit in {} seconds", self.0))
}
```

## Related Examples

**Next Steps:**
- **[timer](../timer/)** - Timer with widgets and more features
- **[spinner](../spinner/)** - Another timer-based animation
- **[help](../help/)** - Key binding organization

**Similar Patterns:**
- **[fullscreen](../fullscreen/)** - Another countdown example with alt-screen
- **[window-size](../window-size/)** - Simple state with terminal integration

## Files

- `main.rs` — Complete simple countdown implementation
- `Cargo.toml` — Minimal dependencies (bubbletea-rs, crossterm, tokio)
- `simple.gif` — Demo animation
- `README.md` — This documentation

## Why Start Here

This example is the perfect introduction because it demonstrates:

1. **All Core Concepts**: MVU architecture, messages, commands, state management
2. **Minimal Complexity**: Just 90 lines of well-commented code
3. **Real Functionality**: Actual timer, keyboard input, process control
4. **Foundation Patterns**: Everything you learn here applies to complex applications

**Every bubbletea-rs application** builds on these patterns shown in the simple example.