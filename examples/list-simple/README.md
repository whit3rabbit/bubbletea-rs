# List Simple

<img width="1200" src="./list-simple.gif" />

A demonstration of basic list navigation using the bubbletea-widgets list component, showing cursor navigation, pagination, and custom item rendering with a clean, numbered interface.

## Features

- **Numbered List Items**: Each item displays with sequential numbering
- **Cursor Navigation**: Up/down arrow keys and vi-style (j/k) navigation
- **Visual Selection**: Selected item highlighted with different color and cursor
- **Pagination**: Automatic pagination for long lists with dot indicators
- **Custom Styling**: Selected items styled differently from normal items
- **Responsive Layout**: Adapts to terminal width changes

## Running the Example

From the repository root:

```bash
cargo run --example list-simple
```

**Controls:**
- `↑`/`k` - Move cursor up
- `↓`/`j` - Move cursor down
- `q` / `Ctrl+C` - Quit

## What this demonstrates

### Key Concepts for Beginners

**List Widget Pattern**: This example shows how to use pre-built list components for:
1. Displaying collections of data
2. Providing keyboard navigation
3. Implementing custom item rendering
4. Managing pagination automatically
5. Creating consistent user interfaces

**Widget Architecture**: Demonstrates the delegate pattern for custom item display while leveraging widget functionality for navigation and state management.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{Cmd, KeyMsg, Model, Msg, WindowSizeMsg};
```

**List Widget System:**
```rust
use bubbletea_widgets::list::{Item, ItemDelegate, Model as List};
use bubbletea_widgets::paginator::Type as PaginatorType;
```

- `List<T>`: Generic list widget for any item type
- `Item` trait: Interface for list items
- `ItemDelegate` trait: Custom rendering logic

**Key Binding Integration:**
```rust
use bubbletea_widgets::key::{matches_binding, new_binding, with_help, with_keys_str, Binding};
```

### Architecture Walkthrough

#### Item Type Definition

```rust
#[derive(Debug, Clone)]
struct FoodItem(String);

impl Item for FoodItem {
    fn filter_value(&self) -> String {
        self.0.clone()  // Used for search/filter functionality
    }
}
```

The `Item` trait enables the widget to work with custom data types.

#### Custom Rendering Delegate

```rust
struct FoodDelegate {
    item_style: Style,           // Normal item styling
    selected_item_style: Style,  // Selected item styling
}

impl ItemDelegate<FoodItem> for FoodDelegate {
    fn render(&self, list: &List<FoodItem>, index: usize, item: &FoodItem) -> String {
        let content = format!("{}. {}", index + 1, item.0);
        
        if index == list.cursor() {
            self.selected_item_style.render(&format!("> {}", content))
        } else {
            self.item_style.render(&content)
        }
    }
    
    fn height(&self) -> usize {
        1  // Each item occupies exactly 1 line
    }
}
```

The delegate pattern separates data (items) from presentation (rendering).

#### List Configuration

```rust
fn init() -> (Self, Option<Cmd>) {
    let items = vec![
        FoodItem("Ramen".to_string()),
        FoodItem("Tomato Soup".to_string()),
        FoodItem("Hamburgers".to_string()),
        // ... more items
    ];
    
    let mut list = List::new(items)
        .with_delegate(Box::new(FoodDelegate::default()))
        .with_height(14)  // Visible area height
        .with_paginator_type(PaginatorType::Dots);  // Dot-style pagination
    
    list.select_first();  // Start with first item selected
    
    let model = Model {
        list,
        keys: KeyBindings::default(),
    };
    
    (model, Some(init_render_cmd()))
}
```

### Rust-Specific Patterns

**Generic List Widget:**
```rust
List<FoodItem>  // Type-safe list for specific item type
```

**Trait Implementation:**
```rust
impl Item for FoodItem {
    fn filter_value(&self) -> String { ... }
}

impl ItemDelegate<FoodItem> for FoodDelegate {
    fn render(&self, list: &List<FoodItem>, index: usize, item: &FoodItem) -> String { ... }
}
```

**Style Configuration:**
```rust
let item_style = Style::new().padding_left(4);
let selected_style = Style::new()
    .padding_left(2)
    .foreground(Color::from("170"));  // Purple highlight
```

**Key Binding Integration:**
```rust
if matches_binding(&self.keys.up, key_msg) {
    return self.list.update(msg);  // Forward to list widget
}
```

### Navigation Behavior

**Cursor Movement:**
- `↑`/`k`: Move to previous item (wraps to bottom)
- `↓`/`j`: Move to next item (wraps to top)
- Smooth scrolling when cursor moves off-screen

**Visual Feedback:**
```
Normal items:     1. Ramen
                  2. Tomato Soup
Selected item:  > 3. Hamburgers    <- highlighted with cursor and color
                  4. Cheeseburgers
```

### Pagination System

**Automatic Pagination:**
- List automatically handles pagination based on height
- Dot indicators show current page position
- Smooth scrolling between pages

**Pagination Types:**
```rust
PaginatorType::Dots     // • • ○ •  (dot style)
PaginatorType::Arabic   // 1 2 3 4  (number style)
```

### Window Responsiveness

```rust
if let Some(window_msg) = msg.downcast_ref::<WindowSizeMsg>() {
    let new_height = (window_msg.height - 4).max(1);  // Leave space for UI
    self.list.set_height(new_height as usize);
}
```

The list adapts its visible area to terminal size changes.

### Custom Styling Examples

**Different Item Styles:**
```rust
// Minimalist style
Style::new().padding_left(2)

// Highlighted style  
Style::new()
    .padding_left(2)
    .foreground(Color::from("170"))  // Purple
    .bold(true)

// With borders
Style::new()
    .border(border::NORMAL, true)
    .padding([1, 2])
```

### Message Flow

1. **User Input**: Arrow keys or j/k pressed
2. **Key Matching**: `matches_binding()` identifies navigation keys
3. **Widget Forwarding**: Message forwarded to list widget
4. **List Update**: Widget handles cursor movement and scrolling
5. **Re-render**: Updated list state triggers view refresh

### Testing List Behavior

```rust
#[test]
fn test_list_navigation() {
    let mut model = Model::new();
    
    // Test initial state
    assert_eq!(model.list.cursor(), 0);
    
    // Test navigation
    model.list.cursor_down();
    assert_eq!(model.list.cursor(), 1);
    
    // Test wrapping
    model.list.cursor_up();
    assert_eq!(model.list.cursor(), 0);
}
```

### Performance Considerations

**Efficient Rendering:**
- Widget only renders visible items
- Scrolling doesn't re-render entire list  
- Delegate pattern allows custom optimization

**Memory Usage:**
- Items stored efficiently in Vec
- Pagination reduces render overhead
- Style objects shared, not duplicated per item

### Real-world Applications

**File Lists:**
```rust
struct FileItem {
    name: String,
    size: u64,
    modified: SystemTime,
}

impl Item for FileItem {
    fn filter_value(&self) -> String {
        self.name.clone()
    }
}
```

**Menu Systems:**
```rust
struct MenuItem {
    label: String,
    action: MenuAction,
    enabled: bool,
}
```

**Search Results:**
```rust
struct SearchResult {
    title: String,
    snippet: String,
    relevance: f64,
}
```

### Extension Ideas

- Add search/filter functionality using `filter_value()`
- Implement multi-selection with checkboxes
- Add sorting capabilities
- Include icons or metadata in item display
- Add context menus or actions on selection

## Related Examples

- **[list-default](../list-default/)** - More advanced list with additional features
- **[list-fancy](../list-fancy/)** - Styled list with complex rendering
- **[file-picker](../file-picker/)** - List-based file navigation

## Files

- `main.rs` — Complete simple list implementation using widgets
- `Cargo.toml` — Dependencies including bubbletea-widgets
- `list-simple.gif` — Demo showing navigation and pagination
- `README.md` — This documentation

## Implementation Tips

- Always implement both `Item` and `ItemDelegate` traits for custom lists
- Use appropriate pagination type for your content
- Consider responsive height for different terminal sizes
- Test navigation with both small and large item collections
- Style selected items clearly for good user experience