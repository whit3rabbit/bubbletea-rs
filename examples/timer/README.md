# Timer

<img width="1200" src="./timer.gif" />

A direct port of the Go Bubble Tea timer example demonstrating how to use `bubbletea-widgets` components. This example closely mirrors [`bubbletea/examples/timer/main.go`](https://github.com/charmbracelet/bubbletea/blob/master/examples/timer/main.go) behavior and output.

## Features

- **5s countdown** that starts automatically with high-precision timing (v0.0.8+ improved accuracy)
- **Toggle start/stop** with `s` (key binding changes based on timer state)
- **Reset** to full timeout with `r`
- **Quit** with `q` or `Ctrl+C`
- **Automatic quit** when timer reaches zero
- **Help system** that shows only enabled key bindings

## Widget Components Used

- **`bubbletea-widgets::timer`**: High-precision countdown timer with automatic tick management (v0.0.8+ improved timing accuracy)
- **`bubbletea-widgets::key`**: Organized key binding system with help text and enable/disable functionality
- **`bubbletea-widgets::help`**: Automatic help text generation that displays available key bindings

## Run

From the repository root:

```bash
cargo run --example timer
```

Or using the example package:

```bash
cargo run -p timer
```

## What this demonstrates

- **Timer Widget Usage**: Creating a timer with `new_with_interval(timeout, Duration::from_millis(1))` for smooth millisecond updates
- **Key Binding Management**: Using structured key bindings with help text and conditional enabling/disabling
- **Message Handling**: Properly routing timer messages (`TickMsg`, `StartStopMsg`, `TimeoutMsg`) between components
- **State-Based UI**: Dynamic key binding enabling based on timer state (start enabled when stopped, stop enabled when running)
- **Help System Integration**: Automatic help text generation that filters to show only enabled bindings

## Architecture (Go → Rust Translation)

### Go Structs → Rust Structs
```go
type model struct {
    timer    timer.Model
    keymap   keymap  
    help     help.Model
    quitting bool
}

type keymap struct {
    start key.Binding
    stop  key.Binding
    reset key.Binding
    quit  key.Binding
}
```

```rust
pub struct Model {
    timer: TimerModel,
    keymap: Keymap,
    help: HelpModel,
    quitting: bool,
}

pub struct Keymap {
    pub start: Binding,
    pub stop: Binding,
    pub reset: Binding, 
    pub quit: Binding,
}
```

### Go Key Bindings → Rust Key Bindings
```go
start: key.NewBinding(
    key.WithKeys("s"),
    key.WithHelp("s", "start"),
),
```

```rust
start: new_binding(vec![
    with_keys_str(&["s"]),
    with_help("s", "start"),
]),
```

### Go Message Handling → Rust Message Handling
```go
switch msg := msg.(type) {
case timer.TickMsg:
    var cmd tea.Cmd
    m.timer, cmd = m.timer.Update(msg)
    return m, cmd
}
```

```rust
if let Some(_tick_msg) = msg.downcast_ref::<TickMsg>() {
    return self.timer.update(msg);
}
```

### Go Key State Management → Rust Key State Management
```go
case timer.StartStopMsg:
    var cmd tea.Cmd
    m.timer, cmd = m.timer.Update(msg)
    m.keymap.stop.SetEnabled(m.timer.Running())
    m.keymap.start.SetEnabled(!m.timer.Running())
    return m, cmd
```

```rust
if let Some(_start_stop_msg) = msg.downcast_ref::<StartStopMsg>() {
    let cmd = self.timer.update(msg);
    self.keymap.stop.set_enabled(self.timer.running());
    self.keymap.start.set_enabled(!self.timer.running());
    return cmd;
}
```

## Key Differences from Go Version

### Help System Widget
The Rust version now uses the `bubbletea-widgets::help` widget matching Go's `bubbles/help` package, providing automatic help text generation from key bindings with proper filtering for enabled/disabled states.

### Rust-Specific Patterns
- **Pattern Matching**: Uses `if let Some()` patterns instead of Go's type switches
- **Ownership**: Timer messages are properly forwarded while respecting Rust's ownership rules
- **Error Handling**: Uses `Result<(), Box<dyn std::error::Error>>` for proper error propagation

### Maintained Compatibility
- **Same timing behavior**: Millisecond precision updates
- **Same key bindings**: Identical keyboard controls
- **Same output format**: Matching display and help text
- **Same state logic**: Start/stop key enabling follows Go's behavior exactly

## Advanced Timer Example

For a more sophisticated timer application with multiple presets, progress bars, and advanced styling, see the [`timer-advanced`](../timer-advanced/) example which demonstrates:

- Multiple timer types (Quick, Pomodoro, Break)
- Visual progress bars and status indicators
- Rich styling with colors and formatting
- Responsive layouts and advanced UI patterns

## Using Timer Widgets in Your Code

### Basic Usage
```rust
use bubbletea_widgets::timer::{new_with_interval, Model as TimerModel};
use std::time::Duration;

// Create a 30-second timer with 100ms precision
let timer = new_with_interval(Duration::from_secs(30), Duration::from_millis(100));

// Initialize and start the timer
let init_cmd = timer.init();
```

### Message Handling
```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    // Forward timer messages
    if msg.is::<TickMsg>() || msg.is::<StartStopMsg>() || msg.is::<TimeoutMsg>() {
        return self.timer.update(msg);
    }
    
    // Handle other messages...
    None
}
```

### Key Integration
```rust
use bubbletea_widgets::key::{new_binding, with_keys_str, with_help};

let toggle_binding = new_binding(vec![
    with_keys_str(&["space"]),
    with_help("space", "start/stop timer"),
]);

// In update loop:
if toggle_binding.matches(&key_msg) {
    return Some(self.timer.toggle());
}
```

## Files

- `main.rs` — Direct Go port implementation
- `Cargo.toml` — Example dependencies
- `README.md` — This documentation