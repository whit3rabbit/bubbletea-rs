# Focus Blur

A demonstration of terminal focus/blur event handling, showing how applications can respond when users switch between terminal windows or applications.

## Features

- **Focus State Detection**: Knows when terminal window gains/loses focus
- **Toggle Focus Reporting**: Enable/disable focus event handling at runtime  
- **Visual Feedback**: Clear indication of current focus state
- **Event Handling**: Shows proper handling of `FocusMsg` and `BlurMsg` events

## Running the Example

From the repository root:

```bash
cargo run --example focus-blur
```

**Controls:**
- `t` - Toggle focus reporting on/off
- `q` / `Ctrl+C` - Quit

**Testing Focus Events:**
1. Run the example
2. Click on another application/window
3. Click back to the terminal
4. Observe the focus state changes

## What this demonstrates

### Key Concepts for Beginners

**Focus Events**: Modern terminals can report when they gain/lose focus, allowing applications to:
- Pause animations when not visible
- Show connection status indicators
- Implement "away" states
- Optimize resource usage

**Event-Driven Architecture**: This example shows how the MVU pattern handles events that aren't user-initiated.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, BlurMsg, FocusMsg, KeyMsg, Model, Msg, Program};
```

- `FocusMsg`: Sent when terminal gains focus
- `BlurMsg`: Sent when terminal loses focus  
- Standard MVU message handling pattern

**Key Binding System:**
```rust
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};
```

### Architecture Walkthrough

#### Model Structure
```rust
pub struct AppModel {
    focused: bool,      // Current focus state
    reporting: bool,    // Whether focus reporting is enabled
    keys: KeyBindings,  // Organized keyboard shortcuts
}
```

#### Focus Event Handling

The core focus/blur logic is straightforward:

```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    if msg.downcast_ref::<FocusMsg>().is_some() {
        self.focused = true;  // Terminal gained focus
    } else if msg.downcast_ref::<BlurMsg>().is_some() {
        self.focused = false; // Terminal lost focus
    }
    // ... other message handling
}
```

#### Program Configuration

Focus reporting must be explicitly enabled:

```rust
let program = Program::<AppModel>::builder()
    .report_focus(true)  // Enable focus/blur event reporting
    .build()?;
```

### Rust-Specific Patterns

**Message Type Checking:**
```rust
if msg.downcast_ref::<FocusMsg>().is_some() {
    // Handle focus gained
} else if msg.downcast_ref::<BlurMsg>().is_some() {
    // Handle focus lost
}
```

Unlike Go's type switches, Rust uses runtime type checking with `downcast_ref()`.

**State Toggle Pattern:**
```rust
if self.keys.toggle.matches(key_msg) {
    self.reporting = !self.reporting;  // Toggle boolean state
}
```

**Conditional Display:**
```rust
fn view(&self) -> String {
    let mut s = String::from("Hi. Focus report is currently ");
    s.push_str(if self.reporting { "enabled" } else { "disabled" });
    
    if self.reporting {
        s.push_str(if self.focused { 
            "This program is currently focused!" 
        } else { 
            "This program is currently blurred!" 
        });
    }
    s
}
```

### Practical Applications

**Resource Management:**
```rust
// Example: Pause expensive operations when not focused
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    if msg.downcast_ref::<BlurMsg>().is_some() {
        self.paused = true;
        return Some(stop_animations());
    } else if msg.downcast_ref::<FocusMsg>().is_some() {
        self.paused = false;
        return Some(resume_animations());
    }
    // ...
}
```

**Connection Status:**
```rust
// Example: Show "away" status when blurred
fn view(&self) -> String {
    let status = if self.focused {
        "Online"
    } else {
        "Away"
    };
    format!("Status: {}\n{}", status, self.main_content)
}
```

### Terminal Compatibility

Focus reporting requires terminal support:
- ✅ **Modern terminals**: iTerm2, Alacritty, Windows Terminal, GNOME Terminal
- ❌ **Limited terminals**: Basic terminal emulators, some SSH sessions
- **Graceful degradation**: Applications should work without focus events

### Event Flow

1. **User switches away**: Terminal sends blur escape sequence
2. **Framework detection**: bubbletea-rs converts to `BlurMsg`  
3. **Model update**: `focused = false`
4. **View refresh**: UI shows blurred state
5. **User returns**: Terminal sends focus escape sequence
6. **Framework detection**: bubbletea-rs converts to `FocusMsg`
7. **Model update**: `focused = true`
8. **View refresh**: UI shows focused state

### Runtime Toggle Feature

The example includes a toggle feature to demonstrate enabling/disabling focus reporting:

```rust
if self.keys.toggle.matches(key_msg) {
    self.reporting = !self.reporting;
}

// In view()
if self.reporting {
    // Show focus state
} else {
    // Hide focus information
}
```

This is useful for:
- Testing applications with/without focus events
- Debugging focus-related issues
- User preference settings

## Related Examples

- **[simple](../simple/)** - Basic event handling patterns
- **[spinner](../spinner/)** - Example where focus could pause animations
- **[realtime](../realtime/)** - Another real-time event example

## Files

- `main.rs` — Complete focus/blur event handling
- `Cargo.toml` — Dependencies and build configuration
- `README.md` — This documentation

## Usage Notes

- Not all terminal emulators support focus reporting
- Focus events work best in graphical terminals (not TTY)
- Consider making focus-dependent features optional for compatibility
- Test with your target deployment environments