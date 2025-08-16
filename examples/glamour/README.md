# Glamour

<img width="1200" src="./glamour.gif" />

A demonstration of styled content rendering and viewport scrolling, simulating markdown document display with rich styling, tables, and navigation in terminal applications.

## Features

- **Viewport Scrolling**: Navigate through long content with arrow keys
- **Rich Styling**: Headers, tables, and text with color styling
- **Responsive Layout**: Content adapts to terminal width with proper margins
- **Markdown-style Formatting**: Simulates glamour markdown rendering
- **Bordered Interface**: Rounded border viewport with professional appearance
- **Table Rendering**: Formatted tables with headers and aligned columns

## Running the Example

From the repository root:

```bash
cargo run --example glamour
```

**Controls:**
- `↑`/`k` - Scroll up
- `↓`/`j` - Scroll down
- `q` / `Ctrl+C` - Quit

## What this demonstrates

### Key Concepts for Beginners

**Viewport Pattern**: This example shows how to display content longer than the terminal height:
1. Scrollable content area with fixed height
2. Border and padding around content
3. Keyboard navigation for content exploration
4. Responsive width calculations

**Content Styling**: Demonstrates rich text formatting in terminal applications using colors, bold text, and structured layouts.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{quit, KeyMsg, Model, Msg, Program};
```

**Viewport Widget:**
```rust
use bubbletea_widgets::viewport;
```

- `viewport::new()`: Create scrollable content display
- `viewport::Model`: Pre-built scrolling component
- Built-in keyboard navigation and content management

**Styling System:**
```rust
use lipgloss_extras::lipgloss::{border, Color, Style};
use lipgloss_extras::table::Table;
```

- `Style::new().foreground().bold()`: Text styling
- `Table::new().headers().rows()`: Table creation
- `border::ROUNDED`: Border styling

### Architecture Walkthrough

#### Model Structure
```rust
pub struct GlamourModel {
    viewport: viewport::Model,  // Scrollable content widget
    content: String,           // Pre-rendered styled content
    keys: KeyBindings,         // Navigation key bindings
}
```

#### Content Rendering System

The example creates rich content using manual styling:

```rust
fn render_glamour_content() -> String {
    // Define consistent color scheme
    let h1_style = Style::new()
        .foreground(Color::from("212")) // Bright pink
        .bold(true);
    let h2_style = Style::new()
        .foreground(Color::from("39"))  // Bright blue
        .bold(true);
    
    let mut content = String::new();
    
    // Headers with styling
    content.push_str(&h1_style.render("# Today's Menu"));
    content.push_str(&h2_style.render("## Appetizers"));
    
    // Tables with structured data
    let mut table = Table::new()
        .headers(vec!["Name", "Price", "Notes"])
        .rows(vec![
            vec!["Tsukemono", "$2", "Just an appetizer"],
            vec!["Tomato Soup", "$4", "Made with San Marzano tomatoes"],
        ]);
    
    content.push_str(&table.render());
    content
}
```

#### Viewport Configuration

```rust
fn init() -> (Self, Option<Cmd>) {
    let content = render_glamour_content();
    
    // Create viewport with styled border
    let viewport_style = Style::new()
        .border(border::ROUNDED, true)
        .border_foreground(Color::from("62"))  // Purple border
        .padding([0, 2]);  // Left/right padding
    
    let mut vp = viewport::new(&[
        viewport::with_content(&content),
        viewport::with_width(78),
        viewport::with_height(20),
        viewport::with_style(viewport_style),
    ]);
    
    let model = Self {
        viewport: vp,
        content,
        keys: KeyBindings::default(),
    };
    
    (model, None)
}
```

### Rust-Specific Patterns

**Color Specification:**
```rust
Color::from("212")  // 256-color palette index
Color::from("39")   // Blue
Color::from("#FF00FF")  // Hex colors also supported
```

**Style Chaining:**
```rust
Style::new()
    .foreground(Color::from("212"))
    .bold(true)
    .render("Styled Text")
```

**Table Building:**
```rust
let mut table = Table::new()
    .headers(vec!["Col1", "Col2", "Col3"])
    .rows(vec![
        vec!["Data1", "Data2", "Data3"],
        vec!["More1", "More2", "More3"],
    ]);
```

**Widget Message Forwarding:**
```rust
// Let viewport handle its own navigation messages
self.viewport.update(msg)
```

### Content Organization

The example creates a restaurant menu with multiple sections:

**Headers:** Styled with different colors and bold formatting
**Tables:** Structured data with aligned columns
**Lists:** Checkbox-style items with symbols
**Text Blocks:** Formatted paragraphs with appropriate spacing

### Responsive Design

```rust
// Calculate content width accounting for borders and padding
let content_width = terminal_width.saturating_sub(4);  // 2 padding + 2 border
self.viewport.set_width(content_width);
```

### Simulated Glamour Rendering

**Why "Simulated"?**
The Go version uses the `glamour` library for markdown rendering. Since Rust doesn't have a direct equivalent, this example manually creates styled content that resembles glamour output.

**Color Scheme:**
- Headers: Pink (#212) and Blue (#39)
- Tables: Cyan headers, green prices
- Checkboxes: Green checkmarks, gray boxes
- Text: Default terminal foreground

**Typography Simulation:**
```rust
// Simulate different markdown elements
let h1_style = Style::new().foreground(Color::from("212")).bold(true);
let h2_style = Style::new().foreground(Color::from("39")).bold(true);
let code_style = Style::new().foreground(Color::from("8"));  // Gray
```

### Viewport Widget Features

**Scrolling:**
- Arrow keys and vi-style navigation (j/k)
- Page up/down support
- Content bounds checking

**Styling:**
- Configurable borders (rounded, square, etc.)
- Padding control
- Color customization

**Content Management:**
- Dynamic content updates
- Width/height responsiveness
- Automatic scrollbar indicators

### Performance Considerations

**Content Pre-rendering:**
```rust
// Render content once during initialization
let content = render_glamour_content();
```

**Efficient Scrolling:**
- Widget handles viewport calculations internally
- Only visible content is rendered
- Minimal redraws on scroll

### Real-world Applications

**Documentation Viewers:**
```rust
// Display README files with styling
let content = load_and_style_markdown("README.md")?;
let viewport = create_styled_viewport(content);
```

**Log Viewers:**
```rust
// Colorized log display
let styled_logs = colorize_log_entries(&log_lines);
viewport.set_content(&styled_logs);
```

**Help Systems:**
```rust
// Multi-section help with navigation
let help_content = render_help_sections(&sections);
show_help_viewer(help_content);
```

## Related Examples

- **[help](../help/)** - Another styled content display example
- **[list-simple](../list-simple/)** - Scrollable list navigation
- **[file-picker](../file-picker/)** - Widget-based interface patterns

## Files

- `main.rs` — Complete glamour-style content rendering with viewport
- `Cargo.toml` — Dependencies including lipgloss-extras and bubbletea-widgets
- `glamour.gif` — Demo showing styled content and scrolling
- `README.md` — This documentation

## Future Improvements

- Integration with actual Rust markdown parsing libraries
- Dynamic content loading from files
- Syntax highlighting for code blocks
- Image placeholder rendering
- Link extraction and navigation