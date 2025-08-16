# Result

<img width="1200" src="./result.gif" />

A demonstration of retrieving values from a bubbletea-rs program after it exits, showing how to capture user selections and return them to the calling code for further processing.

## Features

- **Menu Selection**: Choose from multiple options using keyboard navigation
- **Vi-style Navigation**: Use `j`/`k` or arrow keys for movement
- **Return Value Capture**: Selected choice is available after program exits
- **Multiple Quit Options**: Exit with `q`, `Ctrl+C`, or `Esc`
- **Clean Interface**: Simple list with cursor highlighting

## Running the Example

From the repository root:

```bash
cargo run --example result
```

**Controls:**
- `↑`/`k` - Move cursor up
- `↓`/`j` - Move cursor down  
- `Enter` - Select current option and exit
- `q`/`Ctrl+C`/`Esc` - Quit without selection

## What this demonstrates

### Key Concepts for Beginners

**Program Result Pattern**: This example shows how to:
1. Run a TUI application for user input
2. Capture the final state when the program exits
3. Use that result in the calling code
4. Handle both successful selections and early exits

This pattern is essential for building TUI utilities that need to return values to shell scripts, other programs, or larger applications.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
```

- `Program::run().await?` returns the final model state
- Standard MVU pattern with navigation logic

**Key Binding System:**
```rust
use bubbletea_widgets::key::{matches_binding, new_binding, with_help, with_keys_str, Binding};
```

- `matches_binding()`: Check if key matches binding
- Organized key mappings with help text

### Architecture Walkthrough

#### Model Structure
```rust
struct AppModel {
    cursor: usize,   // Current selection index
    choice: String,  // Final selected value
    keymap: KeyMap,  // Organized key bindings
}
```

#### Menu Navigation Logic
```rust
const CHOICES: &[&str] = &["Taro", "Coffee", "Lychee"];

// Move cursor up with wrapping
if matches_binding(&self.keymap.up, key_msg) {
    if self.cursor == 0 {
        self.cursor = CHOICES.len() - 1; // Wrap to bottom
    } else {
        self.cursor -= 1;
    }
}

// Move cursor down with wrapping
if matches_binding(&self.keymap.down, key_msg) {
    if self.cursor >= CHOICES.len() - 1 {
        self.cursor = 0; // Wrap to top
    } else {
        self.cursor += 1;
    }
}
```

#### Selection and Exit
```rust
// User makes selection
if matches_binding(&self.keymap.enter, key_msg) {
    self.choice = CHOICES[self.cursor].to_string(); // Capture choice
    return Some(quit()); // Exit with selection
}

// User quits without selection
if matches_binding(&self.keymap.quit, key_msg) {
    return Some(quit()); // Exit with empty choice
}
```

#### Result Retrieval Pattern

In `main()`, the final model state is available:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<AppModel>::builder().build()?;
    
    // Run program and get final model state
    let final_model = program.run().await?;
    
    // Use the result!
    if !final_model.choice.is_empty() {
        println!("You chose: {}", final_model.choice);
    } else {
        println!("No selection made");
    }
    
    Ok(())
}
```

### Rust-Specific Patterns

**Array Wrapping Navigation:**
```rust
// Safe wrapping without overflow/underflow
if self.cursor == 0 {
    self.cursor = CHOICES.len() - 1;  // Bottom wrap
} else {
    self.cursor -= 1;  // Normal decrement
}
```

**Index Bounds Safety:**
```rust
if self.cursor >= CHOICES.len() - 1 {
    self.cursor = 0;  // Top wrap  
} else {
    self.cursor += 1;  // Normal increment
}
```

**Static Choice Arrays:**
```rust
const CHOICES: &[&str] = &["Taro", "Coffee", "Lychee"];
//     ^--compile-time constant, efficient memory usage
```

**Key Binding Matching:**
```rust
if matches_binding(&self.keymap.enter, key_msg) {
    // Handle selection
}
```

More explicit than pattern matching on raw keys.

### Visual Rendering

The view shows a cursor-highlighted menu:

```rust
fn view(&self) -> String {
    let mut s = "What kind of Bubble Tea would you like to order?\n\n".to_string();
    
    for (i, choice) in CHOICES.iter().enumerate() {
        if self.cursor == i {
            s.push_str(&format!("> {}\n", choice)); // Highlight current
        } else {
            s.push_str(&format("  {}\n", choice)); // Normal item
        }
    }
    
    s.push_str("\nPress q to quit.\n");
    s
}
```

**Example Output:**
```
What kind of Bubble Tea would you like to order?

  Taro
> Coffee      <- cursor position
  Lychee

Press q to quit.
```

### Use Cases & Patterns

**Shell Integration:**
```bash
#!/bin/bash
CHOICE=$(cargo run --example result)
echo "User selected: $CHOICE"
```

**Configuration Selection:**
```rust
// Select configuration file
let config_file = run_config_selector().await?;
let config = load_config(&config_file)?;
```

**Multi-step Wizards:**
```rust
// Step 1: Choose database type
let db_type = run_database_selector().await?;

// Step 2: Configure connection  
let connection = run_connection_wizard(&db_type).await?;
```

**Error Handling:**
```rust
match program.run().await {
    Ok(model) => {
        if model.choice.is_empty() {
            println!("No selection made");
            std::process::exit(1);
        } else {
            println!("Selected: {}", model.choice);
        }
    }
    Err(e) => {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

### Testing Result Patterns

```rust
#[tokio::test]
async fn test_user_selection() {
    let mut model = AppModel::new();
    
    // Simulate user navigation  
    model.cursor = 1;
    
    // Simulate enter key
    let result = model.update(Box::new(KeyMsg::new(KeyCode::Enter)));
    
    // Verify selection was captured
    assert_eq!(model.choice, "Coffee");
    assert!(result.is_some()); // Should return quit command
}
```

### Program Exit States

**Successful Selection:**
- `choice` field contains selected value
- Program returns `Ok(model)`
- Calling code can use `model.choice`

**Early Exit (no selection):**
- `choice` field remains empty
- Program returns `Ok(model)`  
- Calling code sees empty choice

**Error Exit:**
- Program returns `Err(...)`
- Calling code handles error appropriately

## Related Examples

- **[list-simple](../list-simple/)** - More complex list navigation
- **[file-picker](../file-picker/)** - File selection with results
- **[debounce](../debounce/)** - Another example with exit conditions

## Files

- `main.rs` — Complete menu selection with result capture
- `Cargo.toml` — Dependencies including bubbletea-widgets
- `result.gif` — Demo showing navigation and selection
- `README.md` — This documentation

## Real-world Applications

- Configuration wizards that return settings
- File/option selectors for shell scripts  
- Interactive command-line tools
- Multi-step installation processes
- User preference collection utilities