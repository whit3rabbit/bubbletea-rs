# Suspend

A demonstration of process suspension and resumption, showing how to handle `Ctrl+Z` signals, suspend the application, and cleanly resume when brought back to the foreground in terminal applications.

## Features

- **Process Suspension**: Handle `Ctrl+Z` to suspend the application
- **Automatic Resume**: Detect and handle resume events when brought to foreground
- **Signal Handling**: Proper integration with Unix job control
- **State Management**: Track suspension state and update UI accordingly
- **Multiple Exit Options**: Support for quit (`q`) and interrupt (`Ctrl+C`)
- **Terminal Integration**: Clean terminal state management during suspend/resume

## Running the Example

From the repository root:

```bash
cargo run --example suspend
```

**Testing Suspend/Resume:**
1. Run the example
2. Press `Ctrl+Z` to suspend the process
3. You'll be back at your shell prompt
4. Type `fg` to resume the process
5. The application resumes with updated state

**Controls:**
- `Ctrl+Z` - Suspend the application (job control)
- `fg` (from shell) - Resume the suspended application
- `q` / `Esc` - Quit normally
- `Ctrl+C` - Interrupt and quit

## What this demonstrates

### Key Concepts for Beginners

**Process Suspension**: This example shows how to:
1. Handle Unix job control signals properly
2. Suspend applications gracefully without losing state
3. Resume applications and refresh the display
4. Integrate with shell job control mechanisms
5. Manage terminal state across suspend/resume cycles

**Signal Handling**: Demonstrates proper handling of `SIGTSTP` (suspend) and `SIGCONT` (continue) signals in terminal applications.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{interrupt, quit, suspend, Cmd, KeyMsg, Model, Msg, Program, ResumeMsg};
```

- `suspend()`: Command to suspend the application
- `ResumeMsg`: Message sent when application resumes
- `interrupt()`: Clean interrupt handling for `Ctrl+C`
- Standard signal integration

### Architecture Walkthrough

#### Model Structure
```rust
struct SuspendModel {
    quitting: bool,     // Tracking exit state
    suspending: bool,   // Tracking suspension state
}
```

Simple state tracking for demonstration purposes.

#### Suspension Handling
```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
        match key_msg.key {
            // Handle Ctrl+Z suspension
            KeyCode::Char('z') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                self.suspending = true;
                return Some(suspend());  // Suspend the process
            }
            _ => {}
        }
    }
    
    // Handle resume from suspension
    if msg.downcast_ref::<ResumeMsg>().is_some() {
        self.suspending = false;  // Clear suspension state
        return None;
    }
    
    None
}
```

#### Signal Integration Pattern
```rust
// Different exit patterns for different signals
KeyCode::Char('q') => {
    self.quitting = true;
    return Some(quit());     // Normal graceful exit
}

KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
    self.quitting = true;
    return Some(interrupt()); // Signal-based interrupt
}

KeyCode::Char('z') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
    self.suspending = true;
    return Some(suspend());   // Process suspension
}
```

### Rust-Specific Patterns

**Signal Command Creation:**
```rust
return Some(suspend());  // Framework handles SIGTSTP signal
```

The framework abstracts Unix signal handling.

**Resume Message Detection:**
```rust
if msg.downcast_ref::<ResumeMsg>().is_some() {
    self.suspending = false;
    // Application resumed, update state accordingly
}
```

**State Flag Management:**
```rust
self.suspending = true;   // Set before suspension
self.suspending = false;  // Clear on resume
```

Simple boolean flags track suspension state.

### Process Lifecycle

**Normal Operation:**
1. Application runs normally
2. Processes user input and updates display
3. Handles keyboard commands

**Suspension Flow:**
1. User presses `Ctrl+Z`
2. Application sets `suspending = true`
3. `suspend()` command issued
4. Framework sends `SIGTSTP` to process
5. Process suspends, terminal returns to shell
6. Shell shows job control status

**Resume Flow:**
1. User types `fg` in shell
2. Shell sends `SIGCONT` to process
3. Framework detects resume and sends `ResumeMsg`
4. Application sets `suspending = false`
5. Display refreshes with current state
6. Normal operation continues

### Terminal State Management

**During Suspension:**
- Terminal control returns to shell
- Application state preserved in memory
- No CPU usage while suspended
- Terminal settings restored

**On Resume:**
- Terminal control returns to application
- Previous terminal settings restored
- Display refreshed immediately
- Input handling resumes

### Shell Integration

**Job Control Commands:**

```bash
# Suspend running application
Ctrl+Z

# List jobs
jobs

# Resume in foreground
fg

# Resume in background (if applicable)
bg

# Kill suspended job
kill %1
```

### Error Handling

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<SuspendModel>::builder()
        .signal_handler(true)  // IMPORTANT: Enable signal handling
        .build()?;
    
    // Handle different exit scenarios
    match program.run().await {
        Ok(_) => {
            // Normal exit
            println!("Goodbye!");
        }
        Err(bubbletea_rs::Error::Interrupted) => {
            // Ctrl+C interrupt - different exit code
            process::exit(130);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
    
    Ok(())
}
```

### Platform Compatibility

**Unix/Linux/macOS:**
- ✅ Full suspend/resume support
- ✅ Job control integration
- ✅ Signal handling works correctly

**Windows:**
- ⚠️ Limited job control support
- ⚠️ `Ctrl+Z` may not suspend process
- ✅ `Ctrl+C` interrupt handling works

### Real-world Applications

**Long-running Processes:**
```rust
// File processing that can be suspended
struct FileProcessor {
    current_file: usize,
    total_files: usize,
    suspended: bool,
}

impl Model for FileProcessor {
    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if msg.downcast_ref::<ResumeMsg>().is_some() {
            self.suspended = false;
            // Resume processing where we left off
            return Some(continue_processing(self.current_file));
        }
        // ... handle suspension
    }
}
```

**Interactive Applications:**
```rust
// Text editor that preserves state during suspension
struct TextEditor {
    content: Vec<String>,
    cursor: (usize, usize),
    suspended: bool,
}
```

**System Monitors:**
```rust
// Monitoring app that pauses data collection when suspended
struct SystemMonitor {
    collecting_data: bool,
    last_update: SystemTime,
}

impl Model for SystemMonitor {
    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if msg.downcast_ref::<ResumeMsg>().is_some() {
            self.collecting_data = true;
            return Some(resume_data_collection());
        }
        
        if let Some(_) = msg.downcast_ref::<KeyMsg>() {
            if key_is_suspend(key) {
                self.collecting_data = false;
                return Some(suspend());
            }
        }
    }
}
```

### Best Practices

**State Preservation:**
- Save important state before suspension
- Avoid losing user work during suspend/resume
- Handle partial operations gracefully

**Resource Management:**
- Pause expensive operations during suspension
- Release system resources when possible
- Resume operations efficiently

**User Experience:**
- Provide clear feedback about suspension state
- Update display immediately on resume
- Handle multiple suspend/resume cycles

### Testing Suspend/Resume

```bash
# Test basic suspend/resume
cargo run --example suspend
# Press Ctrl+Z
fg

# Test with background jobs
cargo run --example suspend
# Press Ctrl+Z
bg
jobs
fg %1
```

### Implementation Notes

**Signal Handler Requirement:**
```rust
let program = Program::<SuspendModel>::builder()
    .signal_handler(true)  // MUST be enabled for suspend/resume
    .build()?;
```

Without signal handling, suspend/resume won't work properly.

**State Cleanup:**
Always clean up suspension state on resume to prevent UI inconsistencies.

**Terminal Compatibility:**
Test with different terminal emulators as some may handle job control differently.

## Related Examples

- **[exec](../exec/)** - Another example involving process control
- **[focus-blur](../focus-blur/)** - Terminal focus event handling
- **[simple](../simple/)** - Basic signal handling patterns

## Files

- `main.rs` — Complete suspend/resume implementation with signal handling
- `Cargo.toml` — Dependencies and build configuration
- `README.md` — This documentation

## Usage Tips

- Always enable signal handling for suspend/resume functionality
- Test suspension in different terminals and shells
- Handle suspension state in your application logic
- Consider resource management during suspension periods
- Provide user feedback about suspension/resume events