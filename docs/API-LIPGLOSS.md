# lipgloss-rs API Reference

Repo: https://github.com/whit3rabbit/lipgloss-rs
Docs: https://docs.rs/crate/lipgloss/latest

`lipgloss-rs` is a Rust port of the popular Go library for styling terminal layouts. It provides an expressive, declarative API for building beautiful Terminal User Interfaces (TUIs).

This document serves as the official API reference for the entire `lipgloss-rs` ecosystem.

### A Note on Examples

For simplicity and to demonstrate the recommended usage, all examples in this document assume you are using the `lipgloss-extras` facade crate with the `full` feature enabled. This provides access to the core `lipgloss` library and all components under a single dependency.

```toml
[dependencies]
lipgloss-extras = { version = "0.0.8", features = ["full"] }
```

## Table of Contents

- [Installation & Usage](#installation--usage)
- [Core API: `lipgloss`](#core-api-lipgloss)
  - [The `Style` Struct](#the-style-struct)
    - [Creating a Style](#creating-a-style)
    - [Rendering](#rendering)
    - [Text Attributes](#text-attributes)
    - [Colors](#colors)
    - [Layout & Sizing](#layout--sizing)
    - [Borders](#borders)
    - [Unsetting Rules](#unsetting-rules)
  - [Responsive Layouts & Terminal Size](#responsive-layouts--terminal-size)
    - [The Two-Step Process for Responsive Layouts](#the-two-step-process-for-responsive-layouts)
    - [A Complete Example](#a-complete-example)
    - [Handling Dynamic Resizing](#handling-dynamic-resizing)
  - [Layout Functions](#layout-functions)
  - [Color System](#color-system)
    - [Theme-Aware Constants](#theme-aware-constants)
    - [Color Utilities](#color-utilities)
  - [Gradients & Blending](#gradients--blending)
  - [Utility Functions](#utility-functions)
- [Components API](#components-api)
  - [`lipgloss-list`](#lipgloss-list)
  - [`lipgloss-tree`](#lipgloss-tree)
  - [`lipgloss-table`](#lipgloss-table)
- [Security & Performance](#security--performance)

## Installation & Usage

The recommended way to use the library is through the `lipgloss-extras` facade crate, which re-exports the core `lipgloss` library and optional components.

```toml
[dependencies]
# For core functionality + all components
lipgloss-extras = { version = "0.0.8", features = ["full"] }

# Or, pick and choose components
# lipgloss-extras = { version = "0.0.8", features = ["lists", "tables"] }
```

The `prelude` module can be used to bring all common items into scope:

```rust
use lipgloss_extras::prelude::*;

let style = Style::new().foreground(Color::from("201"));
println!("{}", style.render("Hello, world!"));
```

## Core API: `lipgloss`

The core `lipgloss` crate contains the fundamental building blocks for styling, layout, and color management.

### The `Style` Struct

The `Style` struct is the heart of `lipgloss-rs`. It uses a fluent builder pattern to define a set of rules for rendering text.

#### Creating a Style

**`Style::new() -> Style`**

Creates a new, empty style with default settings.

```rust
use lipgloss_extras::lipgloss::Style;

let style = Style::new();
```

#### Rendering

**`render(&self, text: &str) -> String`**

Applies all configured style properties to the given text and returns a string with the appropriate ANSI escape sequences.

**`apply(&self, text: &str) -> String`**

A convenient alias for `render()`.

```rust
use lipgloss_extras::lipgloss::Style;

let style = Style::new().bold(true).foreground("red");
let styled_text = style.render("This is important!");
println!("{}", styled_text);
```

#### Text Attributes

These methods control text formatting like bold, italic, and underline.

| Method                       | Description                               |
| ---------------------------- | ----------------------------------------- |
| `.bold(bool)`                | Sets bold text.                           |
| `.italic(bool)`              | Sets italic text.                         |
| `.underline(bool)`           | Sets underlined text.                     |
| `.strikethrough(bool)`       | Sets strikethrough text.                  |
| `.reverse(bool)`             | Swaps foreground and background colors.   |
| `.blink(bool)`               | Sets blinking text.                       |
| `.faint(bool)`               | Sets faint (dim) text.                    |
| `.underline_spaces(bool)`    | Extends underlines to cover spaces.       |
| `.strikethrough_spaces(bool)`| Extends strikethroughs to cover spaces. |
| `.color_whitespace(bool)`    | Applies background color to whitespace.   |

```rust
use lipgloss_extras::lipgloss::Style;

let special_style = Style::new()
    .bold(true)
    .underline(true)
    .foreground("201");

println!("{}", special_style.render("Special Announcement"));
```

#### Colors

Colors can be set for text, backgrounds, borders, and margins. They accept any type that implements `TerminalColor`, including strings for hex codes and ANSI indices.

| Method                            | Description                                      |
| --------------------------------- | ------------------------------------------------ |
| `.foreground(color)`              | Sets the text color.                             |
| `.background(color)`              | Sets the background color.                       |
| `.margin_background(color)`       | Sets the background color for margin areas.      |
| `.border_foreground(color)`       | Sets the color for all border sides.             |
| `.border_background(color)`       | Sets the background for all border sides.        |
| `.border_top_foreground(color)`   | Sets the color for the top border.               |
| `.border_right_foreground(color)` | Sets the color for the right border.             |
| ... (and so on for all sides)     | ...                                              |

```rust
use lipgloss_extras::lipgloss::{Style, Color, AdaptiveColor};

// Using a hex color string
let style1 = Style::new().background("#7D56F4");

// Using an AdaptiveColor for theme-awareness
let style2 = Style::new().foreground(AdaptiveColor { light: "#333", dark: "#EEE" });
```

#### Layout & Sizing

These methods control the dimensions, spacing, and alignment of styled blocks.

| Method                       | Description                                                   |
| ---------------------------- | ------------------------------------------------------------- |
| `.width(i32)`                | Sets a minimum width. Content will be padded to fit.          |
| `.height(i32)`               | Sets a minimum height. Content will be padded to fit.         |
| `.max_width(i32)`            | Sets a maximum width. Content will wrap or be truncated.      |
| `.max_height(i32)`           | Sets a maximum height. Content will be truncated.             |
| `.padding(t, r, b, l)`       | Sets padding for all four sides.                              |
| `.padding_shorthand(&[i32])`  | Sets padding using CSS-style shorthand (1-4 values).      |
| `.margin(t, r, b, l)`        | Sets margin for all four sides.                               |
| `.margin_shorthand(&[i32])`   | Sets margin using CSS-style shorthand (1-4 values).       |
| `.align_horizontal(pos)`     | Sets horizontal alignment (`LEFT`, `CENTER`, `RIGHT`).        |
| `.align_vertical(pos)`       | Sets vertical alignment (`TOP`, `CENTER`, `BOTTOM`).          |

```rust
use lipgloss_extras::lipgloss::{Style, CENTER, BOTTOM};

let block_style = Style::new()
    .width(40)
    .height(10)
    .padding(1, 2, 1, 2)
    .margin(1, 1, 1, 1)
    .align_horizontal(CENTER)
    .align_vertical(BOTTOM);

println!("{}", block_style.render("Aligned content."));
```

#### Borders

`lipgloss` provides a rich border system with several presets.

| Method                    | Description                                    |
| ------------------------- | ---------------------------------------------- |
| `.border(border)`         | Sets the border style and enables all sides.   |
| `.border_style(border)`   | Sets the border style without enabling sides.  |
| `.border_top(bool)`       | Enables or disables the top border.            |
| ... (and so on for all sides) | ...                                     |

**Available Border Presets:**
`normal_border()`, `rounded_border()`, `thick_border()`, `double_border()`, `block_border()`, `hidden_border()`, `markdown_border()`, `ascii_border()`.

```rust
use lipgloss_extras::lipgloss::{Style, rounded_border};

let border_style = Style::new()
    .border(rounded_border())
    .border_foreground("63");

println!("{}", border_style.render("A bordered box."));
```

#### Unsetting Rules

All style rules can be unset, reverting them to their default state.

```rust
use lipgloss_extras::lipgloss::Style;

let style = Style::new()
    .bold(true)
    .foreground("red")
    .unset_bold(); // No longer bold, but still red

println!("{}", style.render("Just red."));
```

### Responsive Layouts & Terminal Size

`lipgloss-rs` is designed to be fully responsive to the terminal window size. However, it follows a deliberate separation of concerns: **the library provides the tools for layout, but it does not perform terminal I/O itself.**

This means that `lipgloss-rs` does not automatically detect the terminal width or height. Instead, you detect the size using a terminal manipulation library (like `crossterm`, which is already a `lipgloss` dependency) and then pass those dimensions to `lipgloss`'s layout functions.

#### The Two-Step Process for Responsive Layouts

Creating a layout that adapts to the terminal size is a straightforward two-step process:

##### Step 1: Detect the Terminal Size

Use a crate like `crossterm` to get the current number of columns and rows. It's best practice to provide a sensible fallback width in case detection fails (e.g., when the output is piped to a file).

```rust
// crossterm is a dependency of lipgloss, so it's available in your project.
use crossterm::terminal;

fn get_terminal_width() -> i32 {
    match terminal::size() {
        Ok((cols, _rows)) => cols as i32,
        Err(_) => 80, // Default to 80 columns if detection fails
    }
}
```

##### Step 2: Apply the Size with Lipgloss

Once you have the terminal width, use it to configure your `lipgloss` styles and layouts. The most common methods for this are:

*   **`Style::width(i32)`** or **`Style::max_width(i32)`**: Constrains a styled block to the detected width.
*   **`place()`** or **`place_horizontal()`**: Positions a rendered block within the full terminal width, perfect for centering layouts.
*   **`join_horizontal()`**: For creating multi-column layouts that collectively fit within the terminal width.

#### A Complete Example

This example builds a simple responsive layout with a header and a main content area. The entire layout is centered horizontally within the terminal window.

```rust
use lipgloss_extras::lipgloss::{
    join_vertical, place, Style,
    Color, CENTER, LEFT, rounded_border,
};
use crossterm::terminal; // A dependency of lipgloss

fn main() {
    // --- Step 1: Detect Terminal Size ---
    let terminal_width = match terminal::size() {
        Ok((cols, _)) => cols as i32,
        Err(_) => 80, // Fallback width
    };

    // --- Step 2: Create and Apply Responsive Styles ---

    // Header Style
    let header_style = Style::new()
        .bold(true)
        .foreground(Color::from("#FAFAFA"))
        .background(Color::from("#7D56F4"))
        .padding(0, 1, 0, 1);

    // Main Content Style
    let content_style = Style::new()
        .border(rounded_border())
        .border_foreground(Color::from("63"))
        .padding(1, 2, 1, 2);

    // Render the components
    let header = header_style.render("My Awesome App");
    let content = content_style.render("This is the main content area.\nIt will be centered within the terminal.");

    // Combine the components vertically
    let app_view = join_vertical(LEFT, &[&header, &content]);

    // Place the entire application view in the center of the terminal window
    let final_layout = place(
        terminal_width, // Use the detected width here
        lipgloss_extras::lipgloss::height(&app_view) as i32,
        CENTER,
        CENTER,
        &app_view,
        &[],
    );

    println!("{}", final_layout);
}
```

#### Handling Dynamic Resizing

For a true Terminal User Interface (TUI) that reflows when the user resizes the window, you need an event loop. The pattern is:

1.  **Get Initial Size** and render the view.
2.  **Listen for Events** from the terminal.
3.  **Handle Resize Events**: When a resize event occurs, get the new dimensions.
4.  **Re-render**: Call your view function again, passing the new dimensions to `lipgloss`.

This is typically handled by a library like **[`bubbletea-rs`](https://github.com/whit3rabbit/bubbletea-rs)**, which is designed to work perfectly with `lipgloss-rs`. You can also build a simple event loop directly with `crossterm`.

### Layout Functions

**`join_horizontal(pos: Position, strs: &[&str]) -> String`**
**`join_vertical(pos: Position, strs: &[&str]) -> String`**

Combines multiple rendered strings into a single layout.

```rust
use lipgloss_extras::lipgloss::{join_horizontal, TOP};

let block1 = "Block 1";
let block2 = "Block 2\nLine 2";
let layout = join_horizontal(TOP, &[block1, block2]);
println!("{}", layout);
```

### Color System

| Type                | Description                                                          |
| ------------------- | -------------------------------------------------------------------- |
| `Color(String)`     | The primary color type, accepting hex codes or ANSI indices.         |
| `AdaptiveColor`     | Automatically selects a color for light or dark backgrounds.         |
| `CompleteColor`     | Specifies exact colors for each terminal profile (ANSI, 256, TrueColor). |
| `TerminalColor` (trait) | The trait implemented by all color types.                            |

#### Theme-Aware Constants

For building UIs that work well in any terminal theme, `lipgloss` provides a set of `AdaptiveColor` constants.

| Category   | Constants                                                               |
| ---------- | ----------------------------------------------------------------------- |
| **Text**   | `TEXT_PRIMARY`, `TEXT_MUTED`, `TEXT_SUBTLE`, `TEXT_HEADER`                |
| **Accents**| `ACCENT_PRIMARY`, `ACCENT_SECONDARY`, `INTERACTIVE`                     |
| **Status** | `STATUS_SUCCESS`, `STATUS_WARNING`, `STATUS_ERROR`, `STATUS_INFO`         |
| **UI**     | `SURFACE_SUBTLE`, `SURFACE_ELEVATED`, `BORDER_SUBTLE`, `BORDER_PROMINENT` |

```rust
use lipgloss_extras::lipgloss::{Style, TEXT_PRIMARY, STATUS_SUCCESS};

let normal_text = Style::new().foreground(TEXT_PRIMARY);
let success_text = Style::new().foreground(STATUS_SUCCESS);

println!("{}", normal_text.render("This is standard text."));
println!("{}", success_text.render("Operation successful!"));
```

#### Color Utilities

Functions for color manipulation.

- `lighten(color, percent)`: Makes a color lighter.
- `darken(color, percent)`: Makes a color darker.
- `alpha(color, value)`: Adjusts a color's transparency.
- `complementary(color)`: Returns the complementary color.

### Gradients & Blending

Create smooth, perceptually uniform color gradients.

**`gradient(start_hex: &str, end_hex: &str, count: usize) -> Vec<Color>`**
**`bilinear_interpolation_grid(x, y, corners: (&str, &str, &str, &str)) -> Vec<Vec<Color>>`**

```rust
use lipgloss_extras::lipgloss::{gradient, Style};

// Create a gradient bar
let colors = gradient("#FF6B6B", "#4ECDC4", 20);
let mut bar = String::new();
for color in colors {
    bar.push_str(&Style::new().background(color).render(" "));
}
println!("{}", bar);
```

### Utility Functions

- `width(s: &str) -> usize`: Calculates the visible terminal width of a string (Unicode-aware).
- `height(s: &str) -> usize`: Calculates the number of lines in a string.
- `strip_ansi(s: &str) -> String`: Removes all ANSI escape codes from a string.
- `style_ranges(s: &str, ranges: &[Range]) -> String`: Applies different styles to different parts of a string.

## Components API

### `lipgloss-list`

A component for rendering simple and nested lists.

**`List::new() -> List`**

Creates a new list. It uses a builder pattern for configuration.

| Method                     | Description                                  |
| -------------------------- | -------------------------------------------- |
| `.items(vec![&str])`        | Sets the list items.                         |
| `.item(&str)`              | Adds a single item.                          |
| `.item_list(List)`         | Adds a nested sublist.                       |
| `.enumerator(Enumerator)`  | Sets the bullet/numbering style.             |
| `.item_style(Style)`       | Sets the style for all items.                |
| `.enumerator_style(Style)` | Sets the style for all enumerators.          |

**Available Enumerators:** `arabic`, `alphabet`, `roman`, `bullet`, `dash`, `asterisk`.

```rust
use lipgloss_extras::list::{List, roman};

let list = List::new()
    .item("First item")
    .item("Second item")
    .item_list(
        List::new()
            .items(vec!["Sub-item A", "Sub-item B"])
            .enumerator(roman)
    );
println!("{}", list);
```

### `lipgloss-tree`

A component for rendering tree structures.

**`Tree::new() -> Tree`**

| Method                    | Description                                  |
| ------------------------- | -------------------------------------------- |
| `.root(value)`            | Sets the root value of the tree.             |
| `.child(vec![Node])`       | Adds a vector of children.                   |
| `.add_child(Node)`        | Adds a single child.                         |
| `.enumerator(Enumerator)` | Sets the branch character style.             |
| `.item_style(Style)`      | Sets the style for all items.                |

**Available Enumerators:** `default_enumerator`, `rounded_enumerator`.

```rust
use lipgloss_extras::tree::{Tree, Leaf, Node};

let tree = Tree::new().root("Project")
    .child(vec![
        Box::new(Leaf::new("README.md", false)) as Box<dyn Node>,
        Box::new(Tree::new().root("src")
            .child(vec![
                Box::new(Leaf::new("main.rs", false)) as Box<dyn Node>
            ])
        ) as Box<dyn Node>,
    ]);

println!("{}", tree);
```

### `lipgloss-table`

A component for rendering tables with advanced styling and layout options.

**`Table::new() -> Table`**

| Method                       | Description                                                     |
| ---------------------------- | --------------------------------------------------------------- |
| `.headers(vec![&str])`        | Sets the table headers.                                         |
| `.rows(vec![vec![&str]])`     | Sets all data rows.                                             |
| `.row(vec![&str])`            | Adds a single data row.                                         |
| `.width(i32)`                | Sets a fixed width for the table (enables wrapping).            |
| `.border(Border)`            | Sets the border style.                                          |
| `.style_func(fn(row, col))`  | Sets a function to style cells dynamically.                     |
| `.style_func_boxed(closure)` | Sets a styling closure that can capture its environment.        |

```rust
use lipgloss_extras::lipgloss::{Style, Color, CENTER};
use lipgloss_extras::table::{Table, HEADER_ROW};

let data = vec![
    vec!["Pikachu", "Electric", "Static"],
    vec!["Charmander", "Fire", "Blaze"],
];

let style_fn = |row, col| {
    if row == HEADER_ROW {
        return Style::new().bold(true).align_horizontal(CENTER);
    }
    // Custom styling for data rows...
    Style::new()
};

let table = Table::new()
    .headers(vec!["Pokémon", "Type", "Ability"])
    .rows(data)
    .width(40)
    .style_func_boxed(Box::new(style_fn));

println!("{}", table.render());
```

## Security & Performance

`lipgloss-rs` is designed with safety and performance in mind:

- **Memory Safety**: Dimensions for width, height, padding, and margin are automatically clamped to a safe maximum (`10,000`) to prevent memory exhaustion attacks from malicious or oversized input.
- **DoS Protection**: Parsing of ANSI escape sequences is bounded to prevent hangs on malformed input.
- **Performance**:
  - The rendering pipeline is optimized to minimize string allocations.
  - Style comparisons are performed via direct field checks, which is 10-100x faster than string-based comparisons, improving performance in applications that frequently group or compare styles.