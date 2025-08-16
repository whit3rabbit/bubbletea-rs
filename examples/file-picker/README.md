# File Picker

An interactive file browser example using `bubbletea-widgets::filepicker`, demonstrating directory navigation, file filtering, and user selection with a polished interface.

## Features

- **Interactive File Navigation**: Browse directories using arrow keys
- **File Type Filtering**: Only shows specified file extensions (.go, .rs, .txt, .md, etc.)
- **Visual Feedback**: Shows permissions, file sizes, and styling
- **Error Handling**: Displays helpful messages for invalid selections
- **Keyboard Controls**: Full keyboard navigation with multiple quit options
- **Home Directory Start**: Automatically starts in user's home directory

## Running the Example

From the repository root:

```bash
cargo run --example file-picker
```

**Controls:**
- `↑↓` - Navigate files and directories
- `Enter` - Select file or enter directory  
- `Backspace` - Go up one directory level
- `q` / `Ctrl+C` / `Esc` - Quit

## What this demonstrates

### Key Concepts for Beginners

**File System Navigation**: This example shows how to build a file browser TUI with:
1. Directory traversal and listing
2. File filtering by extension
3. Permission and size display
4. Error handling for invalid selections

**Widget Integration**: Demonstrates using pre-built widgets instead of building UI from scratch.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
```

- Standard MVU pattern implementation
- `tick()` for delayed error message clearing  
- `quit()` for program termination

**File Picker Widget:**
```rust
use bubbletea_widgets::filepicker;
```

- `filepicker::new()`: Creates file picker widget
- `filepicker::Model`: Pre-built file navigation component
- Built-in directory reading and filtering

**Key Binding System:**
```rust
use bubbletea_widgets::key::{new_binding, with_help, with_keys_str, Binding};
```

### Architecture Walkthrough

#### Model Structure
```rust
pub struct FilePickerModel {
    filepicker: filepicker::Model,  // The core file picker widget
    selected_file: String,          // Currently selected file path
    quitting: bool,                 // Exit state tracking  
    error: Option<String>,          // Error message display
    keys: KeyBindings,              // Organized keyboard shortcuts
}
```

#### File Picker Configuration

The widget is configured in `init()` to match specific requirements:

```rust
// File selection rules
fp_model.file_allowed = true;        // Can select files
fp_model.dir_allowed = false;        // Cannot select directories (only navigate)
fp_model.allowed_types = vec![       // Allowed file extensions
    ".mod", ".sum", ".go", ".txt", ".md", ".rs", ".toml"
];

// Display options  
fp_model.show_hidden = false;        // Hide dotfiles
fp_model.show_permissions = true;    // Show file permissions
fp_model.show_size = true;           // Show file sizes
fp_model.set_height(15);             // Visible entries count
```

#### File Selection Logic

The example demonstrates different types of user interactions:

```rust
// Valid file selection - quit immediately  
if let (true, path) = self.filepicker.did_select_file(&msg) {
    self.selected_file = path;
    return Some(quit());
}

// Invalid file selection - show error
if let (true, path) = self.filepicker.did_select_disabled_file(&msg) {
    self.error = Some(format!("{} is not valid.", path));
    return Some(clear_error_after(Duration::from_secs(2)));
}
```

### Rust-Specific Patterns

**Widget Initialization:**
```rust
let (mut fp_model, init_cmd) = filepicker::Model::init();
```

Widgets follow the same MVU pattern as the main application, returning initial commands.

**Home Directory Detection:**
```rust
if let Ok(home_dir) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
    fp_model.current_directory = std::path::PathBuf::from(home_dir);
    fp_model.read_dir();  // Refresh directory listing
}
```

Cross-platform home directory detection (Unix vs Windows).

**Error Message Auto-Clear:**
```rust
fn clear_error_after(duration: Duration) -> Cmd {
    tick(duration, |_| Box::new(ClearErrorMsg) as Msg)
}
```

Demonstrates using timer commands for automatic UI updates.

**Message Forwarding:**
```rust
// Let the widget handle its own messages
return self.filepicker.update(msg);
```

Widgets process their own message types internally.

### Widget Communication Patterns

**Checking Widget State:**
```rust
// Non-destructive state checking
if let (true, path) = self.filepicker.did_select_file(&msg) {
    // Widget detected file selection
}
```

**Command Batching:**
```rust
Some(bubbletea_rs::batch(vec![
    fp_cmd,                                    // Widget's command
    clear_error_after(Duration::from_secs(2)) // Our command
]))
```

Combine widget commands with application commands.

**Style Integration:**
```rust
let error_style = self.filepicker.styles.disabled_file.clone();
output.push_str(&error_style.render(error));
```

Reuse widget styling for consistent appearance.

### Program Configuration

```rust
let program = Program::<FilePickerModel>::builder()
    .alt_screen(true)      // Full-screen mode for better file browsing
    .signal_handler(true)  // Handle Ctrl+C properly
    .build()?;

let final_model = program.run().await?;

// Access final state after program exits
if !final_model.selected_file.is_empty() {
    println!("You selected: {}", final_model.selected_file);
}
```

### Common Pitfalls & Solutions

**Directory vs File Selection:**
- Set `dir_allowed = false` to prevent directory selection
- Use `file_allowed = true` to enable file selection
- The widget handles navigation vs selection automatically

**Window Sizing:**
```rust
let window_size_cmd = bubbletea_rs::window_size();
```

Send window size messages to ensure proper widget layout.

**File Filtering:**
- Use `allowed_types` for extension filtering
- Widget automatically grays out non-selectable files
- `did_select_disabled_file()` catches invalid selections

## Related Examples

- **[list-default](../list-default/)** - Basic list navigation patterns
- **[textinput](../textinput/)** - Another widget integration example
- **[help](../help/)** - Key binding and help system patterns

## Files

- `main.rs` — Complete file picker implementation using widgets
- `Cargo.toml` — Dependencies including bubbletea-widgets
- `README.md` — This documentation

## Extension Ideas

- Add file preview for text files
- Implement bookmark/favorites system  
- Add search/filter functionality
- Support for multiple file selection
- Integration with external programs (edit, view, etc.)