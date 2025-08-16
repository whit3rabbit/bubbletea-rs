# Send Message

<img width="1200" src="./send-msg.gif" />

A demonstration of external message sending patterns, showing how to send messages to a running bubbletea-rs program from background tasks, external processes, or other parts of your application.

## Features

- **External Message Injection**: Send messages from outside the main event loop
- **Background Task Simulation**: Multiple concurrent food preparation tasks
- **Real-time Updates**: Messages appear as they arrive from external sources
- **Visual Feedback**: Spinner animation shows the program is active and waiting
- **Message History**: Display accumulated results from external operations
- **Random Timing**: Simulates real-world async operations with variable duration

## Running the Example

From the repository root:

```bash
cargo run --example send-msg
```

**Controls:**
- `q` / `Ctrl+C` - Quit
- Watch messages arrive from background tasks

## What this demonstrates

### Key Concepts for Beginners

**External Message Patterns**: This example shows how to:
1. Send messages to a running TUI from external sources
2. Handle async operations that report back to the UI
3. Coordinate between background tasks and the main event loop
4. Display real-time updates from multiple concurrent operations

**Use Cases:** Background file processing, network operations, system monitoring, or any scenario where external events need to update the UI.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{batch, quit, tick, Cmd, KeyMsg, Model, Msg, Program};
```

- Messages can be sent from any async context
- `Program` handles external message integration
- Standard MVU pattern processes external messages like user input

**Async Task Integration:**
```rust
use rand::Rng;
use std::time::Duration;
```

### Architecture Walkthrough

#### Message Types

```rust
#[derive(Debug, Clone)]
struct ResultMsg {
    duration: Duration,  // How long the operation took
    food: String,       // What was prepared
}

impl ResultMsg {
    fn to_display_string(&self) -> String {
        if self.duration.is_zero() {
            // Placeholder for empty results
            let dot_style = Style::new().foreground(Color::from("241"));
            dot_style.render(&".".repeat(30))
        } else {
            // Actual result from background task
            format!("🍔 Ate {} {:?}", self.food, self.duration)
        }
    }
}
```

#### Model Structure

```rust
pub struct SendMsgModel {
    spinner_frame: usize,        // Animation state
    results: Vec<ResultMsg>,     // Messages received from external tasks
    quitting: bool,              // Exit state
}
```

#### Background Task Simulation

The example creates multiple concurrent "cooking" tasks:

```rust
fn start_background_tasks() -> Vec<Cmd> {
    let foods = ["hamburgers", "cheeseburgers", "fries"];
    
    foods.iter().map(|food| {
        let food = food.to_string();
        Box::pin(async move {
            // Simulate variable cooking time
            let duration = Duration::from_millis(
                rand::rng().random_range(500..2000)
            );
            tokio::time::sleep(duration).await;
            
            // Send result back to UI
            Some(Box::new(ResultMsg::new(duration, food)) as Msg)
        })
    }).collect()
}
```

#### External Message Handling

```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    // Handle messages from background tasks
    if let Some(result_msg) = msg.downcast_ref::<ResultMsg>() {
        self.results.push(result_msg.clone());
        return None; // No additional commands needed
    }
    
    // Handle spinner animation
    if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
        self.spinner_frame = (self.spinner_frame + 1) % Self::spinner_frames().len();
        return Some(tick(Duration::from_millis(100), |_| {
            Box::new(SpinnerTickMsg) as Msg
        }));
    }
    
    None
}
```

### Rust-Specific Patterns

**Concurrent Task Creation:**
```rust
fn init() -> (Self, Option<Cmd>) {
    let model = Self::new();
    
    // Start spinner and background tasks concurrently
    let mut commands = vec![
        // Spinner animation
        tick(Duration::from_millis(100), |_| Box::new(SpinnerTickMsg) as Msg),
        
        // Initial render trigger
        init_render_cmd(),
    ];
    
    // Add all background tasks
    commands.extend(start_background_tasks());
    
    (model, Some(batch(commands)))
}
```

**Message Cloning:**
```rust
if let Some(result_msg) = msg.downcast_ref::<ResultMsg>() {
    self.results.push(result_msg.clone());  // Store copy of message
}
```

Required because messages are consumed during processing.

**Random Duration Generation:**
```rust
let duration = Duration::from_millis(
    rand::rng().random_range(500..2000)  // 0.5-2 seconds
);
tokio::time::sleep(duration).await;
```

**Async Task Return:**
```rust
Box::pin(async move {
    // Async work here
    tokio::time::sleep(duration).await;
    
    // Return message to main loop
    Some(Box::new(ResultMsg::new(duration, food)) as Msg)
})
```

### Message Flow Diagram

```
Background Task 1 ────┐
Background Task 2 ────┼──→ Message Queue ──→ MVU Update Loop ──→ UI Display
Background Task 3 ────┘
Spinner Timer    ────┘
```

### Real-world Applications

**File Processing:**
```rust
fn process_file(path: PathBuf) -> Cmd {
    Box::pin(async move {
        match std::fs::read_to_string(&path) {
            Ok(content) => Some(Box::new(FileProcessedMsg {
                path,
                line_count: content.lines().count(),
            }) as Msg),
            Err(e) => Some(Box::new(FileErrorMsg {
                path,
                error: e.to_string(),
            }) as Msg),
        }
    })
}
```

**Network Requests:**
```rust
fn fetch_data(url: String) -> Cmd {
    Box::pin(async move {
        match reqwest::get(&url).await {
            Ok(response) => Some(Box::new(DataReceivedMsg {
                url,
                data: response.text().await.unwrap_or_default(),
            }) as Msg),
            Err(e) => Some(Box::new(NetworkErrorMsg {
                url,
                error: e.to_string(),
            }) as Msg),
        }
    })
}
```

**System Monitoring:**
```rust
fn monitor_system() -> Cmd {
    Box::pin(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            
            let cpu_usage = get_cpu_usage().await;
            let memory_usage = get_memory_usage().await;
            
            // Send periodic updates
            return Some(Box::new(SystemStatsMsg {
                cpu: cpu_usage,
                memory: memory_usage,
                timestamp: std::time::SystemTime::now(),
            }) as Msg);
        }
    })
}
```

### Command Batching Strategy

The example uses `batch()` to start multiple concurrent operations:

```rust
let commands = vec![
    spinner_animation_cmd,
    background_task_1,
    background_task_2,
    background_task_3,
    initial_render_cmd,
];

Some(batch(commands))  // All run concurrently
```

### Visual Design

**Result Display:**
```rust
fn view(&self) -> String {
    let mut s = String::from("Waiting for food to finish cooking...\n");
    
    // Show spinner while waiting
    s.push_str(&format!("   {}  \n\n", self.current_spinner_frame()));
    
    // Show results as they arrive
    for result in &self.results {
        s.push_str(&format!("{}\n", result.to_display_string()));
    }
    
    // Fill remaining slots with dots
    let remaining = 3 - self.results.len();
    for _ in 0..remaining {
        let dot_style = Style::new().foreground(Color::from("241"));
        s.push_str(&format!("{}\n", dot_style.render(&".".repeat(30))));
    }
    
    s.push_str("\nPress q to quit\n");
    s
}
```

**Progressive Display:**
```
Waiting for food to finish cooking...

   ⠹  

🍔 Ate hamburgers 1.234s
🍔 Ate fries 0.876s
..............................

Press q to quit
```

### Testing External Messages

```rust
#[tokio::test]
async fn test_external_messages() {
    let mut model = SendMsgModel::new();
    
    // Simulate external message
    let msg = ResultMsg::new(Duration::from_secs(1), "test food".to_string());
    model.update(Box::new(msg));
    
    assert_eq!(model.results.len(), 1);
    assert_eq!(model.results[0].food, "test food");
}
```

### Performance Considerations

**Message Handling:**
- External messages processed same as user input
- No blocking in message handlers
- Concurrent task execution

**Memory Management:**
- Results stored in bounded vector
- Consider implementing result rotation for long-running applications
- Messages are lightweight (Duration + String)

## Related Examples

- **[realtime](../realtime/)** - Another external event example
- **[debounce](../debounce/)** - Timer-based message patterns
- **[progress-download](../progress-download/)** - Progress from background tasks

## Files

- `main.rs` — Complete external message sending implementation
- `Cargo.toml` — Dependencies including tokio and rand
- `send-msg.gif` — Demo showing messages arriving from background tasks
- `README.md` — This documentation

## Usage Patterns

- Progress reporting from background operations
- Real-time data feeds and notifications
- System monitoring and alerts
- Multi-threaded application coordination
- External API integration with status updates