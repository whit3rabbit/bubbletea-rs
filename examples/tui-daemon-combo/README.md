# TUI Daemon Combo

<img width="800" src="./tui-daemon-combo.gif" />

## Overview

This example demonstrates how to build a dual-mode application that can run either as an interactive TUI or as a headless daemon. It automatically detects the environment (TTY vs non-TTY) and adapts accordingly, making it perfect for applications that need to work both interactively and in automated environments like CI/CD pipelines.

## Key Features

- **Dual-Mode Operation**: Automatically detects TTY and switches between TUI and daemon modes
- **Command-Line Interface**: Supports `-d` flag to force daemon mode
- **Intelligent Logging**: Different logging strategies for TUI vs daemon modes
- **Spinner Animation**: Smooth loading animation with custom styling
- **Work Simulation**: Realistic async task processing with random timing
- **Rolling Buffer**: Displays last 5 completed tasks with emoji indicators
- **Environment Detection**: Uses libc `isatty()` for TTY detection

## How It Works

The application performs the same core functionality (simulated work with progress indication) but presents it differently based on the execution context:

- **TUI Mode**: Visual interface with spinner, progress updates, and interactive controls
- **Daemon Mode**: Headless operation with structured logging to stderr

The program automatically chooses the appropriate mode by detecting if stdout is connected to a terminal (TTY).

## Code Structure

### Model (`TuiDaemonModel`)
- `spinner_frame: usize` - Current spinner animation frame
- `results: Vec<WorkResult>` - Rolling buffer of completed tasks
- `quitting: bool` - Exit state management

### Key Components

1. **TTY Detection**
   ```rust
   use std::os::unix::io::AsRawFd;
   
   fn is_tty() -> bool {
       let stdout_fd = io::stdout().as_raw_fd();
       unsafe { libc::isatty(stdout_fd) == 1 }
   }
   ```

2. **Dual Logging Configuration**
   ```rust
   fn setup_logging(daemon_mode: bool) {
       if daemon_mode {
           // Log to stderr for daemon mode
           env_logger::Builder::from_default_env()
               .target(env_logger::Target::Stderr)
               .init();
       } else {
           // Discard logs in TUI mode
           env_logger::Builder::from_default_env()
               .target(env_logger::Target::Pipe(Box::new(io::sink())))
               .init();
       }
   }
   ```

3. **Spinner Animation**
   ```rust
   fn spinner_frames() -> &'static [&'static str] {
       &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
   }
   
   fn current_spinner(&self) -> String {
       let frames = Self::spinner_frames();
       let frame = frames[self.spinner_frame % frames.len()];
       let style = Style::new().foreground(Color::from("206")); // Pink
       style.render(frame)
   }
   ```

4. **Async Work Simulation**
   ```rust
   fn run_pretend_process() -> Cmd {
       Box::pin(async {
           let pause = Duration::from_millis(rng.gen_range(100..=999));
           let emoji = random_emoji();
           
           tokio::time::sleep(pause).await;
           
           Some(Box::new(ProcessFinishedMsg {
               duration: pause,
               emoji,
           }) as Msg)
       })
   }
   ```

## API Usage

### Program Builder Configuration
```rust
let mut builder = Program::<TuiDaemonModel>::builder()
    .signal_handler(true);

if force_daemon {
    builder = builder.without_renderer(); // Disable TUI rendering
}

let program = builder.build()?;
```

### Command Batching
```rust
use bubbletea_rs::batch;

let cmds = vec![
    tick(Duration::from_millis(100), |_| Box::new(SpinnerTickMsg) as Msg),
    run_pretend_process(),
];

(model, Some(batch(cmds)))
```

### Async Commands
- `tick()` - Single-shot timer for spinner animation
- `batch()` - Execute multiple commands concurrently
- Custom async commands for work simulation

## Running the Example

### Interactive TUI Mode
```bash
cd examples/tui-daemon-combo
cargo run
```

### Daemon Mode
```bash
# Force daemon mode with flag
cargo run -- -d

# Or pipe/redirect to trigger daemon mode automatically
cargo run > output.log 2>&1

# Background execution
cargo run &
```

### Command Line Options
- `-d, --daemon`: Force daemon mode regardless of TTY detection
- `-h, --help`: Show help information

## Key Bindings (TUI Mode Only)

- **Any Key**: Quit the application

## Implementation Notes

### TTY Detection Logic
The program uses Unix `isatty()` to detect if stdout is connected to a terminal:
- **TTY detected**: Run in interactive TUI mode
- **No TTY**: Run in headless daemon mode (pipes, redirects, background execution)

### Logging Strategy
- **TUI Mode**: Logs are discarded to avoid interfering with the visual interface
- **Daemon Mode**: Logs go to stderr for proper daemon behavior and debugging

### Rolling Buffer Implementation
The results buffer maintains exactly 5 entries:
```rust
fn add_result(&mut self, result: WorkResult) {
    // Shift all results left
    for i in 1..self.results.len() {
        self.results[i - 1] = self.results[i].clone();
    }
    // Add new result at the end
    if let Some(last) = self.results.last_mut() {
        *last = result;
    }
}
```

### Work Simulation
Each simulated task:
- Takes 100-999ms (random duration)
- Uses a random emoji from a curated set
- Logs completion with timing information
- Immediately schedules the next task

### Error Handling
The example demonstrates proper error handling for both modes:
- TUI mode errors: Display to user via eprintln
- Daemon mode errors: Log via warn! macro

This example showcases how to build robust CLI tools that work seamlessly in both interactive and automated environments, a common requirement for production software.
