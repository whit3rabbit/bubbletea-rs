# Stopwatch

A direct port of the Go Bubble Tea stopwatch example demonstrating how to use `bubbletea-widgets` components. This example closely mirrors [`bubbletea/examples/stopwatch/main.go`](https://github.com/charmbracelet/bubbletea/blob/master/examples/stopwatch/main.go) behavior and output.

## Features

- **Count-up timer** that starts automatically with millisecond precision
- **Toggle start/stop** with `s` (key binding changes based on state)
- **Reset** to 00:00.000 with `r`
- **Quit** with `q` or `Ctrl+C`
- **Help system** that shows only enabled key bindings

## Widget Components Used

- **`bubbletea-widgets::stopwatch`**: High-precision count-up timer with automatic tick management
- **`bubbletea-widgets::key`**: Organized key binding system with help text and enable/disable functionality
- **`bubbletea-widgets::help`**: Automatic help text generation that displays available key bindings

## Run

From the repository root:

```bash
cargo run --example stopwatch
```

Or using the example package:

```bash
cargo run -p stopwatch
```

## What this demonstrates

- **Stopwatch Widget Usage**: Creating a stopwatch with `new_with_interval(Duration::from_millis(1))` for smooth millisecond updates
- **Key Binding Management**: Using structured key bindings with help text and conditional enabling/disabling
- **Message Handling**: Properly routing stopwatch messages between components
- **State-Based UI**: Dynamic key binding enabling based on stopwatch state (start enabled when stopped, stop enabled when running)
- **Help System Integration**: Automatic help text generation that filters to show only enabled bindings

## Architecture (Go → Rust Translation)

### Go Structs → Rust Structs
```go
type model struct {
    stopwatch stopwatch.Model
    keymap    keymap  
    help      help.Model
    quitting  bool
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
    stopwatch: StopwatchModel,
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
case tea.KeyMsg:
    switch {
    case key.Matches(msg, m.keymap.reset):
        return m, m.stopwatch.Reset()
    }
}
var cmd tea.Cmd
m.stopwatch, cmd = m.stopwatch.Update(msg)
return m, cmd
```

```rust
if let Some(key) = msg.downcast_ref::<KeyMsg>() {
    if matches_binding(key, &self.keymap.reset) {
        return Some(self.stopwatch.reset());
    }
}
self.stopwatch.update(msg)
```

### Go Key State Management → Rust Key State Management
```go
case key.Matches(msg, m.keymap.start, m.keymap.stop):
    m.keymap.stop.SetEnabled(!m.stopwatch.Running())
    m.keymap.start.SetEnabled(m.stopwatch.Running())
    return m, m.stopwatch.Toggle()
```

```rust
if matches_binding(key, &self.keymap.start) || matches_binding(key, &self.keymap.stop) {
    self.keymap.stop.set_enabled(!self.stopwatch.running());
    self.keymap.start.set_enabled(self.stopwatch.running());
    return Some(self.stopwatch.toggle());
}
```

## Key Differences from Go Version

### Help System Widget
The Rust version uses the `bubbletea-widgets::help` widget matching Go's `bubbles/help` package, providing automatic help text generation from key bindings with proper filtering for enabled/disabled states.

### Rust-Specific Patterns
- **Pattern Matching**: Uses `if let Some()` patterns instead of Go's type switches
- **Ownership**: Stopwatch messages are properly forwarded while respecting Rust's ownership rules
- **Error Handling**: Uses `Result<(), Box<dyn std::error::Error>>` for proper error propagation

### Maintained Compatibility
- **Same timing behavior**: Millisecond precision updates
- **Same key bindings**: Identical keyboard controls
- **Same output format**: Matching display and help text (00:00.000 format)
- **Same state logic**: Start/stop key enabling follows Go's behavior exactly

## Time Format

The stopwatch displays time in MM:SS.mmm format:
- **00:05.234** - 5 seconds and 234 milliseconds
- **01:23.456** - 1 minute, 23 seconds, and 456 milliseconds
- **12:34:56.789** - Hours are shown when needed

## Using Stopwatch Widgets in Your Code

### Basic Usage
```rust
use bubbletea_widgets::stopwatch::{new_with_interval, Model as StopwatchModel};
use std::time::Duration;

// Create a stopwatch with millisecond precision
let stopwatch = new_with_interval(Duration::from_millis(1));

// Initialize and start the stopwatch
let init_cmd = stopwatch.init();
```

### Message Handling
```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    // Handle key messages first
    if let Some(key) = msg.downcast_ref::<KeyMsg>() {
        if toggle_binding.matches(key) {
            return Some(self.stopwatch.toggle());
        }
    }
    
    // Forward other messages to stopwatch
    self.stopwatch.update(msg)
}
```

### Key Integration
```rust
use bubbletea_widgets::key::{new_binding, with_keys_str, with_help};

let reset_binding = new_binding(vec![
    with_keys_str(&["r"]),
    with_help("r", "reset stopwatch"),
]);

// In update loop:
if reset_binding.matches(&key_msg) {
    return Some(self.stopwatch.reset());
}
```

## Files

- `main.rs` — Direct Go port implementation
- `Cargo.toml` — Example dependencies
- `README.md` — This documentation