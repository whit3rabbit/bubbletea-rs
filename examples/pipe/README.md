# Pipe

A demonstration of stdin pipe handling, showing how to build terminal applications that can receive data through Unix pipes, process piped input, and integrate with shell workflows and command-line tools.

## Features

- **Stdin Pipe Detection**: Automatically detects piped input vs interactive mode
- **Text Input Widget**: Uses bubbletea-widgets for text editing
- **Pipe Data Processing**: Reads and processes data from stdin pipes
- **Shell Integration**: Works seamlessly with Unix pipes and redirects
- **Interactive Editing**: Edit piped content after receiving it
- **Exit Handling**: Multiple ways to exit (Enter, Esc, Ctrl+C)

## Running the Example

From the repository root:

**With piped input:**
```bash
echo "Hello, world!" | cargo run --example pipe
cat README.md | cargo run --example pipe
ls -la | cargo run --example pipe
```

**Interactive mode (will show error and exit):**
```bash
cargo run --example pipe
# Shows: "Try piping in some text."
```

**Controls:**
- Type to edit the piped content
- `Enter` - Save and quit
- `Esc` / `Ctrl+C` - Quit without saving

## What this demonstrates

### Key Concepts for Beginners

**Unix Pipe Integration**: This example shows how to:
1. Detect when data is being piped into your application
2. Read stdin data before starting the TUI
3. Process piped content and make it editable
4. Build applications that work well in shell pipelines
5. Handle both piped and interactive execution modes

**TTY vs Non-TTY**: Demonstrates the difference between terminal (TTY) and pipe (non-TTY) input modes.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
```

**Text Input Widget:**
```rust
use bubbletea_widgets::textinput;
```

- `textinput::new()`: Create editable text input widget
- `set_value()`: Pre-populate with piped content
- `cursor_end()`: Position cursor at end of text

**Standard I/O:**
```rust
use std::io::{self, Read};
```

### Architecture Walkthrough

#### Pipe Detection and Reading

```rust
fn read_piped_input() -> Result<String, io::Error> {
    use std::io::IsTerminal;
    
    // Check if stdin is a terminal (TTY) or pipe
    if io::stdin().is_terminal() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No piped input detected"
        ));
    }
    
    // Read all data from stdin pipe
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    
    // Trim trailing newlines for cleaner display
    Ok(buffer.trim_end().to_string())
}
```

#### Model Initialization

```rust
fn init() -> (Self, Option<Cmd>) {
    // Read piped input before starting TUI
    let piped_input = read_piped_input().unwrap_or_else(|_| {
        eprintln!("Try piping in some text.");
        std::process::exit(1);
    });
    
    // Create model with piped content pre-loaded
    let model = PipeModel::new(piped_input);
    (model, None)
}
```

#### Text Input Configuration

```rust
fn new(initial_value: String) -> Self {
    let mut textinput = textinput::new();
    
    // Configure for pipe content editing
    textinput.set_width(48);              // Fixed width
    textinput.set_value(&initial_value);  // Pre-populate with piped data
    textinput.cursor_end();               // Position cursor at end
    
    Self { user_input: textinput, /* ... */ }
}
```

### Rust-Specific Patterns

**TTY Detection:**
```rust
use std::io::IsTerminal;

if io::stdin().is_terminal() {
    // Interactive mode - no pipe
    return Err(io::Error::new(io::ErrorKind::InvalidInput, "No piped input"));
} else {
    // Pipe detected - read data
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
}
```

**Error Handling:**
```rust
let piped_input = read_piped_input().unwrap_or_else(|_| {
    eprintln!("Try piping in some text.");
    std::process::exit(1);
});
```

Exit immediately if no pipe is detected.

**String Processing:**
```rust
Ok(buffer.trim_end().to_string())  // Remove trailing whitespace
```

Clean up piped content for better display.

**Widget Integration:**
```rust
// Forward messages to text input widget
return self.user_input.update(msg);
```

### Shell Pipeline Integration

**Common Usage Patterns:**

**Text Processing:**
```bash
# Edit command output
ls -la | cargo run --example pipe

# Process file content
cat config.txt | cargo run --example pipe

# Edit grep results
grep -n "TODO" src/*.rs | cargo run --example pipe
```

**Data Transformation:**
```bash
# Process JSON
curl -s api.example.com/data | jq '.' | cargo run --example pipe

# Edit CSV data
cut -d',' -f1,3 data.csv | cargo run --example pipe

# Process log files
tail -f /var/log/app.log | cargo run --example pipe
```

**Command Chaining:**
```bash
# Multi-stage pipeline
cat input.txt | grep "pattern" | cargo run --example pipe > output.txt

# With other tools
find . -name "*.rs" | cargo run --example pipe | xargs wc -l
```

### Real-world Applications

**Text Editor Filter:**
```rust
// Edit and transform piped text
struct TextFilter {
    content: String,
    transformations: Vec<Transform>,
}

impl Model for TextFilter {
    fn init() -> (Self, Option<Cmd>) {
        let content = read_piped_input()
            .unwrap_or_else(|_| std::process::exit(1));
        
        let model = Self {
            content,
            transformations: Vec::new(),
        };
        
        (model, None)
    }
}
```

**Log Viewer:**
```rust
// Interactive log analysis
struct LogViewer {
    lines: Vec<String>,
    filter: String,
    current_line: usize,
}

impl LogViewer {
    fn from_pipe() -> Result<Self, io::Error> {
        let input = read_piped_input()?;
        let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
        
        Ok(Self {
            lines,
            filter: String::new(),
            current_line: 0,
        })
    }
}
```

**Data Formatter:**
```rust
// Format structured data
struct DataFormatter {
    raw_data: String,
    format: OutputFormat,
    parsed_data: Value,
}

enum OutputFormat {
    Json,
    Yaml,
    Table,
    CSV,
}
```

### Error Handling Strategies

**Graceful Pipe Failure:**
```rust
fn init() -> (Self, Option<Cmd>) {
    match read_piped_input() {
        Ok(content) => {
            let model = PipeModel::new(content);
            (model, None)
        }
        Err(_) => {
            // Could provide default content or exit
            eprintln!("Usage: echo 'text' | {} pipe", env!("CARGO_PKG_NAME"));
            std::process::exit(1);
        }
    }
}
```

**Large Input Handling:**
```rust
fn read_piped_input_limited(max_size: usize) -> Result<String, io::Error> {
    if io::stdin().is_terminal() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No pipe"));
    }
    
    let mut buffer = Vec::new();
    let mut handle = io::stdin().take(max_size as u64);
    handle.read_to_end(&mut buffer)?;
    
    String::from_utf8(buffer)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))
}
```

**Binary Data Detection:**
```rust
fn is_binary_data(data: &[u8]) -> bool {
    data.iter().any(|&byte| byte < 32 && !matches!(byte, b'\n' | b'\r' | b'\t'))
}

fn read_text_input() -> Result<String, io::Error> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    
    if is_binary_data(&buffer) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Binary data not supported"
        ));
    }
    
    String::from_utf8(buffer)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))
}
```

### Performance Considerations

**Streaming vs Buffering:**
```rust
// For large inputs, consider streaming
use std::io::BufRead;

fn read_piped_lines() -> Result<Vec<String>, io::Error> {
    let stdin = io::stdin();
    let lines: Result<Vec<_>, _> = stdin.lock().lines().collect();
    lines
}
```

**Memory Usage:**
- Be mindful of large piped inputs
- Consider processing data in chunks
- Implement pagination for very large datasets

### Testing Pipe Functionality

```bash
# Test with various inputs
echo "Hello" | cargo run --example pipe
printf "Multi\nLine\nText" | cargo run --example pipe
cat /dev/null | cargo run --example pipe  # Empty input
yes | head -100 | cargo run --example pipe  # Large input

# Test error conditions
cargo run --example pipe  # No pipe (should fail)
echo -n | cargo run --example pipe  # Empty pipe
```

### Platform Compatibility

**Unix/Linux/macOS:**
- ✅ Full pipe support
- ✅ TTY detection works correctly
- ✅ Standard shell integration

**Windows:**
- ✅ PowerShell pipe support
- ✅ Command prompt pipes
- ⚠️ Some TTY detection differences

### Shell Integration Best Practices

**Documentation:**
```bash
# Include usage examples in your app
cargo run --example pipe --help
# Should show: "Usage: command | pipe [options]"
```

**Exit Codes:**
```rust
// Use appropriate exit codes for shell integration
match result {
    Ok(_) => std::process::exit(0),      // Success
    Err(_) if no_pipe => std::process::exit(2),  // Usage error  
    Err(_) => std::process::exit(1),     // General error
}
```

**Output Handling:**
```rust
// Write results to stdout for further piping
fn save_and_quit(&self) -> Cmd {
    println!("{}", self.user_input.value());
    quit()
}
```

## Related Examples

- **[textinput](../textinput/)** - Text input widget usage
- **[file-picker](../file-picker/)** - Another data input example
- **[exec](../exec/)** - Process integration patterns

## Files

- `main.rs` — Complete pipe input handling with text editing
- `Cargo.toml` — Dependencies including bubbletea-widgets
- `README.md` — This documentation

## Implementation Tips

- Always check for TTY vs pipe input mode
- Handle empty pipe input gracefully
- Consider memory usage for large piped data
- Test with various shell environments
- Provide clear error messages for incorrect usage
- Support both editing and pass-through modes