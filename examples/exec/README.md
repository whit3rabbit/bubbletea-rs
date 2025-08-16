# Exec

<img width="1200" src="./exec.gif" />

An example demonstrating how to execute external programs (like text editors) from within a bubbletea-rs application while properly handling terminal control.

## Features

- **External Editor Integration**: Opens `$EDITOR` (defaults to vim) from within the TUI
- **Alt-screen Toggle**: Switch between inline and fullscreen modes
- **Process Management**: Proper handling of external process execution and terminal control
- **Key Bindings**: Structured key binding system with help text
- **Error Handling**: Graceful handling of editor execution failures

## Running the Example

From the repository root:

```bash
cargo run --example exec
```

Set your preferred editor:
```bash
EDITOR=nano cargo run --example exec
```

## What this demonstrates

### Key Concepts for Beginners

**External Process Execution**: Sometimes TUI applications need to temporarily hand control to external programs (editors, pagers, etc.). This example shows how to:
1. Suspend the TUI
2. Launch an external program with terminal access
3. Resume the TUI when the program exits

**Alt-screen Management**: Demonstrates switching between normal terminal flow and alternate screen buffer.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{enter_alt_screen, exit_alt_screen, quit, Cmd, KeyMsg, Model, Msg, Program};
```

- `enter_alt_screen()`: Switch to alternate screen buffer
- `exit_alt_screen()`: Return to normal terminal
- Standard MVU pattern with `Model`, `Msg`, `Cmd`

**Widget System:**
```rust
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};
```

- `Binding`: Structured key binding with help text
- `with_keys_str()`: Define which keys trigger the binding
- `with_help()`: Provide help text for user interface

### Architecture Walkthrough

#### Model Structure
```rust
struct ExecModel {
    altscreen_active: bool,  // Track current screen mode
    err: Option<String>,     // Store any execution errors
    keys: KeyBindings,       // Organized key bindings
}
```

#### Key Binding Organization
```rust
pub struct KeyBindings {
    pub toggle_altscreen: Binding,
    pub open_editor: Binding,
    pub quit: Binding,
}
```

This pattern centralizes all keyboard shortcuts and their help text, making them easy to modify and display.

#### External Command Execution

The core of this example is the `open_editor()` command:

```rust
fn open_editor() -> Cmd {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    
    Box::pin(async move {
        let mut cmd = Command::new(&editor);
        cmd.stdin(Stdio::inherit())      // Give editor access to terminal input
            .stdout(Stdio::inherit())    // Give editor access to terminal output  
            .stderr(Stdio::inherit());   // Give editor access to error output
        
        match cmd.status() {
            Ok(status) if status.success() => {
                Some(Box::new(EditorFinishedMsg { err: None }) as Msg)
            }
            Ok(_) => Some(Box::new(EditorFinishedMsg {
                err: Some("Editor exited with non-zero status".to_string()),
            }) as Msg),
            Err(e) => Some(Box::new(EditorFinishedMsg {
                err: Some(format!("Failed to execute editor: {}", e)),
            }) as Msg),
        }
    })
}
```

### Rust-Specific Patterns

**Environment Variable Handling:**
```rust
let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
```

Safely reads environment variables with fallback defaults.

**Process Management:**
```rust
use std::process::{Command, Stdio};

let mut cmd = Command::new(&editor);
cmd.stdin(Stdio::inherit())    // Inherit parent's stdin/stdout/stderr
   .stdout(Stdio::inherit())   // This gives the editor full terminal control
   .stderr(Stdio::inherit());
```

**Custom Message Types:**
```rust
#[derive(Debug)]
struct EditorFinishedMsg {
    err: Option<String>,  // Optional error message
}
```

**Key Binding Matching:**
```rust
if self.keys.open_editor.matches(key_msg) {
    return Some(open_editor());
}
```

The widget system provides clean pattern matching for complex key combinations.

### Critical Implementation Details

**Why inherit stdio streams?**
- External editors need direct terminal access
- `Stdio::inherit()` gives the editor full control of input/output
- The TUI framework automatically handles suspending/resuming

**Alt-screen vs Inline Mode:**
- **Alt-screen**: Program runs in separate screen buffer (like vim, htop)
- **Inline**: Program output appears in normal terminal flow
- Toggle shows different terminal integration approaches

**Error Propagation:**
```rust
if let Some(err) = &editor_msg.err {
    self.err = Some(err.clone());
    return Some(quit());  // Exit on editor errors
}
```

### Message Flow

1. **User presses 'e'** → `KeyMsg` received
2. **Key binding matches** → `open_editor()` command scheduled  
3. **Editor launches** → Process runs with inherited terminal
4. **Editor exits** → `EditorFinishedMsg` sent with result
5. **Result handled** → Either continue or quit with error

### Program Configuration

```rust
let program = Program::<ExecModel>::builder()
    .signal_handler(true)    // Handle Ctrl+C properly
    .build()?;
```

The signal handler is important when external processes are involved.

## Related Examples

- **[altscreen-toggle](../altscreen-toggle/)** - More alt-screen examples
- **[prevent-quit](../prevent-quit/)** - Advanced quit handling patterns
- **[suspend](../suspend/)** - Process suspension and resumption

## Files

- `main.rs` — Complete external process integration
- `Cargo.toml` — Dependencies and build configuration
- `exec.gif` — Demo recording  
- `README.md` — This documentation

## Usage Tips

- Try different editors: `EDITOR=nano`, `EDITOR=emacs`, `EDITOR=code`
- Test both alt-screen and inline modes to see the difference
- The example creates a temporary file `pico.save` that you can edit