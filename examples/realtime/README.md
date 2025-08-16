# Realtime

<img width="1200" src="./realtime.gif" />

A demonstration of real-time event handling with simulated activity, showing how to manage concurrent operations, async message generation, and live activity counters in terminal applications.

## Features

- **Simulated Real-time Activity**: Random events every 100-1000ms
- **Activity Counter**: Tracks total number of events received
- **Spinner Animation**: Visual indicator showing the system is active
- **Concurrent Operations**: Spinner and activity events run simultaneously
- **Async Command System**: Demonstrates proper async/await patterns
- **Any Key Quit**: Simple exit mechanism

## Running the Example

From the repository root:

```bash
cargo run --example realtime
```

**Controls:**
- Any key - Quit the application
- Watch events arrive at random intervals

## What this demonstrates

### Key Concepts for Beginners

**Real-time Applications**: This example shows patterns for applications that need to:
- Handle events as they arrive (not user-initiated)
- Display live activity counters or status
- Maintain visual feedback during waiting periods
- Manage multiple concurrent background operations

**Async Programming**: Demonstrates using Rust's async/await system within the MVU pattern.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{batch, quit, tick, Cmd, KeyMsg, Model, Msg, Program};
```

- `batch()`: Run multiple commands concurrently
- Async command patterns with `Box::pin(async move { ... })`
- Random timing simulation with tokio::time::sleep

### Architecture Walkthrough

#### Model Structure
```rust
pub struct RealtimeModel {
    spinner_frame: usize, // Current spinner animation frame
    responses: u32,       // Count of activity events received
    quitting: bool,       // Exit state tracking
}
```

#### Real-time Event Simulation

The core pattern simulates irregular activity:

```rust
fn listen_for_activity() -> Cmd {
    Box::pin(async move {
        use rand::Rng;
        loop {
            // Wait random time between 100-1000ms
            let delay_ms = rand::rng().random_range(100..=1000);
            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            
            // Send activity event
            return Some(Box::new(ResponseMsg) as Msg);
        }
    })
}
```

#### Concurrent Command Management

The application runs two independent async operations:

```rust
fn init() -> (Self, Option<Cmd>) {
    let model = Self::new();
    
    // Start both spinner animation and activity listening
    let initial_cmd = batch(vec![
        tick(Duration::from_millis(100), |_| Box::new(SpinnerTickMsg) as Msg),
        wait_for_activity(), // Start listening for events
    ]);
    
    (model, Some(initial_cmd))
}
```

#### Message Handling Pattern

```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    // Handle activity events
    if msg.downcast_ref::<ResponseMsg>().is_some() {
        self.responses += 1;
        // Restart activity listener for next event
        return Some(wait_for_activity());
    }
    
    // Handle spinner animation
    if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
        self.spinner_frame = (self.spinner_frame + 1) % Self::spinner_frames().len();
        // Continue spinner animation
        return Some(tick(Duration::from_millis(100), |_| {
            Box::new(SpinnerTickMsg) as Msg
        }));
    }
    
    // Handle quit on any key
    if msg.downcast_ref::<KeyMsg>().is_some() {
        self.quitting = true;
        return Some(quit());
    }
    
    None
}
```

### Rust-Specific Patterns

**Async Command Creation:**
```rust
fn listen_for_activity() -> Cmd {
    Box::pin(async move {
        // Async operations here
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
        Some(Box::new(ResponseMsg) as Msg)
    })
}
```

**Random Number Generation:**
```rust
use rand::Rng;
let delay_ms = rand::rng().random_range(100..=1000);
//                                        ^--^ inclusive range
```

**Command Batching:**
```rust
// Run multiple commands concurrently
Some(batch(vec![
    spinner_cmd,
    activity_cmd,
]))
```

**Event Counting:**
```rust
if msg.downcast_ref::<ResponseMsg>().is_some() {
    self.responses += 1;  // Thread-safe increment (single-threaded MVU)
    return Some(wait_for_activity()); // Restart listener
}
```

### Concurrency Model

**MVU Threading:**
- All message handling is single-threaded
- Commands run concurrently in tokio runtime
- Messages arrive in event loop when commands complete

**Event Flow:**
1. **Activity Command**: Runs async, sleeps random time, sends ResponseMsg
2. **Spinner Command**: Runs async, sleeps 100ms, sends SpinnerTickMsg  
3. **Message Handler**: Processes messages sequentially, schedules new commands
4. **Repeat**: Both commands restart after handling their messages

### Performance Characteristics

**Concurrent Efficiency:**
- Two independent async operations
- No blocking in message handler
- Commands use tokio's efficient sleep timers
- Minimal CPU usage between events

**Memory Usage:**
- Fixed model size regardless of event count
- Commands are short-lived async tasks
- No event accumulation or buffering

### Real-world Applications

**System Monitoring:**
```rust
// Monitor system resources
fn monitor_cpu_usage() -> Cmd {
    Box::pin(async move {
        let usage = get_cpu_usage().await;
        Some(Box::new(CpuUsageMsg(usage)) as Msg)
    })
}
```

**Network Activity:**
```rust
// Listen for incoming connections
fn listen_for_connections() -> Cmd {
    Box::pin(async move {
        let connection = listener.accept().await?;
        Some(Box::new(NewConnectionMsg(connection)) as Msg)
    })
}
```

**File System Watching:**
```rust
// Watch for file changes
fn watch_file_changes() -> Cmd {
    Box::pin(async move {
        let event = watcher.next().await;
        Some(Box::new(FileChangedMsg(event)) as Msg)
    })
}
```

### Command Lifecycle

**Command Creation:** Functions return `Cmd` (boxed async future)
**Command Execution:** Framework runs commands in tokio runtime
**Message Generation:** Commands complete and optionally return messages
**Message Processing:** MVU update() handles messages and may create new commands
**Command Restart:** New commands continue the cycle

### Error Handling Patterns

```rust
fn fallible_activity() -> Cmd {
    Box::pin(async move {
        match some_operation().await {
            Ok(data) => Some(Box::new(SuccessMsg(data)) as Msg),
            Err(e) => Some(Box::new(ErrorMsg(e.to_string())) as Msg),
        }
    })
}
```

### Testing Real-time Behavior

**Deterministic Testing:**
```rust
// Use fixed delays for tests
fn test_activity() -> Cmd {
    Box::pin(async move {
        tokio::time::sleep(Duration::from_millis(100)).await; // Fixed delay
        Some(Box::new(ResponseMsg) as Msg)
    })
}
```

## Related Examples

- **[spinner](../spinner/)** - Animation without real-time events
- **[debounce](../debounce/)** - Timer-based message patterns  
- **[progress-animated](../progress-animated/)** - Another concurrent animation example

## Files

- `main.rs` — Complete real-time activity simulation
- `Cargo.toml` — Dependencies including rand and tokio
- `realtime.gif` — Demo showing live event counting
- `README.md` — This documentation

## Usage Patterns

- System monitoring dashboards
- Chat applications with live message feeds
- Real-time data processing displays
- Background task status indicators
- Live log viewers or activity feeds