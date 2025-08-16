# Split Editors Example

<img width="1200" src="./split-editors.gif" />

A faithful Rust port of the Go Bubble Tea [split-editors example](https://github.com/charmbracelet/bubbletea/tree/main/examples/split-editors), demonstrating advanced terminal UI concepts including full-width cursor line highlighting, focus management, and custom component development.

## ✨ Features

- **Multiple side-by-side text editors** with dynamic resizing
- **Professional cursor line highlighting** - full-width purple background like VS Code
- **Line numbers with end-of-buffer markers** ("~") like Vim
- **Focus management** with visual feedback (rounded borders for active editor)
- **Dynamic editor management** - add up to 6 editors, remove down to 1 minimum
- **Keyboard navigation** - Tab/Shift+Tab to switch between editors
- **Responsive layout** - adapts to terminal window size changes

## 🎯 Key Learning Concepts

### 1. ANSI Escape Codes and Terminal Styling
This example demonstrates how terminal styling works under the hood:
- Colors are represented as ANSI codes (e.g., `\x1b[48;5;57m` for purple background)  
- The `\x1b[0m` reset code clears ALL formatting
- Understanding these codes is crucial for advanced terminal UI development

### 2. The Nested Styling Problem
**Problem**: When you nest styled strings, inner reset codes cancel parent styling:
```rust
// ❌ WRONG - This creates gaps in background color:
let cursor = Style::new().background("212").render("█");  // Contains reset code!
let line = Style::new().background("57").render(format!("text{}more", cursor));
// Result: "text" = purple, "█" = bright, "more" = no background (reset code broke it!)
```

**Solution**: Style parts separately, then concatenate:
```rust
// ✅ CORRECT - Each part gets complete styling:
let styled_text = cursor_line_style().render("text");
let styled_cursor = cursor_line_style().background("212").render("█");  
let styled_more = cursor_line_style().render("more");
let complete = format!("{}{}{}", styled_text, styled_cursor, styled_more);
// Result: Full-width purple background with bright cursor!
```

### 3. Full-Width Background Colors
The `color_whitespace(true)` method is essential for professional-looking editor interfaces:
- Without it: Background only appears behind characters
- With it: Background extends across the entire line width

### 4. Cursor-Following Horizontal Scrolling
One of the most complex features is implementing smooth horizontal scrolling that keeps the cursor visible when typing long lines.

#### The Challenge
Terminal UIs have fixed viewport widths. When text extends beyond the visible area:
- Simple truncation: User can type but can't see what they're typing
- No scrolling: Cursor becomes invisible, breaking the editing experience

#### Our Solution: Smart Viewport Management
```
Full line:    "This is a very long line of text that extends beyond the viewport"
Viewport:             [---------------visible area--------------]
Cursor pos:                                                    ^
```

**Key Concepts:**
- **`horizontal_offset`**: The first visible column index 
- **Visible window**: `horizontal_offset` to `horizontal_offset + width`
- **Cursor following**: Automatically adjust offset to keep cursor visible
- **Smart margins**: Keep 2 characters visible on each side for better UX

#### The Scrolling Algorithm
```rust
fn update_horizontal_scroll(&mut self) {
    const SCROLL_MARGIN: usize = 2;
    let visible_end = self.horizontal_offset + self.width;
    
    // Scroll left if cursor is too far left
    if self.cursor_col < self.horizontal_offset + SCROLL_MARGIN {
        self.horizontal_offset = self.cursor_col.saturating_sub(SCROLL_MARGIN);
    }
    
    // Scroll right if cursor is too far right  
    if self.cursor_col >= visible_end.saturating_sub(SCROLL_MARGIN) {
        self.horizontal_offset = self.cursor_col + SCROLL_MARGIN - self.width + 1;
    }
}
```

#### Visual Example
```
Line: "The quick brown fox jumps over the lazy dog and keeps going..."
Width: 20 characters, Margin: 2

Initial state (cursor at start):
[The quick brown fo]  <- visible window
 ^                    <- cursor at position 0
 offset=0

After typing to position 16:
[The quick brown fo]
                ^     <- cursor near right edge (pos 16)
                      <- Triggers right scroll

After right scroll:
  [ick brown fox jump]  <- window shifted right
                  ^     <- cursor now has margin on right
                        <- offset=2

Typing continues smoothly:
    [brown fox jumps ov]  <- window keeps following cursor
                    ^     <- always visible with margins
                          <- offset=4
```

#### Coordinate Transformation
The tricky part is converting between absolute and visible coordinates:
```rust
// Absolute cursor position in the full line
let absolute_cursor = self.cursor_col;  // e.g., 25

// Visible cursor position in the viewport  
let visible_cursor = absolute_cursor - self.horizontal_offset;  // e.g., 25 - 10 = 15

// Extract visible portion of line
let visible_text = &line[horizontal_offset..horizontal_offset + width];
```

#### When to Reset Scrolling
- **Line changes** (up/down arrows): Reset to show start of new line
- **New lines** (Enter): Reset to position 0  
- **Home key**: Reset to show beginning of current line
- **Horizontal movement**: Only scroll when cursor would go off-screen

## 🏗️ Architecture Overview

### Custom TextArea Implementation
We built a custom `TextArea` instead of using `bubbletea-widgets::textarea` because we needed:
- Precise control over line number rendering and alignment
- Complex cursor line highlighting with ANSI escape code management  
- End-of-buffer markers ("~") for empty lines
- Full integration with lipgloss styling system

This demonstrates when and how to build custom components when existing widgets don't meet your requirements.

### Focus Management System
```rust
struct SplitEditorsModel {
    inputs: Vec<TextArea>,    // Multiple editor instances
    focus: usize,            // Index of currently focused editor
    // ...
}
```

The focus system:
1. Only one editor is focused at a time
2. Focused editor gets rounded borders + cursor line highlighting
3. Tab/Shift+Tab cycles focus between editors
4. Visual feedback helps users understand current state

### Dynamic Layout System
The layout adapts to terminal size using:
- `WindowSizeMsg` for resize events
- Responsive width calculation (70% of terminal width)
- Height constraints to prevent oversized editors
- Consistent spacing and alignment

## 🔧 Code Structure

### Styling Functions (`lines 45-89`)
- `cursor_line_style()` - Purple background for active cursor line
- `focused_border_style()` - Rounded borders for active editor  
- `blurred_border_style()` - Hidden borders for inactive editors

### TextArea Implementation (`lines 91-420`)
- Line-by-line text storage and cursor tracking
- Complex view rendering with line numbers and cursor highlighting
- Key handling for text editing operations

### Horizontal Scrolling System (`lines 160-180, 195+`)
Implements cursor-following horizontal scrolling:
1. **`update_horizontal_scroll()`** - Smart algorithm that maintains cursor visibility
2. **Viewport management** - Tracks `horizontal_offset` for the visible window  
3. **Coordinate transformation** - Converts absolute cursor positions to visible positions
4. **Trigger points** - Called after text edits and cursor movements

### Critical Cursor Line Rendering (`lines 344-424`)
Demonstrates the solution to the nested styling problem:
1. **Split** content into parts (before cursor, cursor char, after cursor, padding)
2. **Style** each part separately with `cursor_line_style().render()`
3. **Concatenate** styled parts without additional nesting
4. **Apply horizontal scrolling** - Extract visible window from full line content
5. **Result** - Perfect full-width purple highlighting with smooth scrolling

### Application Model (`lines 456-641`)
Standard Bubble Tea MVU (Model-View-Update) architecture:
- `init()` - Setup initial state and commands
- `update()` - Handle keyboard input and resize events  
- `view()` - Render all editors with horizontal layout

## ⌨️ Key Bindings

| Key | Action |
|-----|--------|
| `Tab` | Focus next editor |
| `Shift+Tab` | Focus previous editor |
| `Ctrl+N` | Add new editor (max 6) |
| `Ctrl+W` | Remove current editor (min 1) |
| `Esc` / `Ctrl+C` | Quit application |
| Arrow keys | Navigate cursor within editor |
| Text input | Type in focused editor |

## 🚀 Running the Example

```bash
cd examples/split-editors
cargo run
```

Start typing in the first editor, use Tab to switch between editors, and try the different keyboard shortcuts to see focus management and dynamic editor creation in action.

## 🚧 Implementation Challenges & Solutions

### Challenge 1: Coordinate System Complexity
**Problem**: Managing two coordinate systems (absolute vs. visible) gets confusing
**Solution**: Clear naming conventions and comprehensive comments explaining transformations

### Challenge 2: When to Trigger Scrolling
**Problem**: Scrolling at the wrong times creates jarring user experience
**Solution**: Strategic scroll triggers - after edits and horizontal movement only

### Challenge 3: Margin Calculation Edge Cases
**Problem**: Scrolling margins can push viewport beyond line boundaries
**Solution**: Careful use of `saturating_sub()` and boundary checks

### Challenge 4: Cursor Visibility with Complex Styling
**Problem**: Horizontal scrolling + ANSI styling + cursor highlighting interact in complex ways
**Solution**: Apply scrolling first, then handle styling with adjusted coordinates

## 💡 What You'll Learn

1. **Advanced terminal styling** - Understanding ANSI codes and the nested styling pitfall
2. **Custom component development** - When and how to build widgets from scratch
3. **Focus management patterns** - Managing state across multiple interactive components  
4. **Responsive terminal layouts** - Creating professional-looking terminal applications
5. **Cursor-following scrolling** - Complex viewport management with smooth user experience
6. **Coordinate system transformations** - Managing absolute vs. relative positioning
7. **MVU architecture** - Clean separation of concerns in interactive applications

This example showcases production-ready techniques for building sophisticated terminal user interfaces that rival desktop applications in functionality and polish. The horizontal scrolling implementation alone demonstrates advanced TUI development concepts that are applicable to many other interactive terminal applications.