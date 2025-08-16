# Alt Screen Toggle

<img width="1200" src="./altscreen-toggle.gif" />

## Overview

This example demonstrates how to toggle between terminal screen buffers in a TUI application. It showcases the difference between "inline mode" (normal terminal output) and "alternate screen mode" (fullscreen TUI mode) that many terminal applications use.

## Key Features

- **Dual Screen Modes**: Toggle between inline and alternate screen buffers
- **Suspend/Resume Support**: Handle Ctrl+Z signal to suspend the program
- **Key Binding System**: Structured key handling using `bubbletea_widgets::key`
- **Immediate Rendering**: Shows content immediately after startup
- **Styled Output**: Uses lipgloss-extras for consistent terminal styling

## How It Works

The example starts in inline mode, where content is mixed with your regular terminal output. When you press space, it switches to alternate screen mode - a separate screen buffer that takes over the entire terminal. This is the same mechanism used by applications like `vim`, `less`, or `htop`.

The key difference:
- **Inline mode**: Output appears in your normal terminal session and persists when the program exits
- **Alternate screen mode**: Takes over the entire terminal and restores your previous content when exiting

## Code Structure

### Model (`AltScreenModel`)
- `altscreen: bool` - Tracks current screen mode
- `quitting: bool` - Manages graceful exit state
- `suspending: bool` - Handles suspend/resume operations
- `keys: KeyBindings` - Structured key binding definitions

### Key Components

1. **Key Bindings System**
   ```rust
   use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};
   
   let quit = new_binding(vec![
       with_keys_str(&["q", "esc"]),
       with_help("q/esc", "quit"),
   ]);
   ```

2. **Screen Toggle Commands**
   ```rust
   let cmd = if self.altscreen {
       exit_alt_screen()  // Switch to inline mode
   } else {
       enter_alt_screen() // Switch to alternate screen
   };
   ```

3. **Immediate Rendering Pattern**
   ```rust
   struct InitRenderMsg;
   
   fn init_render_cmd() -> Cmd {
       Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
   }
   ```

## API Usage

### Screen Buffer Control
- `enter_alt_screen()` - Switch to alternate screen buffer
- `exit_alt_screen()` - Return to inline screen buffer

### Program Configuration
```rust
let program = Program::<AltScreenModel>::builder()
    .signal_handler(true)     // Enable Ctrl+Z suspend/resume
    .alt_screen(false)        // Start in inline mode
    .build()?;
```

### Suspend/Resume Support
- `suspend()` - Command to suspend the program
- `ResumeMsg` - Message received when program resumes

## Running the Example

```bash
cd examples/altscreen-toggle
cargo run
```

## Key Bindings

- **Space**: Toggle between inline and alternate screen modes
- **Ctrl+Z**: Suspend the program (try it to see the difference!)
- **q** or **Esc**: Quit the application

## Implementation Notes

### InitRenderMsg Pattern
The example uses a synthetic `InitRenderMsg` to trigger an immediate render after startup. This ensures users see content right away rather than waiting for the first user interaction.

### Inline Demo
Before starting the TUI, the program prints `$ ./altscreen-toggle` to the terminal. This demonstrates inline output - when you toggle back to inline mode, you'll see this text is still there.

### Suspend/Resume Behavior
When you press Ctrl+Z:
1. The program suspends and returns control to your shell
2. Type `fg` to resume the program
3. The `ResumeMsg` clears the suspending state and continues normal operation

### Screen Buffer Persistence
- In **inline mode**: Content persists in your terminal history
- In **alternate screen mode**: Content disappears when the program exits, restoring your previous terminal state

This example mirrors the Go Bubble Tea `altscreen-toggle` example and demonstrates fundamental terminal control concepts that are essential for building professional TUI applications.
