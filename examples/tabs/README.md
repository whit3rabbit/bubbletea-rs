# Tabs

<img width="800" src="./tabs.gif" />

A sophisticated tabbed interface example demonstrating advanced layout techniques, border manipulation, and keyboard navigation patterns using bubbletea-rs and lipgloss-extras for styling.

## Features

- **Multi-Tab Navigation**: Switch between 5 different cosmetic product tabs
- **Keyboard Navigation**: Arrow keys, Vim keys (h/j/k/l), and Tab/Shift+Tab support  
- **Visual Border Connections**: Seamless border rendering with custom corner handling
- **Dynamic Content**: Each tab displays different content based on selection
- **Responsive Layout**: Content window automatically adjusts to tab row width
- **Purple Theme**: Consistent color scheme using adaptive purple (#874BFD)
- **Graceful Exit**: Multiple exit options (q, Ctrl+C)

## Running the Example

From the repository root:

```bash
cargo run --example tabs
```

Or from the tabs directory:

```bash
cd examples/tabs
cargo run
```

**Controls:**
- `→` / `l` / `n` / `Tab` - Next tab
- `←` / `h` / `p` / `Shift+Tab` - Previous tab  
- `q` - Quit immediately
- `Ctrl+C` - Quit immediately

## Understanding Tabs

### What Are Tabs?

Tabs are a fundamental UI pattern that allow users to switch between different views or content sections within the same interface. They're essential for:

- **Organizing related content** into logical groups
- **Saving screen space** by showing only active content
- **Providing clear navigation** with visible state indicators
- **Creating familiar user experiences** that users intuitively understand

### Visual Components

```
┌─────────────┬────────┬────────────┬─────────┬────────────┐
│  Lip Gloss  │ Blush  │ Eye Shadow │ Mascara │ Foundation │  ← Tab Row
└─────────────┴──┬─────┴────────────┴─────────┴────────────┘  ← Border Connection
                 │
┌────────────────┴──────────────────────────────────────────┐
│                                                            │
│                    Lip Gloss Tab                           │  ← Content Area
│                                                            │
└────────────────────────────────────────────────────────────┘
```

**Key Visual Elements:**
- **Active Tab**: No bottom border, connecting directly to content
- **Inactive Tabs**: Complete borders with specific corner characters
- **Content Window**: Bordered area displaying current tab's content
- **Border Connections**: Special characters (├, ┤, │) for seamless appearance

## Step-by-Step Implementation

### 1. Setting Up the Model

The foundation is a simple struct tracking tabs and content:

```rust
#[derive(Debug)]
struct TabModel {
    tabs: Vec<String>,        // Tab names to display
    tab_content: Vec<String>, // Content for each tab
    active_tab: usize,        // Index of currently selected tab
}
```

### 2. Creating Custom Tab Borders

Tabs require special border handling to connect properly:

```rust
fn tab_border_with_bottom(left: &'static str, middle: &'static str, right: &'static str) -> Border {
    let mut border = rounded_border();
    border.bottom_left = left;    // ┘ for active, ┴ for inactive
    border.bottom = middle;       // " " for active, "─" for inactive  
    border.bottom_right = right;  // └ for active, ┴ for inactive
    border
}
```

**Border Patterns:**
- **Active Tab**: `("┘", " ", "└")` - Open bottom for content connection
- **Inactive Tab**: `("┴", "─", "┴")` - Closed bottom border

### 3. Handling Navigation

Navigation logic with bounds checking:

```rust
KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('n') | KeyCode::Tab => {
    self.active_tab = min(self.active_tab + 1, self.tabs.len() - 1);
}
KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('p') | KeyCode::BackTab => {
    self.active_tab = max(self.active_tab.saturating_sub(1), 0);
}
```

**Multiple Input Methods:**
- **Arrow Keys**: Standard navigation
- **Vim Keys**: h/l for power users  
- **Letter Keys**: n/p for next/previous
- **Tab Keys**: Tab/Shift+Tab for accessibility

### 4. Rendering the Interface

The view combines styled tabs with content:

1. **Create Styles**: Define active and inactive tab appearance
2. **Render Each Tab**: Apply styles and handle border connections
3. **Join Horizontally**: Combine tabs into a single row
4. **Render Content**: Display content below with matching width
5. **Apply Document Style**: Add overall padding

## Architecture Deep Dive

### Border Character System

Unicode box-drawing characters create seamless connections:

```
Standard Borders:    Special Connections:
┌─┬─┐               ├ (connect left)
│ │ │               ┤ (connect right)  
├─┼─┤               │ (vertical connection)
│ │ │               ┴ (T-junction up)
└─┴─┘               ┘ (corner up-left)
                    └ (corner up-right)
```

**First Tab Connections:**
- Active: `│` - Vertical connection to content
- Inactive: `├` - T-junction allowing content border to continue

**Last Tab Connections:**  
- Active: `│` - Vertical connection to content
- Inactive: `┤` - T-junction allowing content border to continue

### Style Composition Pattern

Building complex styles from base styles:

```rust
// Base inactive style
let inactive_tab_style = Style::new()
    .border_style(inactive_tab_border)
    .border_foreground(Color::from("#874BFD"))
    .padding(0, 1, 0, 1)
    // ... border settings

// Active style inherits from inactive  
let active_tab_style = inactive_tab_style
    .clone()
    .border_style(active_tab_border);  // Only change the border
```

### Layout Calculation

Dynamic width calculation ensures proper alignment:

```rust
let row = join_horizontal(TOP, &rendered_tabs);
let content_width = width(&row) as i32 - window_style.get_horizontal_frame_size();
let content = window_style.width(content_width).render(&content_text);
```

**Why This Works:**
- `width(&row)` gets total tab row width
- `get_horizontal_frame_size()` accounts for border thickness
- Content window matches exactly for seamless connection

## Code Walkthrough

### Model Definition and Initialization

```rust
impl Model for TabModel {
    fn init() -> (Self, Option<Cmd>) {
        (TabModel::default(), Some(init_render_cmd()))
    }
```

**Why init_render_cmd()?** Forces an immediate render on startup, ensuring users see the interface right away instead of waiting for first input.

### Update Logic Flow

```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    // 1. Handle keyboard input for navigation and quitting
    if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
        // Navigation and quit logic
    }
    
    // 2. Handle synthetic init message (no action needed)
    if let Some(_init_msg) = msg.downcast_ref::<InitRenderMsg>() {
        // Triggers initial render
    }
    
    None // No further commands needed
}
```

### View Rendering Pipeline

```rust
fn view(&self) -> String {
    // 1. Define border styles for active/inactive states
    // 2. Create overall styling (tab style, content style, document style)
    // 3. Render each tab with appropriate styling and border connections
    // 4. Join tabs horizontally with TOP alignment
    // 5. Calculate content width and render content window
    // 6. Combine tab row and content vertically
    // 7. Apply document-level styling (padding, etc.)
}
```

## Extending the Example

### Adding More Tabs

```rust
let tabs = vec![
    "Dashboard".to_string(),
    "Analytics".to_string(), 
    "Settings".to_string(),
    "Profile".to_string(),
];
```

### Dynamic Content

```rust
let content = match self.active_tab {
    0 => self.generate_dashboard_content(),
    1 => self.generate_analytics_content(),
    2 => self.generate_settings_content(),
    _ => "Not implemented".to_string(),
};
```

### Custom Colors

```rust
let theme_color = match self.active_tab {
    0 => "#FF6B6B", // Red for dashboard
    1 => "#4ECDC4", // Teal for analytics  
    2 => "#45B7D1", // Blue for settings
    _ => "#874BFD", // Default purple
};
```

### Tab Icons

```rust
let tab_name = format!("{} {}", self.get_tab_icon(i), self.tabs[i]);

fn get_tab_icon(&self, index: usize) -> &str {
    match index {
        0 => "🏠", // Dashboard
        1 => "📊", // Analytics
        2 => "⚙️", // Settings  
        _ => "📄", // Default
    }
}
```

## Common Patterns

### Tab State Management

```rust
#[derive(Debug)]
struct TabModel {
    tabs: Vec<Tab>,  // Instead of just strings
    active_tab: usize,
}

#[derive(Debug)]
struct Tab {
    name: String,
    content: String,
    enabled: bool,    // Disable certain tabs
    badge: Option<u32>, // Show notification counts
}
```

### Keyboard Shortcuts

```rust
KeyCode::Char(c) if ('1'..='9').contains(&c) => {
    let tab_index = (c as usize) - ('1' as usize);
    if tab_index < self.tabs.len() {
        self.active_tab = tab_index;
    }
}
```

### Content Scrolling

```rust
#[derive(Debug)]
struct TabContent {
    text: String,
    scroll_offset: usize,
    height: usize,
}

// In update()
KeyCode::Up => self.scroll_content_up(),
KeyCode::Down => self.scroll_content_down(),
```

## Troubleshooting

### Border Rendering Issues

**Problem:** Tabs don't connect properly to content window
**Solution:** Ensure `join_horizontal(TOP, ...)` aligns tabs at the top edge

**Problem:** Unicode characters display incorrectly  
**Solution:** Ensure terminal supports UTF-8 and has box-drawing character fonts

### Layout Problems

**Problem:** Content window is too narrow/wide
**Solution:** Check `get_horizontal_frame_size()` calculation and ensure consistent border styles

**Problem:** Tabs wrap to multiple lines
**Solution:** Reduce tab count or tab name lengths for terminal width

### Navigation Issues

**Problem:** Can't navigate past first/last tab
**Solution:** Check bounds in `min()`/`max()` calculations

```rust
// Correct bounds checking
self.active_tab = min(self.active_tab + 1, self.tabs.len() - 1);
self.active_tab = max(self.active_tab.saturating_sub(1), 0);
```

## Related Examples

**Next Steps:**
- **[help](../help/)** - Key binding organization for complex tab interfaces
- **[list-fancy](../list-fancy/)** - Combining tabs with selectable lists
- **[composable-views](../composable-views/)** - Multiple UI components together

**Similar Patterns:**
- **[window-size](../window-size/)** - Responsive layout techniques
- **[simple](../simple/)** - Basic Model-View-Update patterns

**Advanced Styling:**
- **[progress-static](../progress-static/)** - Advanced lipgloss styling
- **[spinner](../spinner/)** - Animation within tab content

## Files

- `main.rs` — Complete tabbed interface implementation (177 lines)
- `Cargo.toml` — Dependencies (bubbletea-rs, lipgloss-extras, crossterm, tokio)  
- `tabs.gif` — Demo animation showing navigation
- `README.md` — This comprehensive documentation

## Why Learn Tabs?

Tabs are essential for building professional TUI applications because they demonstrate:

1. **Complex Layout Management**: Multiple visual components working together
2. **State Synchronization**: UI appearance matching application state
3. **Advanced Styling**: Border manipulation and visual composition
4. **User Experience**: Intuitive navigation and clear visual feedback
5. **Scalable Architecture**: Patterns that work for simple and complex interfaces

**Every professional TUI application** uses tab-like patterns for organizing content and navigation.