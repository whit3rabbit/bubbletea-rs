# Sequence

A demonstration of command sequencing and orchestration, showing how to run commands in specific order, combine sequential and concurrent execution, and build complex async workflows in terminal applications.

## Features

- **Sequential Command Execution**: Run commands one after another in order
- **Concurrent Batch Execution**: Run multiple commands simultaneously within sequences
- **Command Orchestration**: Combine `sequence()` and `batch()` for complex workflows
- **Automatic Termination**: Program exits automatically after sequence completion
- **Output to Terminal**: Uses `println()` commands for direct output

## Running the Example

From the repository root:

```bash
cargo run --example sequence
```

**Expected Output:**
```
A
B
C
Z
```

Note: A, B, C appear simultaneously (concurrent batch), then Z appears after they complete (sequential).

**Controls:**
- Any key - Quit early (before sequence completes)

## What this demonstrates

### Key Concepts for Beginners

**Command Orchestration**: This example shows how to:
1. Run multiple commands in a specific order using `sequence()`
2. Run commands concurrently using `batch()` within sequences
3. Combine sequential and concurrent patterns for complex workflows
4. Handle automatic program flow without user interaction

**Async Coordination**: Demonstrates coordinating async operations with precise timing and dependency management.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{batch, println, quit, sequence, Cmd, KeyMsg, Model, Msg, Program};
```

- `sequence(Vec<Cmd>)`: Run commands one after another
- `batch(Vec<Cmd>)`: Run commands concurrently
- `println(String)`: Output text directly to terminal
- Command composition patterns

### Architecture Walkthrough

#### Command Sequence Design

The example creates a specific execution pattern:

```rust
let sequence_cmd = sequence(vec![
    // Step 1: Run A, B, C concurrently
    batch(vec![
        println("A".to_string()),
        println("B".to_string()),
        println("C".to_string()),
    ]),
    
    // Step 2: Run Z after batch completes
    println("Z".to_string()),
    
    // Step 3: Quit the program
    quit(),
]);
```

#### Execution Flow

1. **Batch Phase**: A, B, C execute simultaneously
2. **Wait for Completion**: Sequence waits for all batch items to finish
3. **Sequential Phase**: Z executes only after batch completes
4. **Termination**: Program quits automatically

#### Minimal Model

```rust
struct SequenceModel;  // Unit struct - no state needed

impl Model for SequenceModel {
    fn init() -> (Self, Option<Cmd>) {
        (SequenceModel, Some(sequence_cmd))  // Start sequence immediately
    }
    
    fn view(&self) -> String {
        String::new()  // Empty - output handled by println commands
    }
}
```

### Rust-Specific Patterns

**Command Vector Creation:**
```rust
sequence(vec![
    cmd1,
    cmd2,
    cmd3,
])
```

Type-safe command composition with compile-time checking.

**Nested Command Patterns:**
```rust
sequence(vec![
    batch(vec![cmd_a, cmd_b, cmd_c]),  // Concurrent inner batch
    cmd_z,                             // Sequential after batch
])
```

**String Ownership:**
```rust
println("A".to_string())  // Owned strings for command lifetime
```

### Command Execution Patterns

**Sequential Only:**
```rust
sequence(vec![
    println("First".to_string()),
    println("Second".to_string()),
    println("Third".to_string()),
])
```

Output: `First`, then `Second`, then `Third` (in strict order).

**Concurrent Only:**
```rust
batch(vec![
    println("A".to_string()),
    println("B".to_string()),
    println("C".to_string()),
])
```

Output: `A`, `B`, `C` appear simultaneously (any order).

**Mixed Patterns:**
```rust
sequence(vec![
    println("Start".to_string()),
    batch(vec![
        println("Concurrent 1".to_string()),
        println("Concurrent 2".to_string()),
    ]),
    println("End".to_string()),
])
```

Output: `Start`, then (`Concurrent 1` + `Concurrent 2` simultaneously), then `End`.

### Real-world Applications

**Application Startup:**
```rust
sequence(vec![
    println("Initializing...".to_string()),
    batch(vec![
        load_configuration(),
        connect_to_database(),
        setup_logging(),
    ]),
    println("Ready!".to_string()),
    start_main_loop(),
])
```

**File Processing Pipeline:**
```rust
sequence(vec![
    println("Starting batch process...".to_string()),
    batch(vec![
        process_file("file1.txt".to_string()),
        process_file("file2.txt".to_string()),
        process_file("file3.txt".to_string()),
    ]),
    generate_report(),
    println("Process complete!".to_string()),
])
```

**Network Operations:**
```rust
sequence(vec![
    println("Fetching data...".to_string()),
    batch(vec![
        fetch_user_data(),
        fetch_settings(),
        fetch_permissions(),
    ]),
    validate_data(),
    update_ui(),
])
```

**Testing Workflows:**
```rust
sequence(vec![
    println("Running tests...".to_string()),
    batch(vec![
        run_unit_tests(),
        run_integration_tests(),
        run_linting(),
    ]),
    generate_coverage_report(),
    println("Testing complete!".to_string()),
])
```

### Command Factory Patterns

```rust
fn create_startup_sequence() -> Cmd {
    sequence(vec![
        log_message("Application starting"),
        batch(vec![
            initialize_subsystem("database"),
            initialize_subsystem("cache"),
            initialize_subsystem("auth"),
        ]),
        log_message("Startup complete"),
    ])
}

fn initialize_subsystem(name: &str) -> Cmd {
    let system_name = name.to_string();
    Box::pin(async move {
        // Simulate subsystem initialization
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("{} initialized", system_name);
        None
    })
}
```

### Error Handling in Sequences

```rust
fn create_robust_sequence() -> Cmd {
    sequence(vec![
        retry_command(risky_operation(), 3),
        fallback_command(
            primary_operation(),
            backup_operation(),
        ),
        always_cleanup(),
    ])
}

fn retry_command(cmd: Cmd, attempts: u32) -> Cmd {
    Box::pin(async move {
        for attempt in 1..=attempts {
            match execute_command(cmd).await {
                Ok(result) => return Some(result),
                Err(_) if attempt < attempts => continue,
                Err(e) => return Some(Box::new(ErrorMsg(e.to_string())) as Msg),
            }
        }
        None
    })
}
```

### Performance Characteristics

**Sequential Execution:**
- Commands wait for previous completion
- Predictable timing and resource usage
- Total time = sum of individual command times

**Concurrent Execution:**
- Commands run simultaneously
- Higher resource usage during execution
- Total time = max of individual command times

**Memory Usage:**
- Commands stored in vector until execution
- Minimal overhead for coordination
- Clean up after completion

### Testing Sequence Behavior

```rust
#[tokio::test]
async fn test_sequence_order() {
    let mut results = Vec::new();
    
    let sequence_cmd = sequence(vec![
        record_event(&mut results, "First"),
        record_event(&mut results, "Second"),
        record_event(&mut results, "Third"),
    ]);
    
    execute_command(sequence_cmd).await;
    
    assert_eq!(results, vec!["First", "Second", "Third"]);
}

#[tokio::test]
async fn test_batch_concurrency() {
    let start_time = Instant::now();
    
    let batch_cmd = batch(vec![
        delay_command(Duration::from_millis(100)),
        delay_command(Duration::from_millis(100)),
        delay_command(Duration::from_millis(100)),
    ]);
    
    execute_command(batch_cmd).await;
    let elapsed = start_time.elapsed();
    
    // Should take ~100ms (concurrent) not ~300ms (sequential)
    assert!(elapsed < Duration::from_millis(200));
}
```

### Command Composition Patterns

**Pipeline Pattern:**
```rust
sequence(vec![
    input_stage(),
    processing_stage(),
    output_stage(),
])
```

**Fan-out/Fan-in Pattern:**
```rust
sequence(vec![
    prepare_data(),
    batch(vec![  // Fan-out
        process_chunk_1(),
        process_chunk_2(),
        process_chunk_3(),
    ]),
    merge_results(),  // Fan-in
])
```

**Conditional Sequences:**
```rust
fn create_conditional_sequence(condition: bool) -> Cmd {
    let mut commands = vec![initial_setup()];
    
    if condition {
        commands.push(batch(vec![
            optional_task_1(),
            optional_task_2(),
        ]));
    }
    
    commands.push(finalization());
    sequence(commands)
}
```

### Best Practices

**Sequence Design:**
- Keep sequences focused on single workflows
- Use batch for truly independent operations
- Avoid deeply nested sequences for readability

**Error Handling:**
- Plan for command failures in sequences
- Implement cleanup operations
- Consider retry strategies

**Performance:**
- Use batch for I/O-bound operations
- Be mindful of resource contention
- Profile complex sequences for bottlenecks

**Testing:**
- Test sequence order with deterministic operations
- Verify concurrent execution timing
- Mock external dependencies

## Related Examples

- **[send-msg](../send-msg/)** - Background task coordination
- **[realtime](../realtime/)** - Concurrent async operations
- **[progress-download](../progress-download/)** - Sequential progress updates

## Files

- `main.rs` — Complete command sequencing implementation
- `Cargo.toml` — Dependencies and build configuration
- `README.md` — This documentation

## Implementation Tips

- Use `sequence()` when order matters, `batch()` when it doesn't
- Combine patterns for complex workflows
- Consider error handling and cleanup in sequences
- Test timing-dependent behavior carefully
- Profile performance of concurrent operations