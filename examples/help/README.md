# Help

<img width="1200" src="./help.gif" />

A comprehensive demonstration of the help system and key binding management, showing how to create toggleable help displays, organize keyboard shortcuts, and provide user guidance in terminal applications.

## Features

- **Toggleable Help**: Switch between mini and full help displays
- **Organized Key Bindings**: Structured keyboard shortcuts with descriptions
- **Visual Navigation**: Arrow key movement with position display
- **Responsive Layout**: Help text adapts to terminal width
- **Professional Styling**: Color-coded help text and status display
- **Widget Integration**: Uses bubbletea-widgets help system

## Running the Example

From the repository root:

```bash
cargo run --example help
```

**Controls:**
- `↑`/`k` - Move up (with visual feedback)
- `↓`/`j` - Move down (with visual feedback)  
- `←`/`h` - Move left (with visual feedback)
- `→`/`l` - Move right (with visual feedback)
- `?` - Toggle help display (mini ↔ full)
- `q` / `Ctrl+C` - Quit

## What this demonstrates

### Key Concepts for Beginners

**Help System Design**: This example shows how to build user-friendly applications with:
1. Discoverable keyboard shortcuts
2. Context-sensitive help displays
3. Progressive help disclosure (mini → full)
4. Consistent key binding organization
5. Professional user experience patterns

**Widget-Based Architecture**: Demonstrates using pre-built help widgets rather than implementing help systems from scratch.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
```

**Help Widget System:**
```rust
use bubbletea_widgets::help::{KeyMap as HelpKeyMap, Model as HelpModel};
use bubbletea_widgets::key::{matches_binding, new_binding, with_help, with_keys_str, Binding, KeyMap};
```

- `HelpModel`: Pre-built help display widget
- `KeyMap` trait: Interface for help integration
- `matches_binding()`: Clean key matching
- `with_help()`: Attach descriptions to key bindings

### Architecture Walkthrough

#### Key Binding Organization

```rust
pub struct Keymap {
    up: Binding,
    down: Binding,
    left: Binding,
    right: Binding,
    help: Binding,
    quit: Binding,
}
```

Each binding includes both keys and help text:

```rust
up: new_binding(vec![
    with_keys_str(&["up", "k"]),      // Multiple key options
    with_help("↑/k", "move up"),      // Help display text
]),
```

#### Model Structure

```rust
pub struct Model {
    position: Position,        // Current cursor position
    keymap: Keymap,           // All key bindings
    help: HelpModel,          // Help display widget
    last_key: String,         // Last pressed key (for feedback)
}

struct Position {
    x: i32,
    y: i32,
}
```

#### Help Widget Integration

The help system requires implementing the `HelpKeyMap` trait:

```rust
impl HelpKeyMap for Keymap {
    type ShortHelp = Vec<Binding>;
    type FullHelp = Vec<Vec<Binding>>;

    fn short_help(&self) -> Self::ShortHelp {
        // Mini help - most important keys only
        vec![self.help.clone(), self.quit.clone()]
    }

    fn full_help(&self) -> Self::FullHelp {
        // Full help - organized into groups
        vec![
            vec![self.up.clone(), self.down.clone(), self.left.clone(), self.right.clone()],
            vec![self.help.clone(), self.quit.clone()],
        ]
    }
}
```

#### Navigation Logic

```rust
if matches_binding(&self.keymap.up, key_msg) {
    self.position.y -= 1;
    self.last_key = "↑".to_string();
}

if matches_binding(&self.keymap.down, key_msg) {
    self.position.y += 1;  
    self.last_key = "↓".to_string();
}
```

### Rust-Specific Patterns

**Key Binding Matching:**
```rust
// Clean, readable key checking
if matches_binding(&self.keymap.help, key_msg) {
    self.help.show_all = !self.help.show_all;  // Toggle help mode
}
```

**Multiple Key Aliases:**
```rust
with_keys_str(&["up", "k"]),        // Arrow key OR vi-style
with_keys_str(&["left", "h"]),      // Support both conventions
```

**Help Text Formatting:**
```rust
with_help("↑/k", "move up"),        // Unicode arrows + key letters
with_help("?", "toggle help"),      // Clear descriptions
```

**Trait Implementation:**
```rust
impl HelpKeyMap for Keymap {
    // Type aliases for help content structure
    type ShortHelp = Vec<Binding>;
    type FullHelp = Vec<Vec<Binding>>;  // Grouped bindings
}
```

### Help Display Modes

**Mini Help (show_all = false):**
```
? toggle help • q quit
```

**Full Help (show_all = true):**
```
Movement
  ↑/k move up    ↓/j move down    ←/h move left    →/l move right

General
  ? toggle help    q quit
```

### Visual Feedback System

The example provides real-time feedback:

```rust
fn view(&self) -> String {
    let status_style = Style::new().foreground(Color::from("241"));
    
    let mut s = format!(
        "Use {} to move around. Current position: {}, {}.",
        status_style.render("arrow keys or wasd"),
        self.position.x,
        self.position.y
    );
    
    if !self.last_key.is_empty() {
        s.push_str(&format!(" You pressed: {}", self.last_key));
    }
    
    s.push_str(&format!("\n\n{}", self.help.view()));
    s
}
```

### Help Widget Configuration

```rust
let help = HelpModel::new(&[
    help::with_keymap(keymap.clone()),     // Provide key bindings
    help::with_width(terminal_width),       // Responsive width
    help::with_show_all(false),            // Start in mini mode
]);
```

### Window Size Responsiveness

```rust
if let Some(window_msg) = msg.downcast_ref::<WindowSizeMsg>() {
    // Update help widget width for proper text wrapping
    self.help.set_width(window_msg.width as usize);
}
```

### Professional UX Patterns

**Progressive Disclosure:**
- Start with minimal help to avoid overwhelming users
- Provide easy toggle to full help when needed
- Group related commands together

**Visual Hierarchy:**
- Important keys (help, quit) always visible
- Related actions grouped visually
- Clear action descriptions

**Feedback Loop:**
- Show current state (position)
- Confirm user actions (last key pressed)
- Responsive help text

### Real-world Applications

**Text Editors:**
```rust
// Comprehensive help for editing commands
let editor_keys = EditorKeymap::new();
let help = create_editor_help(editor_keys);
```

**File Managers:**
```rust
// Context-sensitive help for file operations
match current_mode {
    FileMode => show_file_help(),
    SearchMode => show_search_help(),
    PreviewMode => show_preview_help(),
}
```

**System Tools:**
```rust
// Multi-mode help for different tool functions
help.set_context(match tool_mode {
    Monitor => monitoring_keys(),
    Configure => config_keys(), 
    Debug => debug_keys(),
});
```

### Testing Help Systems

```rust
#[test]
fn test_help_modes() {
    let mut model = Model::new();
    
    // Test mini help
    assert!(!model.help.show_all);
    
    // Test help toggle
    model.toggle_help();
    assert!(model.help.show_all);
    
    // Test help content
    let help_text = model.help.view();
    assert!(help_text.contains("Movement"));
}
```

### Best Practices

**Key Binding Design:**
- Support multiple conventions (arrows + vi keys)
- Use intuitive mnemonics where possible
- Group related functions together
- Always provide quit option

**Help Text Writing:**
- Keep descriptions concise but clear
- Use consistent formatting
- Include visual symbols (arrows, etc.)
- Group by functionality, not alphabetically

**Widget Integration:**
- Let help widgets handle layout and formatting
- Provide structured key binding data
- Update help on window resize
- Consider context-sensitive help

## Related Examples

- **[result](../result/)** - Another key binding example
- **[file-picker](../file-picker/)** - Widget-based key handling
- **[glamour](../glamour/)** - Styled content display

## Files

- `main.rs` — Complete help system with key binding management
- `Cargo.toml` — Dependencies including bubbletea-widgets
- `help.gif` — Demo showing help toggle and navigation
- `README.md` — This documentation

## Implementation Tips

- Always implement both mini and full help modes
- Group related commands together in full help
- Use consistent help text formatting
- Test help display at various terminal widths
- Consider making help context-sensitive for complex applications