# `bubbletea-widgets` API Reference

Repo: https://github.com/whit3rabbit/bubbles-rs
Docs: https://docs.rs/bubbletea-widgets/

`bubbletea-widgets` is a collection of reusable, production-ready TUI components for building terminal applications with [bubbletea-rs](https://crates.io/crates/bubbletea-rs). This library is a Rust port of the popular Go library [bubbles](https://github.com/charmbracelet/bubbles), providing a consistent and predictable API for building complex terminal user interfaces.

## Table of Contents

- [Installation](#installation)
- [Core Concepts](#core-concepts)
  - [Focus Management: The `Component` Trait](#focus-management-the-component-trait)
  - [Key Bindings: The `key` Module](#key-bindings-the-key-module)
- [Components API](#components-api)
  - [Spinner](#spinner)
  - [Progress](#progress)
  - [Timer](#timer)
  - [Stopwatch](#stopwatch)
  - [TextInput](#textinput)
  - [TextArea](#textarea)
  - [Paginator](#paginator)
  - [Viewport](#viewport)
  - [Help](#help)
  - [List](#list)
  - [Table](#table)
  - [FilePicker](#filepicker)
  - [Cursor](#cursor)

## Installation

Add `bubbletea-widgets` to your `Cargo.toml`. You will also need `bubbletea-rs` and `lipgloss-extras` for a complete TUI application.

```toml
[dependencies]
bubbletea-widgets = "0.1.6"
bubbletea-rs = "0.0.6"
lipgloss-extras = { version = "0.0.8", features = ["full"] }
```

The `prelude` module re-exports the most commonly used items for convenience.

```rust
use bubbletea_widgets::prelude::*;
```

## Core Concepts

### Focus Management: The `Component` Trait

Many interactive components (like `TextInput` and `TextArea`) implement the `Component` trait, which provides a standard interface for managing keyboard focus.

| Method                    | Description                                         |
| ------------------------- | --------------------------------------------------- |
| `focus(&mut self) -> Option<Cmd>` | Sets the component to a focused state. May return a command (e.g., to start a cursor blink). |
| `blur(&mut self)`         | Removes focus from the component.                   |
| `focused(&self) -> bool`  | Returns `true` if the component is currently focused. |

This allows you to easily manage which part of your UI is active and receiving input.

```rust
use bubbletea_widgets::prelude::*;

fn manage_focus<T: Component>(component: &mut T) {
    let _cmd = component.focus();
    assert!(component.focused());

    component.blur();
    assert!(!component.focused());
}

let mut input = textinput_new();
manage_focus(&mut input);
```

### Key Bindings: The `key` Module

The `key` module provides a robust, type-safe system for managing keybindings. It allows you to define semantic actions (like "move up") and associate them with multiple physical key presses (e.g., the `up` arrow and `k`). This is essential for building accessible applications and for generating help views with the `Help` component.

| Type / Trait   | Description                                                                    |
| -------------- | ------------------------------------------------------------------------------ |
| `struct Binding` | Represents a keybinding with associated keys, help text, and an enabled state. |
| `struct KeyPress`| A type-safe representation of a key press (`KeyCode` + `KeyModifiers`).        |
| `trait KeyMap`   | An interface for components to expose their keybindings to the `Help` component. |

**Usage Example:**

```rust
use bubbletea_widgets::key::{Binding, KeyMap};
use bubbletea_rs::{KeyMsg, Model as BubbleTeaModel};
use crossterm::event::{KeyCode, KeyModifiers};

struct AppKeyMap {
    up: Binding,
    quit: Binding,
}

impl Default for AppKeyMap {
    fn default() -> Self {
        Self {
            up: Binding::new(vec![KeyCode::Up, KeyCode::Char('k')])
                .with_help("↑/k", "move up"),
            quit: Binding::new(vec![KeyCode::Char('q'), (KeyCode::Char('c'), KeyModifiers::CONTROL).into()])
                .with_help("q/ctrl+c", "quit"),
        }
    }
}
```

## Components API

### Spinner

A spinner indicates that an operation is in progress. It's highly customizable, with several built-in styles.

#### Creating a Spinner

**`spinner::new(opts: &[SpinnerOption]) -> Model`**

Creates a new spinner. `SpinnerOption` can be `with_spinner(Spinner)` or `with_style(Style)`.

#### Predefined Spinners

A variety of preset `Spinner` styles are available as constants: `LINE`, `DOT`, `MINI_DOT`, `JUMP`, `PULSE`, `POINTS`, `GLOBE`, `MOON`, `MONKEY`, `METER`, `HAMBURGER`, `ELLIPSIS`.

#### Public API

| Method                                      | Description                                                    |
| ------------------------------------------- | -------------------------------------------------------------- |
| `update(&mut self, msg: Msg) -> Option<Cmd>` | Advances the spinner animation. Should be called in your `update` loop. |
| `view(&self) -> String`                     | Renders the current spinner frame as a styled string.          |

#### Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{Cmd, Model as BubbleTeaModel, Msg};
use lipgloss_extras::prelude::*;

struct App {
    spinner: Spinner,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<Cmd>) {
        let s = spinner_new(&[
            with_spinner(DOT.clone()),
            with_style(Style::new().foreground(Color::from("205"))),
        ]);
        // Spinners start automatically on init.
        let (spinner_model, cmd) = Spinner::init();
        (Self { spinner: spinner_model }, cmd)
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        self.spinner.update(msg)
    }

    fn view(&self) -> String {
        format!("{} Loading...", self.spinner.view())
    }
}
```

### Progress

A progress bar to visualize the completion of a task. It supports smooth animation and can be a solid color or a gradient.

#### Creating a Progress Bar

**`progress::new(opts: &[ProgressOption]) -> Model`**

Creates a new progress bar. Options include `with_width`, `with_gradient`, `with_solid_fill`, and `without_percentage`.

#### Public API

| Method                                      | Description                                                                  |
| ------------------------------------------- | ---------------------------------------------------------------------------- |
| `set_percent(&mut self, p: f64) -> Cmd`     | Sets the progress and returns a command to start the animation.              |
| `incr_percent(&mut self, v: f64) -> Cmd`    | Increases progress by a given amount and returns an animation command.       |
| `update(&mut self, msg: Msg) -> Option<Cmd>` | Handles animation frames. Should be called from your `update` loop.          |
| `view(&self) -> String`                     | Renders the progress bar based on its current *animated* state.              |
| `view_as(&self, percent: f64) -> String`    | Renders a static view of the progress bar at a specific percentage.          |

#### Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{Cmd, Model as BubbleTeaModel, Msg};

struct App {
    progress: Progress,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<Cmd>) {
        let mut p = Progress::new();
        p.width = 40;
        let cmd = p.set_percent(0.25); // Set initial progress
        (Self { progress: p }, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // The progress model handles its own animation messages
        self.progress.update(msg)
    }

    fn view(&self) -> String {
        format!("Downloading...\n{}", self.progress.view())
    }
}
```

### Timer

A component for counting down from a specified duration.

#### Creating a Timer

**`timer::new(timeout: Duration) -> Model`**
Creates a new timer with a default 1-second interval.

**`timer::new_with_interval(timeout: Duration, interval: Duration) -> Model`**
Creates a timer with a custom update interval.

#### Public API

| Method                                      | Description                                                       |
| ------------------------------------------- | ----------------------------------------------------------------- |
| `init(&self) -> Cmd`                        | Returns a command that starts the timer's tick cycle.             |
| `toggle(&self) -> Cmd`                      | Returns a command to toggle the timer's running state.            |
| `update(&mut self, msg: Msg) -> Option<Cmd>` | Updates the timer's countdown. Call from your `update` loop.      |
| `view(&self) -> String`                     | Renders the remaining time.                                       |
| `timedout(&self) -> bool`                   | Returns `true` if the timer has finished.                         |

#### Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{Cmd, Model as BubbleTeaModel, Msg};
use std::time::Duration;

struct App {
    timer: Timer,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<Cmd>) {
        let t = timer_new(Duration::from_secs(5));
        let cmd = t.init();
        (Self { timer: t }, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(timeout_msg) = msg.downcast_ref::<TimerTimeoutMsg>() {
            if timeout_msg.id == self.timer.id() {
                return Some(bubbletea_rs::quit());
            }
        }
        self.timer.update(msg)
    }

    fn view(&self) -> String {
        format!("Time left: {}", self.timer.view())
    }
}
```

### Stopwatch

A component for counting up from zero.

#### Creating a Stopwatch

**`stopwatch::new() -> Model`**
Creates a new stopwatch with a default 1-second interval.

**`stopwatch::new_with_interval(interval: Duration) -> Model`**
Creates a stopwatch with a custom update interval.

#### Public API

| Method                                      | Description                                                    |
| ------------------------------------------- | -------------------------------------------------------------- |
| `start(&self) -> Cmd`                       | Returns a command to start or resume the stopwatch.            |
| `stop(&self) -> Cmd`                        | Returns a command to pause the stopwatch.                      |
| `reset(&self) -> Cmd`                       | Returns a command to reset the stopwatch to zero.              |
| `update(&mut self, msg: Msg) -> Option<Cmd>` | Updates the stopwatch's elapsed time. Call from your `update` loop. |
| `view(&self) -> String`                     | Renders the elapsed time.                                      |

#### Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{Cmd, Model as BubbleTeaModel, Msg};

struct App {
    stopwatch: Stopwatch,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<Cmd>) {
        let sw = Stopwatch::new();
        let cmd = sw.start();
        (Self { stopwatch: sw }, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        self.stopwatch.update(msg)
    }

    fn view(&self) -> String {
        format!("Elapsed: {}", self.stopwatch.view())
    }
}
```

### TextInput

A single-line text input field, similar to `<input type="text">`. Implements the `Component` trait.

#### Creating a TextInput

**`textinput::new() -> Model`**
Creates a new text input with default settings.

#### Public API

| Method                                      | Description                                                   |
| ------------------------------------------- | ------------------------------------------------------------- |
| `focus(&mut self) -> Cmd`                   | Focuses the input and returns a cursor blink command.         |
| `set_value(&mut self, s: &str)`             | Sets the input's content.                                     |
| `value(&self) -> String`                    | Gets the input's content.                                     |
| `set_placeholder(&mut self, p: &str)`       | Sets the placeholder text.                                    |
| `set_echo_mode(&mut self, mode: EchoMode)`  | Changes the echo mode (e.g., `EchoPassword`).                 |
| `update(&mut self, msg: Msg) -> Option<Cmd>` | Handles user input.                                           |
| `view(&self) -> String`                     | Renders the text input.                                       |

#### Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{Cmd, KeyMsg, Model as BubbleTeaModel, Msg};
use crossterm::event::KeyCode;

struct App {
    text_input: TextInput,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<Cmd>) {
        let mut ti = textinput_new();
        ti.set_placeholder("Enter your name...");
        let cmd = ti.focus();
        (Self { text_input: ti }, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if key_msg.key == KeyCode::Enter {
                return Some(bubbletea_rs::quit());
            }
        }
        self.text_input.update(msg)
    }

    fn view(&self) -> String {
        format!("What's your name?\n{}\n\n(esc to quit)", self.text_input.view())
    }
}
```

### TextArea

A multi-line text input field that supports soft-wrapping, scrolling, and line numbers. Implements the `Component` trait.

#### Creating a TextArea

**`textarea::new() -> Model`**
Creates a new text area with default settings.

#### Public API

| Method                                       | Description                                       |
| -------------------------------------------- | ------------------------------------------------- |
| `focus(&mut self) -> Option<Cmd>`            | Focuses the text area.                            |
| `set_value(&mut self, s: &str)`              | Sets the content.                                 |
| `value(&self) -> String`                     | Gets the content.                                 |
| `set_width(&mut self, w: usize)`             | Sets the width in characters.                     |
| `set_height(&mut self, h: usize)`            | Sets the height in lines.                         |
| `update(&mut self, msg: Option<Msg>) -> Option<Cmd>` | Handles user input and events.                    |
| `view(&self) -> String`                      | Renders the text area.                            |
| Public Fields                                | `show_line_numbers: bool`, `key_map`, styling structs. |

#### Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{Cmd, Model as BubbleTeaModel, Msg};

struct App {
    textarea: TextArea,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<Cmd>) {
        let mut ta = textarea_new();
        ta.placeholder = "Write a short story...".to_string();
        ta.set_width(50);
        ta.set_height(5);
        let cmd = ta.focus();
        (Self { textarea: ta }, cmd)
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        self.textarea.update(Some(msg))
    }

    fn view(&self) -> String {
        format!("Tell me a story:\n{}\n\n(ctrl+c to quit)", self.textarea.view())
    }
}
```

### Paginator

A component for handling pagination logic and rendering pagination UI (e.g., `1/10` or `● ○ ○`).

#### Creating a Paginator

**`paginator::new() -> Model`**
Creates a new paginator.

#### Public API

| Method                                      | Description                                                 |
| ------------------------------------------- | ----------------------------------------------------------- |
| `set_total_items(&mut self, items: usize)`  | Calculates total pages based on item count.                 |
| `set_per_page(&mut self, per_page: usize)`  | Sets how many items are on a page.                          |
| `get_slice_bounds(&self, len: usize)`       | Returns `(start, end)` indices for the current page of a slice. |
| `update(&mut self, msg: &Msg)`              | Handles key presses for navigation.                         |
| `view(&self) -> String`                     | Renders the paginator UI.                                   |

#### Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{Model as BubbleTeaModel, Msg};

struct App {
    paginator: Paginator,
    items: Vec<String>,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) {
        let items: Vec<String> = (1..=100).map(|i| format!("Item {}", i)).collect();
        let mut p = Paginator::new();
        p.set_per_page(10);
        p.set_total_items(items.len());
        (Self { paginator: p, items }, None)
    }

    fn update(&mut self, msg: Msg) -> Option<bubbletea_rs::Cmd> {
        self.paginator.update(&msg);
        None
    }

    fn view(&self) -> String {
        let (start, end) = self.paginator.get_slice_bounds(self.items.len());
        let page_items = &self.items[start..end];
        format!("Items:\n{}\n\n{}", page_items.join("\n"), self.paginator.view())
    }
}
```

### Viewport

A component for viewing and scrolling large blocks of content, both vertically and horizontally.

#### Creating a Viewport

**`viewport::new(width: usize, height: usize) -> Model`**
Creates a new viewport with the specified dimensions.

#### Public API

| Method                                       | Description                                             |
| -------------------------------------------- | ------------------------------------------------------- |
| `set_content(&mut self, content: &str)`      | Sets the content to be displayed.                       |
| `scroll_down(&mut self, n: usize)`           | Scrolls content down by `n` lines.                      |
| `goto_bottom(&mut self)`                     | Jumps to the end of the content.                        |
| `at_bottom(&self) -> bool`                   | Returns `true` if the viewport is at the bottom.        |
| `update(&mut self, msg: Msg) -> Option<Cmd>` | Handles key presses for scrolling.                      |
| `view(&self) -> String`                      | Renders the visible portion of the content.             |

#### Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{Model as BubbleTeaModel, Msg};

struct App {
    viewport: Viewport,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) {
        let mut vp = viewport::new(80, 20);
        vp.set_content("A very long string with many\nlines of text...");
        (Self { viewport: vp }, None)
    }

    fn update(&mut self, msg: Msg) -> Option<bubbletea_rs::Cmd> {
        self.viewport.update(msg)
    }

    fn view(&self) -> String {
        self.viewport.view()
    }
}
```

### Help

A mini help view that automatically generates its content from a `KeyMap`.

#### Creating a Help View

**`help::new() -> Model`**
Creates a new help model.

#### Public API

| Method                                        | Description                                                          |
| --------------------------------------------- | -------------------------------------------------------------------- |
| `view<K: KeyMap>(&self, keymap: &K) -> String` | Renders the help view based on the provided key map.                 |
| `show_all: bool` (field)                      | Toggles between short (single-line) and full (multi-column) help.    |

#### Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{KeyMsg, Model as BubbleTeaModel, Msg};
use crossterm::event::KeyCode;

// 1. Define your KeyMap
struct AppKeyMap { help: Binding, }
// 2. Implement the help::KeyMap trait
impl KeyMap for AppKeyMap {
    fn short_help(&self) -> Vec<&Binding> { vec![&self.help] }
    fn full_help(&self) -> Vec<Vec<&Binding>> { vec![vec![&self.help]] }
}

struct App {
    keymap: AppKeyMap,
    help: HelpModel,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) {
        (Self {
            keymap: AppKeyMap { help: Binding::new(vec![KeyCode::Char('?')]).with_help("?", "help") },
            help: HelpModel::new(),
        }, None)
    }

    fn update(&mut self, msg: Msg) -> Option<bubbletea_rs::Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if self.keymap.help.matches(key_msg) {
                self.help.show_all = !self.help.show_all;
            }
        }
        None
    }

    fn view(&self) -> String {
        format!("Content...\n\n{}", self.help.view(&self.keymap))
    }
}
```

### List

A feature-rich component for browsing items, with filtering, pagination, and status messages.

#### Creating a List

**`list::new(items: Vec<I>, delegate: impl ItemDelegate, width, height) -> Model<I>`**
Creates a new list. Requires items that implement the `Item` trait and a delegate that implements `ItemDelegate`.

#### Core Concepts

- **`trait Item`**: Must implement `Display` and `filter_value()`. `DefaultItem` is provided.
- **`trait ItemDelegate`**: Controls how items are rendered. `DefaultDelegate` is provided.

#### Public API

| Method                                      | Description                                              |
| ------------------------------------------- | -------------------------------------------------------- |
| `update(&mut self, msg: Msg) -> Option<Cmd>` | Handles navigation and filtering.                        |
| `view(&self) -> String`                     | Renders the entire list component.                       |
| `selected_item(&self) -> Option<&I>`        | Returns the currently selected item.                     |
| **Filter State Management**                 |                                                          |
| `is_filtering(&self) -> bool`               | Returns true if filtering is active (Filtering or FilterApplied states). |
| `clear_filter(&mut self) -> Option<Cmd>`   | Forces complete filter clearing in a single operation.  |
| `filter_state_info(&self) -> FilterStateInfo` | Returns detailed information about current filter state. |

#### Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{Model as BubbleTeaModel, Msg};

struct App {
    list: List<ListDefaultItem>,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) {
        let items = vec![
            ListDefaultItem::new("Turtles", "They are slow"),
            ListDefaultItem::new("Cats", "They are cute"),
        ];
        let delegate = ListDefaultDelegate::new();
        let list_model = List::new(items, delegate, 40, 10);
        (Self { list: list_model }, None)
    }

    fn update(&mut self, msg: Msg) -> Option<bubbletea_rs::Cmd> {
        // Press '/' to filter, type, then 'enter' to apply.
        self.list.update(msg)
    }

    fn view(&self) -> String {
        self.list.view()
    }
}
```

#### Enhanced Filter Management

The List component now provides enhanced API methods for programmatic filter state management.

**FilterStateInfo Structure**

```rust
pub struct FilterStateInfo {
    pub state: FilterState,       // Current filter state enum
    pub query: String,           // Current filter query text  
    pub match_count: usize,      // Number of matching items
    pub is_filtering: bool,      // Whether filtering is active
    pub is_clearing: bool,       // Whether in clearing state (future use)
}
```

**Enhanced Usage Example**

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{KeyMsg, Model as BubbleTeaModel, Msg};
use crossterm::event::KeyCode;

struct App {
    list: List<ListDefaultItem>,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) {
        let items = vec![
            ListDefaultItem::new("Tasks", "View your todo items"),
            ListDefaultItem::new("Settings", "Configure the application"),
            ListDefaultItem::new("Help", "Get assistance"),
        ];
        let list_model = List::new(items, ListDefaultDelegate::new(), 40, 10);
        (Self { list: list_model }, None)
    }

    fn update(&mut self, msg: Msg) -> Option<bubbletea_rs::Cmd> {
        // Example: Clear filter with Ctrl+C
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if key_msg.key == KeyCode::Char('c') 
               && key_msg.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                if self.list.is_filtering() {
                    return self.list.clear_filter();
                }
            }
        }
        
        self.list.update(msg)
    }

    fn view(&self) -> String {
        let mut output = self.list.view();
        
        // Show filter status in footer using the new API
        if self.list.is_filtering() {
            let filter_info = self.list.filter_state_info();
            output.push_str(&format!(
                "\nFilter: '{}' ({} matches) | Ctrl+C to clear", 
                filter_info.query, 
                filter_info.match_count
            ));
        }
        
        output
    }
}
```

**Benefits of New API:**

- **`is_filtering()`**: Simple boolean check for conditional UI logic
- **`clear_filter()`**: Programmatic filter clearing without key simulation  
- **`filter_state_info()`**: Rich state information for advanced applications
- **Eliminates workarounds**: No need for double-escape patterns or state parsing

### Table

A component for displaying and navigating tabular data with a fixed header.

#### Creating a Table

**`table::new(columns: Vec<Column>) -> Model`**
Creates a new table with the specified column definitions.

#### Core Concepts

- **`struct Column { title: String, width: i32 }`**: Defines a table column.
- **`struct Row { cells: Vec<String> }`**: Defines a table row.

#### Public API

| Method                                      | Description                                  |
| ------------------------------------------- | -------------------------------------------- |
| `with_rows(self, rows: Vec<Row>) -> Self`   | A builder-style method to add rows on creation. |
| `update(&mut self, msg: Msg) -> Option<Cmd>` | Handles navigation.                          |
| `view(&self) -> String`                     | Renders the table.                           |
| `selected_row(&self) -> Option<&Row>`       | Gets the currently selected row.             |

#### Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{Model as BubbleTeaModel, Msg};

struct App {
    table: Table,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) {
        let columns = vec![
            table::Column::new("City", 15),
            table::Column::new("Population", 10),
        ];
        let rows = vec![
            table::Row::new(vec!["Tokyo".into(), "37M".into()]),
            table::Row::new(vec!["Delhi".into(), "32M".into()]),
        ];
        let table_model = Table::new(columns).with_rows(rows);
        (Self { table: table_model }, None)
    }

    fn update(&mut self, msg: Msg) -> Option<bubbletea_rs::Cmd> {
        self.table.update(msg)
    }

    fn view(&self) -> String {
        self.table.view()
    }
}
```

### FilePicker

A component for navigating the filesystem and selecting a file or directory.

#### Creating a FilePicker

**`filepicker::Model::init() -> (Model, Option<Cmd>)`**
Initializes a new file picker, reading the current directory.

#### Public API

| Method                                         | Description                                                                  |
| ---------------------------------------------- | ---------------------------------------------------------------------------- |
| `update(&mut self, msg: Msg) -> Option<Cmd>`   | Handles navigation and selection.                                            |
| `view(&self) -> String`                        | Renders the file list.                                                       |
| `did_select_file(&self, msg: &Msg) -> (bool, Option<PathBuf>)` | Checks if a file was selected in the last update. Call this in your `update` loop. |

#### Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_rs::{Cmd, Model as BubbleTeaModel, Msg};
use std::path::PathBuf;

struct App {
    file_picker: filepicker::Model,
    selected_file: Option<String>,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<Cmd>) {
        let (picker, cmd) = filepicker::Model::init();
        (Self { file_picker: picker, selected_file: None }, cmd)
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let (true, Some(path)) = self.file_picker.did_select_file(&msg) {
            self.selected_file = Some(path.to_string_lossy().to_string());
            return Some(bubbletea_rs::quit());
        }
        self.file_picker.update(msg)
    }

    fn view(&self) -> String {
        if let Some(file) = &self.selected_file {
            return format!("You picked: {}", file);
        }
        format!("Pick a file:\n{}", self.file_picker.view())
    }
}
```

### Cursor

A low-level component used inside other components like `TextInput`. It manages the visual state and blinking of the text cursor. It is not typically used standalone.

#### Public API

| Method                           | Description                                            |
| -------------------------------- | ------------------------------------------------------ |
| `set_mode(&mut self, mode: Mode)` | Changes the cursor's behavior (`Blink`, `Static`, `Hide`). |
| `focus(&mut self) -> Option<Cmd>` | Activates the cursor and starts blinking.              |
| `blur(&mut self)`                | Deactivates the cursor.                                |
| `view(&self) -> String`          | Renders the cursor.                                    |