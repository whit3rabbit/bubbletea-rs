# Set Window Title

A demonstration of terminal window title management, showing how to programmatically set the terminal window title from within terminal applications for better user experience and application identification.

## Features

- **Dynamic Window Titles**: Set custom titles for terminal windows
- **Terminal Integration**: Works with most modern terminal emulators
- **Professional Appearance**: Styled interface with title confirmation
- **Simple Interface**: Clear example of title setting functionality
- **Graceful Degradation**: Handles terminals that don't support title changes

## Running the Example

From the repository root:

```bash
cargo run --example set-window-title
```

**What happens:**
1. Terminal window title changes to "Bubble Tea Example"
2. Application displays confirmation of title setting
3. Press any key to quit

**Note:** For best results, run directly in terminal, not through tmux/screen which may intercept title changes.

## What this demonstrates

### Key Concepts for Beginners

**Window Title Control**: This example shows how to:
1. Set terminal window titles programmatically
2. Provide visual feedback to users about application state
3. Create professional-looking terminal applications
4. Handle terminal compatibility issues gracefully

**User Experience Enhancement**: Demonstrates improving application identification and user orientation through title management.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, set_window_title, Cmd, KeyMsg, Model, Msg, Program};
```

- `set_window_title(String)`: Command to set terminal window title
- Standard MVU pattern for state management
- Simple command execution pattern

**Styling System:**
```rust
use lipgloss_extras::lipgloss::{Color, Style};
```

- Professional styling for UI feedback
- Color-coded status messages

### Architecture Walkthrough

#### Model Structure
```rust
struct SetWindowTitleModel {
    title_set: bool,  // Track whether title was set successfully
}
```

#### Title Setting Pattern
```rust
fn init() -> (Self, Option<Cmd>) {
    let model = SetWindowTitleModel { title_set: false };
    
    // Issue command to set window title
    let set_title_cmd = set_window_title("Bubble Tea Example".to_string());
    
    (model, Some(set_title_cmd))
}
```

The title is set immediately on application startup.

#### User Feedback
```rust
fn view(&self) -> String {
    let mut content = String::new();
    
    // App title with styling
    let title_style = Style::new()
        .foreground(Color::from("5"))  // Magenta
        .bold(true);
    content.push_str(&title_style.render("Set Window Title Example"));
    
    // Instructions
    content.push_str("This program sets the terminal window title to ");
    content.push_str("\"Bubble Tea Example\".\n\n");
    
    if self.title_set {
        content.push_str("✓ Title has been set!\n");
    } else {
        content.push_str("Setting window title...\n");
    }
    
    content.push_str("\nPress any key to quit.");
    content
}
```

### Rust-Specific Patterns

**String Ownership:**
```rust
set_window_title("Bubble Tea Example".to_string())
//               ^-- owned string for command lifetime
```

**Simple State Tracking:**
```rust
struct SetWindowTitleModel {
    title_set: bool,  // Boolean flag for confirmation
}
```

**Any Key Quit Pattern:**
```rust
if let Some(_key_msg) = msg.downcast_ref::<KeyMsg>() {
    return Some(quit());  // Any key triggers quit
}
```

### Terminal Compatibility

**✅ Full Support:**
- iTerm2, Alacritty, GNOME Terminal
- Windows Terminal, Terminal.app (macOS)
- Most modern terminal emulators

**⚠️ Limited Support:**
- tmux/screen sessions (may intercept titles)
- Some SSH configurations
- Older terminal emulators

**❌ No Support:**
- Pure TTY sessions
- Some minimal terminals
- Certain embedded systems

### Escape Sequence Details

The `set_window_title()` function sends ANSI escape sequences:

```
ESC ] 0 ; title BEL
\033]0;Bubble Tea Example\007
```

Where:
- `\033]0;` - OSC (Operating System Command) for title
- `title` - The actual title text
- `\007` - BEL (Bell) character to terminate

### Real-world Applications

**Application Identification:**
```rust
// Different titles for different modes
match app_mode {
    Mode::Editor => set_window_title("MyApp - Editor".to_string()),
    Mode::Viewer => set_window_title("MyApp - Viewer".to_string()),
    Mode::Settings => set_window_title("MyApp - Settings".to_string()),
}
```

**Progress Indication:**
```rust
// Show progress in window title
fn update_progress_title(current: usize, total: usize) -> Cmd {
    let title = format!("MyApp - Processing {}/{}", current, total);
    set_window_title(title)
}
```

**File Context:**
```rust
// Show current file in title
fn set_file_title(filename: &str) -> Cmd {
    let title = if filename.is_empty() {
        "Text Editor - Untitled".to_string()
    } else {
        format!("Text Editor - {}", filename)
    };
    set_window_title(title)
}
```

**Status Indication:**
```rust
// Show connection status
fn update_status_title(connected: bool) -> Cmd {
    let status = if connected { "Connected" } else { "Disconnected" };
    set_window_title(format!("Chat App - {}", status))
}
```

### Dynamic Title Updates

```rust
struct DynamicTitleApp {
    counter: u32,
}

impl Model for DynamicTitleApp {
    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(_tick_msg) = msg.downcast_ref::<TickMsg>() {
            self.counter += 1;
            
            // Update title with counter
            let title = format!("Counter: {}", self.counter);
            return Some(set_window_title(title));
        }
        None
    }
}
```

### Error Handling

```rust
// Graceful handling when title setting fails
fn attempt_set_title(title: String) -> Cmd {
    Box::pin(async move {
        // Try to set title
        match set_terminal_title(&title).await {
            Ok(_) => Some(Box::new(TitleSetMsg(true)) as Msg),
            Err(_) => {
                // Fall back to alternative identification
                Some(Box::new(TitleSetMsg(false)) as Msg)
            }
        }
    })
}
```

### Best Practices

**Title Content:**
- Keep titles concise but descriptive
- Include application name for identification
- Show relevant context (file, mode, status)
- Avoid special characters that might break terminals

**Update Frequency:**
- Don't update titles too frequently (causes flicker)
- Update on significant state changes
- Consider debouncing rapid updates

**Compatibility:**
- Provide fallback identification methods
- Test across different terminals
- Handle terminals that ignore title changes

**User Experience:**
- Use titles to help users identify windows
- Show progress for long operations
- Restore original title on exit (optional)

### Testing Title Functionality

```bash
# Test in different terminals
alacritty -e cargo run --example set-window-title
gnome-terminal -- cargo run --example set-window-title
iterm2 -e cargo run --example set-window-title

# Test in tmux (may not work)
tmux new-session -d 'cargo run --example set-window-title'

# Test via SSH (may have limitations)
ssh user@host 'cd /path/to/project && cargo run --example set-window-title'
```

### Title Restoration

```rust
struct TitleManager {
    original_title: String,
}

impl TitleManager {
    fn new() -> Self {
        // Save current title if possible
        let original = get_current_title().unwrap_or_default();
        Self { original_title: original }
    }
    
    fn restore_title(&self) -> Cmd {
        if !self.original_title.is_empty() {
            set_window_title(self.original_title.clone())
        } else {
            // Use default or do nothing
            Box::pin(async { None })
        }
    }
}
```

### Integration with Other Examples

**With File Picker:**
```rust
// Show selected file in title
if let Some(filename) = selected_file {
    return Some(set_window_title(format!("File: {}", filename)));
}
```

**With Progress Bars:**
```rust
// Show completion percentage in title
let title = format!("Processing... {}%", (progress * 100.0) as u32);
return Some(set_window_title(title));
```

**With Text Editors:**
```rust
// Show edit status in title
let title = if modified {
    format!("*{} - Editor", filename)  // * indicates unsaved changes
} else {
    format!("{} - Editor", filename)
};
```

## Related Examples

- **[window-size](../window-size/)** - Another terminal integration feature
- **[focus-blur](../focus-blur/)** - Terminal state awareness
- **[fullscreen](../fullscreen/)** - Terminal mode management

## Files

- `main.rs` — Complete window title setting implementation
- `Cargo.toml` — Dependencies and build configuration
- `README.md` — This documentation

## Implementation Tips

- Set titles early in application lifecycle
- Test across multiple terminal emulators
- Handle terminals that don't support title changes gracefully
- Use titles to enhance user experience, not as primary functionality
- Consider title length limits on some terminals
- Provide visual feedback when titles are set successfully