# Textarea

A demonstration of multi-line text input functionality, showing cursor navigation, line editing, and text area management patterns for building text editors and input forms in terminal applications.

## Features

- **Multi-line Text Input**: Full text editing across multiple lines
- **Cursor Navigation**: Arrow keys for cursor movement within text
- **Line Management**: Enter key creates new lines, backspace handles line deletion
- **Placeholder Text**: Shows guidance when textarea is empty
- **Focus State**: Visual indication of textarea focus state
- **Text Selection**: Basic text editing operations
- **Responsive Width**: Configurable textarea dimensions

## Running the Example

From the repository root:

```bash
cargo run --example textarea
```

**Controls:**
- `←→↑↓` - Move cursor within text
- `Enter` - Create new line
- `Backspace` - Delete character or merge lines
- `Ctrl+C` - Quit
- Type normally to add text

## What this demonstrates

### Key Concepts for Beginners

**Multi-line Text Editing**: This example shows fundamental patterns for:
1. Managing text across multiple lines
2. Cursor positioning within 2D text space
3. Line-based editing operations (insert, delete, merge)
4. Handling word wrapping and scrolling
5. Building text input components from scratch

**Text Editor Architecture**: Demonstrates the core data structures and algorithms needed for text editing interfaces.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};
```

- Standard MVU pattern for text state management
- `KeyCode` enum for detailed key handling
- Character and navigation key processing

### Architecture Walkthrough

#### Model Structure
```rust
pub struct TextAreaModel {
    pub content: Vec<String>,    // Each line as separate string
    pub cursor_line: usize,      // Current line (0-based)
    pub cursor_col: usize,       // Current column (0-based)
    pub placeholder: String,     // Help text when empty
    pub focused: bool,           // Focus state
    pub show_cursor: bool,       // For cursor blinking
    pub height: usize,           // Visible lines
    pub width: usize,            // Character width
}
```

#### Text Line Management

```rust
// Insert character at cursor position
pub fn insert_char(&mut self, ch: char) {
    if self.cursor_line >= self.content.len() {
        self.content.push(String::new());
    }
    
    let line = &mut self.content[self.cursor_line];
    line.insert(self.cursor_col, ch);
    self.cursor_col += 1;
}

// Create new line at cursor
pub fn insert_newline(&mut self) {
    let current_line = &self.content[self.cursor_line];
    let (before, after) = current_line.split_at(self.cursor_col);
    
    self.content[self.cursor_line] = before.to_string();
    self.content.insert(self.cursor_line + 1, after.to_string());
    
    self.cursor_line += 1;
    self.cursor_col = 0;
}
```

#### Cursor Navigation

```rust
// Move cursor left with line wrapping
pub fn cursor_left(&mut self) {
    if self.cursor_col > 0 {
        self.cursor_col -= 1;
    } else if self.cursor_line > 0 {
        // Move to end of previous line
        self.cursor_line -= 1;
        self.cursor_col = self.content[self.cursor_line].len();
    }
}

// Move cursor right with line wrapping
pub fn cursor_right(&mut self) {
    let current_line_len = self.content[self.cursor_line].len();
    if self.cursor_col < current_line_len {
        self.cursor_col += 1;
    } else if self.cursor_line < self.content.len() - 1 {
        // Move to beginning of next line
        self.cursor_line += 1;
        self.cursor_col = 0;
    }
}
```

#### Backspace and Deletion

```rust
pub fn backspace(&mut self) {
    if self.cursor_col > 0 {
        // Delete character within line
        self.content[self.cursor_line].remove(self.cursor_col - 1);
        self.cursor_col -= 1;
    } else if self.cursor_line > 0 {
        // Merge with previous line
        let current_line = self.content.remove(self.cursor_line);
        self.cursor_line -= 1;
        self.cursor_col = self.content[self.cursor_line].len();
        self.content[self.cursor_line].push_str(&current_line);
    }
}
```

### Rust-Specific Patterns

**Vec<String> for Lines:**
```rust
pub content: Vec<String>,  // Each line is independent string
```

More efficient than single string with embedded newlines for editing operations.

**Safe Array Access:**
```rust
if self.cursor_line >= self.content.len() {
    self.content.push(String::new());  // Extend if needed
}

let line = &mut self.content[self.cursor_line];  // Safe after check
```

**String Splitting:**
```rust
let (before, after) = current_line.split_at(self.cursor_col);
self.content[self.cursor_line] = before.to_string();
self.content.insert(self.cursor_line + 1, after.to_string());
```

Efficient line splitting for Enter key handling.

**Character Insertion:**
```rust
line.insert(self.cursor_col, ch);  // Insert at specific position
self.cursor_col += 1;              // Advance cursor
```

**Bounds Checking:**
```rust
pub fn cursor_up(&mut self) {
    if self.cursor_line > 0 {
        self.cursor_line -= 1;
        // Clamp cursor to line length
        let line_len = self.content[self.cursor_line].len();
        self.cursor_col = self.cursor_col.min(line_len);
    }
}
```

### Key Input Processing

```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
        match key_msg.key {
            KeyCode::Char(ch) => {
                self.insert_char(ch);
            }
            KeyCode::Enter => {
                self.insert_newline();
            }
            KeyCode::Backspace => {
                self.backspace();
            }
            KeyCode::Left => {
                self.cursor_left();
            }
            KeyCode::Right => {
                self.cursor_right();
            }
            KeyCode::Up => {
                self.cursor_up();
            }
            KeyCode::Down => {
                self.cursor_down();
            }
            KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                return Some(quit());
            }
            _ => {}
        }
    }
    
    None
}
```

### Visual Rendering

```rust
fn view(&self) -> String {
    let mut output = String::new();
    
    if self.content.is_empty() || (self.content.len() == 1 && self.content[0].is_empty()) {
        // Show placeholder when empty
        if self.focused {
            output.push_str(&format!("{} (placeholder)", self.placeholder));
        } else {
            output.push_str(&self.placeholder);
        }
    } else {
        // Render actual content with cursor
        for (line_idx, line) in self.content.iter().enumerate() {
            if line_idx == self.cursor_line && self.focused && self.show_cursor {
                // Insert cursor at current position
                let (before, after) = line.split_at(self.cursor_col.min(line.len()));
                output.push_str(&format!("{}█{}", before, after));
            } else {
                output.push_str(line);
            }
            
            if line_idx < self.content.len() - 1 {
                output.push('\n');
            }
        }
    }
    
    output.push_str("\n\nPress Ctrl+C to quit");
    output
}
```

### Text Area Dimensions

```rust
// Responsive sizing based on terminal
pub fn set_dimensions(&mut self, width: usize, height: usize) {
    self.width = width;
    self.height = height;
    // TODO: Implement line wrapping based on width
    // TODO: Implement scrolling based on height
}
```

### Real-world Applications

**Code Editor:**
```rust
struct CodeEditor {
    textarea: TextAreaModel,
    language: String,
    syntax_highlighting: bool,
    line_numbers: bool,
}
```

**Form Input:**
```rust
struct CommentForm {
    textarea: TextAreaModel,
    max_length: usize,
    required: bool,
}
```

**Chat Interface:**
```rust
struct ChatInput {
    textarea: TextAreaModel,
    send_on_enter: bool,
    history: Vec<String>,
}
```

### Performance Considerations

**Line-based Storage:**
- Each line is independent for efficient editing
- Line operations don't require processing entire text
- Memory usage scales with content size

**Cursor Operations:**
- O(1) cursor movement within lines
- O(n) for character insertion/deletion (string operations)
- Consider rope data structure for very large texts

### Common Text Editing Features

**Selection Support:**
```rust
pub struct Selection {
    start_line: usize,
    start_col: usize,
    end_line: usize,  
    end_col: usize,
}
```

**Undo/Redo:**
```rust
pub struct EditHistory {
    operations: Vec<EditOperation>,
    current_index: usize,
}
```

**Search and Replace:**
```rust
pub fn find_text(&self, needle: &str) -> Vec<(usize, usize)> {
    // Return positions of matches
}
```

### Testing Text Operations

```rust
#[test]
fn test_text_insertion() {
    let mut textarea = TextAreaModel::new();
    
    textarea.insert_char('H');
    textarea.insert_char('i');
    
    assert_eq!(textarea.content[0], "Hi");
    assert_eq!(textarea.cursor_col, 2);
}

#[test]
fn test_newline_creation() {
    let mut textarea = TextAreaModel::new();
    
    textarea.insert_char('H');
    textarea.insert_char('i');
    textarea.insert_newline();
    textarea.insert_char('B');
    textarea.insert_char('y');
    
    assert_eq!(textarea.content, vec!["Hi".to_string(), "By".to_string()]);
    assert_eq!(textarea.cursor_line, 1);
    assert_eq!(textarea.cursor_col, 2);
}
```

## Related Examples

- **[textinput](../textinput/)** - Single-line text input
- **[textinputs](../textinputs/)** - Multiple text input management
- **[help](../help/)** - Key binding documentation patterns

## Files

- `main.rs` — Complete multi-line text area implementation
- `Cargo.toml` — Dependencies and build configuration  
- `README.md` — This documentation

## Extension Ideas

- Add line wrapping based on textarea width
- Implement vertical scrolling for long content
- Add text selection and copy/paste
- Support for undo/redo operations
- Syntax highlighting for code editing
- Search and replace functionality