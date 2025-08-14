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
bubbletea-widgets = "0.1.10"
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

The `key` module provides a robust, type-safe system for managing keybindings that serves as a higher-level alternative to using `crossterm::event` directly. It allows you to define semantic actions (like "move up") and associate them with multiple physical key presses (e.g., the `up` arrow and `k`). This is essential for building accessible applications and for generating help views with the `Help` component.

**⭐ Recommended Approach**: Use the `key` module's binding system instead of raw `crossterm::event::KeyCode` matching for better maintainability, help integration, and consistent key handling patterns across your application.

#### Why Use the Key API Instead of Crossterm Directly?

**Traditional Crossterm Approach** (❌ Not Recommended):
```rust
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};
use bubbletea_rs::{KeyMsg, Msg};

// Manual key matching - brittle and hard to maintain
fn handle_input(&mut self, msg: &Msg) {
    if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
        match (key_msg.key, key_msg.modifiers) {
            (KeyCode::Char('q'), KeyModifiers::NONE) | 
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                // Quit logic - hard to document for help
            },
            (KeyCode::Up, KeyModifiers::NONE) | 
            (KeyCode::Char('k'), KeyModifiers::NONE) => {
                // Move up - duplicated logic everywhere
            },
            _ => {}
        }
    }
}
```

**Key Module Approach** (✅ Recommended):
```rust
use bubbletea_widgets::key::{Binding, new_binding, with_keys_str, with_help, matches_binding};
use bubbletea_rs::{KeyMsg, Msg};

// Semantic, maintainable, and help-integrated approach
struct AppKeyMap {
    quit: Binding,
    move_up: Binding,
}

impl Default for AppKeyMap {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![
                with_keys_str(&["q", "ctrl+c"]),
                with_help("q/ctrl+c", "quit application"),
            ]),
            move_up: new_binding(vec![
                with_keys_str(&["up", "k"]),
                with_help("↑/k", "move up"),
            ]),
        }
    }
}

fn handle_input(&mut self, key_msg: &KeyMsg) {
    if matches_binding(key_msg, &self.keymap.quit) {
        // Quit logic - automatically documented for help
    } else if matches_binding(key_msg, &self.keymap.move_up) {
        // Move up - reusable binding across components
    }
}
```

**Benefits of the Key API:**
- **Semantic Bindings**: Define what actions mean, not just what keys do
- **Multiple Key Support**: One action can have multiple key combinations
- **Automatic Help Generation**: Built-in integration with the `Help` component
- **Consistency**: Same binding patterns work across all components
- **Maintainability**: Change key bindings in one place
- **Accessibility**: Easy to provide alternative key combinations

#### Core Types

| Type / Trait   | Description                                                                    |
| -------------- | ------------------------------------------------------------------------------ |
| `struct KeyPress` | A type-safe representation of a key press (`KeyCode` + `KeyModifiers`).        |
| `struct Help`     | Help information for displaying keybinding documentation.                      |
| `struct Binding`  | Represents a keybinding with associated keys, help text, and an enabled state. |
| `trait KeyMap`    | An interface for components to expose their keybindings to the `Help` component. |

#### KeyPress API

The `KeyPress` struct represents a specific key press combining a `KeyCode` and `KeyModifiers`.

**Constructors:**
- `From<(KeyCode, KeyModifiers)>` - Create from tuple
- `From<KeyCode>` - Create from KeyCode (no modifiers)  
- `From<&str>` - Create from string representation

**Supported String Formats:**
- Simple keys: "enter", "tab", "esc", "space", "up", "down", "left", "right"
- Function keys: "f1" through "f12"
- Navigation: "home", "end", "pgup"/"pageup", "pgdown"/"pagedown"/"pgdn"
- Special: "backspace", "delete"/"del", "insert"
- Single characters: "a", "1", "?", "/"
- Modifier combinations: "ctrl+c", "alt+f4", "shift+tab"
- Complex combinations: "ctrl+alt+a"

**Usage Examples:**

```rust
use bubbletea_widgets::key::KeyPress;
use crossterm::event::{KeyCode, KeyModifiers};

// From tuple
let ctrl_c: KeyPress = (KeyCode::Char('c'), KeyModifiers::CONTROL).into();

// From KeyCode
let enter: KeyPress = KeyCode::Enter.into();

// From string
let escape: KeyPress = "esc".into();
let alt_f4: KeyPress = "alt+f4".into();
```

#### Help API

The `Help` struct contains human-readable keybinding documentation.

**Fields:**
- `key: String` - Human-readable key representation (e.g., "ctrl+s", "enter")
- `desc: String` - Brief description of what the key binding does

#### Binding API

The `Binding` struct describes a set of keybindings and their associated help text.

**Constructors:**

| Method | Description |
| ------ | ----------- |
| `new<K: Into<KeyPress>>(keys: Vec<K>) -> Self` | Creates a new binding with specified keys |
| `new_binding(opts: Vec<BindingOpt>) -> Self` | Creates a binding using builder options |

**Builder Pattern Methods:**

| Method | Description |
| ------ | ----------- |
| `with_help(key: impl Into<String>, desc: impl Into<String>) -> Self` | Sets help text |
| `with_enabled(enabled: bool) -> Self` | Sets enabled state |
| `with_disabled() -> Self` | Disables the binding |
| `with_keys(keys: &[&str]) -> Self` | Sets keys from string array |

**State Management:**

| Method | Description |
| ------ | ----------- |
| `set_keys<K: Into<KeyPress>>(&mut self, keys: Vec<K>)` | Sets the keys (mutable) |
| `set_help(&mut self, key: impl Into<String>, desc: impl Into<String>)` | Sets help text (mutable) |
| `set_enabled(&mut self, enabled: bool)` | Sets enabled state (mutable) |
| `unbind(&mut self)` | Removes all keys and help text |

**Accessors:**

| Method | Description |
| ------ | ----------- |
| `keys(&self) -> &[KeyPress]` | Returns the key presses |
| `help(&self) -> &Help` | Returns the help information |
| `enabled(&self) -> bool` | Returns true if enabled and has keys |

**Matching:**

| Method | Description |
| ------ | ----------- |
| `matches(&self, key_msg: &KeyMsg) -> bool` | Checks if a KeyMsg matches this binding |
| `matches_any(key_msg: &KeyMsg, bindings: &[&Self]) -> bool` | Static method to check multiple bindings |

#### KeyMap Trait

Components implement `KeyMap` to provide help information to the `Help` component.

**Required Methods:**

| Method | Description |
| ------ | ----------- |
| `short_help(&self) -> Vec<&Binding>` | Returns bindings for single-line help |
| `full_help(&self) -> Vec<Vec<&Binding>>` | Returns organized bindings for multi-column help |

#### Builder Functions

**Primary Builders:**

| Function | Description |
| -------- | ----------- |
| `new_binding(opts: Vec<BindingOpt>) -> Binding` | Go-style binding creation |
| `with_keys_str(keys: &[&str]) -> BindingOpt` | Builder option to set keys from strings |
| `with_keys<K: Into<KeyPress>>(keys: Vec<K>) -> BindingOpt` | Builder option to set keys from KeyPress values |
| `with_help(key: impl Into<String>, desc: impl Into<String>) -> BindingOpt` | Builder option to set help text |
| `with_disabled() -> BindingOpt` | Builder option to disable the binding |

**Utility Functions:**

| Function | Description |
| -------- | ----------- |
| `matches(key_msg: &KeyMsg, bindings: &[&Binding]) -> bool` | Check if KeyMsg matches any binding |
| `matches_binding(key_msg: &KeyMsg, binding: &Binding) -> bool` | Check if KeyMsg matches specific binding |
| `parse_key_string(s: &str) -> KeyPress` | Parse string representation into KeyPress |

**Complete Usage Example:**

```rust
use bubbletea_widgets::key::{Binding, KeyMap, new_binding, with_keys_str, with_help, matches_binding};
use bubbletea_rs::{KeyMsg, Model as BubbleTeaModel};

struct AppKeyMap {
    up: Binding,
    down: Binding,
    quit: Binding,
    save: Binding,
}

impl Default for AppKeyMap {
    fn default() -> Self {
        Self {
            up: new_binding(vec![
                with_keys_str(&["up", "k"]),
                with_help("↑/k", "move up"),
            ]),
            down: new_binding(vec![
                with_keys_str(&["down", "j"]),
                with_help("↓/j", "move down"),
            ]),
            quit: new_binding(vec![
                with_keys_str(&["q", "ctrl+c"]),
                with_help("q/ctrl+c", "quit"),
            ]),
            save: new_binding(vec![
                with_keys_str(&["ctrl+s"]),
                with_help("ctrl+s", "save file"),
            ]),
        }
    }
}

impl KeyMap for AppKeyMap {
    fn short_help(&self) -> Vec<&Binding> {
        vec![&self.up, &self.down, &self.quit]
    }

    fn full_help(&self) -> Vec<Vec<&Binding>> {
        vec![
            vec![&self.up, &self.down],      // Navigation column
            vec![&self.save, &self.quit],    // File operations column
        ]
    }
}

// Usage in update loop
fn handle_keys(&mut self, key_msg: &KeyMsg) -> Option<Cmd> {
    if matches_binding(key_msg, &self.keymap.quit) {
        return Some(bubbletea_rs::quit());
    }
    if matches_binding(key_msg, &self.keymap.save) {
        // Handle save...
        return None;
    }
    None
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
use bubbletea_widgets::key::{Binding, new_binding, with_keys_str, with_help, matches_binding};
use bubbletea_rs::{Cmd, KeyMsg, Model as BubbleTeaModel, Msg};

struct App {
    text_input: TextInput,
    enter_key: Binding,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<Cmd>) {
        let mut ti = textinput_new();
        ti.set_placeholder("Enter your name...");
        let cmd = ti.focus();
        let enter_key = new_binding(vec![
            with_keys_str(&["enter"]),
            with_help("enter", "submit"),
        ]);
        (Self { text_input: ti, enter_key }, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if matches_binding(key_msg, &self.enter_key) {
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

A help component that automatically generates contextual help views from key bindings. It supports both compact single-line displays and expanded multi-column layouts with adaptive styling for light and dark terminal themes.

#### Key Features

- **Dual Display Modes**: Switch between compact and expanded help views
- **Adaptive Styling**: Automatically adjusts colors for light/dark themes  
- **Width Constraints**: Truncates content with ellipsis when space is limited
- **Column Layout**: Organizes key bindings into logical, aligned columns
- **Disabled Key Handling**: Automatically hides disabled key bindings

#### Creating a Help View

**`help::new() -> Model`**
Creates a new help model with default settings.

**`help::Model::new().with_width(width: usize) -> Model`**  
Creates a help model with width constraints for truncation.

#### Public API

| Method                                        | Description                                                          |
| --------------------------------------------- | -------------------------------------------------------------------- |
| `view<K: KeyMap>(&self, keymap: &K) -> String` | Renders the help view based on the provided key map.                 |
| `with_width(self, width: usize) -> Self`     | Sets maximum width with ellipsis truncation.                        |
| `update(self, msg: Msg) -> (Self, Option<Cmd>)` | Compatibility method (no-op for help component).                    |
| `show_all: bool` (field)                      | Toggles between short (single-line) and full (multi-column) help.    |
| `width: usize` (field)                        | Maximum width in characters (0 = no limit).                         |
| `styles: Styles` (field)                      | Styling configuration for all visual elements.                      |

#### View Modes

**Short Help Mode (`show_all = false`)**
Displays key bindings in a horizontal line with bullet separators:
```text
↑/k up • ↓/j down • / filter • q quit • ? more
```

**Full Help Mode (`show_all = true`)**  
Displays key bindings in organized columns:
```text
↑/k      up             / filter         q quit
↓/j      down           esc clear filter ? close help
→/l/pgdn next page      enter apply
←/h/pgup prev page
```

#### KeyMap Implementation Guidelines

The `help::KeyMap` trait defines how your application exposes key bindings:

- **`short_help()`**: Returns 3-6 essential keys for compact display
- **`full_help()`**: Returns grouped key bindings organized into logical columns

```rust
use bubbletea_widgets::help::KeyMap;
use bubbletea_widgets::key::Binding;

impl KeyMap for MyApp {
    fn short_help(&self) -> Vec<&bubbletea_widgets::key::Binding> {
        vec![&self.quit_key, &self.save_key, &self.help_key]
    }

    fn full_help(&self) -> Vec<Vec<&bubbletea_widgets::key::Binding>> {
        vec![
            // Column 1: Navigation
            vec![&self.up_key, &self.down_key, &self.next_page, &self.prev_page],
            // Column 2: Actions  
            vec![&self.save_key, &self.delete_key, &self.edit_key],
            // Column 3: App Control
            vec![&self.help_key, &self.quit_key],
        ]
    }
}
```

#### Basic Usage Example

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_widgets::key::{Binding, new_binding, with_keys_str, with_help, matches_binding};
use bubbletea_widgets::help::{Model as HelpModel, KeyMap};
use bubbletea_rs::{KeyMsg, Model as BubbleTeaModel, Msg};

struct AppKeyMap {
    quit: Binding,
    save: Binding, 
    help: Binding,
}

impl Default for AppKeyMap {
    fn default() -> Self {
        Self {
            quit: new_binding(vec![
                with_keys_str(&["q", "ctrl+c"]),
                with_help("q/ctrl+c", "quit"),
            ]),
            save: new_binding(vec![
                with_keys_str(&["ctrl+s"]),
                with_help("ctrl+s", "save"),
            ]),
            help: new_binding(vec![
                with_keys_str(&["?"]),
                with_help("?", "toggle help"),
            ]),
        }
    }
}

impl KeyMap for AppKeyMap {
    fn short_help(&self) -> Vec<&bubbletea_widgets::key::Binding> {
        vec![&self.save, &self.quit, &self.help]
    }

    fn full_help(&self) -> Vec<Vec<&bubbletea_widgets::key::Binding>> {
        vec![
            vec![&self.save],       // File operations
            vec![&self.help, &self.quit], // App control
        ]
    }
}

struct App {
    keymap: AppKeyMap,
    help: HelpModel,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) {
        (Self {
            keymap: AppKeyMap::default(),
            help: HelpModel::new().with_width(80),
        }, None)
    }

    fn update(&mut self, msg: Msg) -> Option<bubbletea_rs::Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if matches_binding(key_msg, &self.keymap.help) {
                self.help.show_all = !self.help.show_all;
                return None;
            }
            if matches_binding(key_msg, &self.keymap.quit) {
                return Some(bubbletea_rs::quit());
            }
        }
        None
    }

    fn view(&self) -> String {
        let content = "Your application content here...";
        let help_view = self.help.view(&self.keymap);
        format!("{}\n\n{}", content, help_view)
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

#### Enhanced ItemDelegate Trait

The `ItemDelegate` trait now includes callback methods for enhanced interactivity:

| Method                                      | Description                                              |
| ------------------------------------------- | -------------------------------------------------------- |
| `render(&self, m: &Model<I>, index: usize, item: &I) -> String` | Renders an item for display.                            |
| `height(&self) -> usize`                   | Returns the height in lines each item occupies.         |
| `spacing(&self) -> usize`                  | Returns spacing between items.                          |
| `update(&self, msg: &Msg, m: &mut Model<I>) -> Option<Cmd>` | Handles custom update logic.                            |
| `short_help(&self) -> Vec<Binding>`        | Returns key bindings for short help view.               |
| `full_help(&self) -> Vec<Vec<Binding>>`    | Returns organized key bindings for full help view.      |
| `on_select(&self, index: usize, item: &I) -> Option<Cmd>` | Called when an item is selected (Enter key).            |
| `on_remove(&self, index: usize, item: &I) -> Option<Cmd>` | Called before an item is removed.                       |
| `can_remove(&self, index: usize, item: &I) -> bool` | Determines if an item can be removed.                   |

All callback methods have default implementations that return `None` or appropriate defaults, ensuring non-breaking compatibility.

#### Built-in Key Bindings

The List component comes with a comprehensive set of built-in key bindings via `ListKeyMap`:

**Navigation**:
- `↑/k` - Move up one item
- `↓/j` - Move down one item  
- `→/l/pgdn` - Next page
- `←/h/pgup` - Previous page
- `g/home` - Go to start
- `G/end` - Go to end

**Filtering**:
- `/` - Start filtering
- `esc` - Cancel/clear filter
- `enter/tab` - Accept filter

**System**:
- `?` - Toggle help
- `q/esc` - Quit
- `ctrl+c` - Force quit

You can access and customize these through the `keymap` field of your list model or create custom key bindings using the `key` module.

#### Public API

| Method                                      | Description                                              |
| ------------------------------------------- | -------------------------------------------------------- |
| `update(&mut self, msg: Msg) -> Option<Cmd>` | Handles navigation and filtering.                        |
| `view(&self) -> String`                     | Renders the entire list component.                       |
| `selected_item(&self) -> Option<&I>`        | Returns the currently selected item.                     |
| **Direct Item Manipulation**                |                                                          |
| `insert_item(&mut self, index: usize, item: I)` | Inserts an item at the specified index.                 |
| `remove_item(&mut self, index: usize) -> I` | Removes and returns an item at the specified index.     |
| `move_item(&mut self, from: usize, to: usize)` | Moves an item from one position to another.             |
| `push_item(&mut self, item: I)`            | Adds an item to the end of the list.                    |
| `pop_item(&mut self) -> Option<I>`          | Removes and returns the last item.                      |
| **Items Access**                            |                                                          |
| `items(&self) -> &[I]`                     | Gets a reference to all items.                          |
| `items_mut(&mut self) -> &mut Vec<I>`      | Gets a mutable reference to all items.                  |
| `items_len(&self) -> usize`                | Returns the number of items.                            |
| `is_empty(&self) -> bool`                  | Returns true if the list has no items.                  |
| **UI Component Toggles**                    |                                                          |
| `show_title(&self) -> bool`                | Returns whether the title is shown.                     |
| `set_show_title(&mut self, show: bool)`    | Sets whether to show the title.                         |
| `toggle_title(&mut self) -> bool`          | Toggles title visibility and returns new state.         |
| `show_status_bar(&self) -> bool`           | Returns whether the status bar is shown.                |
| `set_show_status_bar(&mut self, show: bool)` | Sets whether to show the status bar.                    |
| `toggle_status_bar(&mut self) -> bool`     | Toggles status bar visibility and returns new state.    |
| `show_spinner(&self) -> bool`              | Returns whether the spinner is shown.                   |
| `set_show_spinner(&mut self, show: bool)`  | Sets whether to show the spinner.                       |
| `toggle_spinner(&mut self) -> bool`        | Toggles spinner visibility and returns new state.       |
| `show_pagination(&self) -> bool`           | Returns whether pagination is shown.                    |
| `set_show_pagination(&mut self, show: bool)` | Sets whether to show pagination.                        |
| `toggle_pagination(&mut self) -> bool`     | Toggles pagination visibility and returns new state.    |
| `show_help(&self) -> bool`                 | Returns whether help is shown.                          |
| `set_show_help(&mut self, show: bool)`     | Sets whether to show help.                              |
| `toggle_help(&mut self) -> bool`           | Toggles help visibility and returns new state.          |
| **Styling Property Access**                 |                                                          |
| `styles(&self) -> &ListStyles`             | Gets a reference to the current styles.                 |
| `styles_mut(&mut self) -> &mut ListStyles` | Gets a mutable reference to the styles.                 |
| `set_styles(&mut self, styles: ListStyles)` | Sets the list styles.                                   |
| `with_styles(self, styles: ListStyles) -> Self` | Builder method to set styles.                           |
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
use bubbletea_widgets::key::{new_binding, with_keys_str, with_help, matches_binding};
use bubbletea_rs::{KeyMsg, Model as BubbleTeaModel, Msg};

struct App {
    list: List<ListDefaultItem>,
    clear_filter_key: key::Binding,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) {
        let items = vec![
            ListDefaultItem::new("Tasks", "View your todo items"),
            ListDefaultItem::new("Settings", "Configure the application"),
            ListDefaultItem::new("Help", "Get assistance"),
        ];
        let list_model = List::new(items, ListDefaultDelegate::new(), 40, 10);
        
        let clear_filter_key = new_binding(vec![
            with_keys_str(&["ctrl+c"]),
            with_help("ctrl+c", "clear filter"),
        ]);
        
        (Self { list: list_model, clear_filter_key }, None)
    }

    fn update(&mut self, msg: Msg) -> Option<bubbletea_rs::Cmd> {
        // Example: Clear filter with Ctrl+C
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            if matches_binding(key_msg, &self.clear_filter_key) {
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

#### Advanced List Usage Example

This example demonstrates the new enhanced features including direct item manipulation, UI toggles, and delegate callbacks:

```rust
use bubbletea_widgets::prelude::*;
use bubbletea_widgets::key::{new_binding, with_keys_str, with_help, matches_binding};
use bubbletea_rs::{Cmd, KeyMsg, Model as BubbleTeaModel, Msg};

// Custom delegate with callback methods
struct CustomDelegate;

impl<I: Item> ItemDelegate<I> for CustomDelegate {
    fn render(&self, m: &Model<I>, index: usize, item: &I) -> String {
        let cursor = if index == m.cursor() { ">" } else { " " };
        format!("{} {}", cursor, item)
    }
    
    fn height(&self) -> usize { 1 }
    fn spacing(&self) -> usize { 0 }
    
    fn update(&self, _msg: &Msg, _m: &mut Model<I>) -> Option<Cmd> { None }
    
    fn on_select(&self, index: usize, item: &I) -> Option<Cmd> {
        println!("Selected item '{}' at index {}", item, index);
        None
    }
    
    fn on_remove(&self, index: usize, item: &I) -> Option<Cmd> {
        println!("Removing item '{}' at index {}", item, index);
        None
    }
    
    fn can_remove(&self, index: usize, _item: &I) -> bool {
        // Don't allow removing the first item
        index != 0
    }
}

struct AppKeyMap {
    add_item: key::Binding,
    remove_item: key::Binding,
    toggle_pagination: key::Binding,
    toggle_help: key::Binding,
}

impl Default for AppKeyMap {
    fn default() -> Self {
        Self {
            add_item: new_binding(vec![
                with_keys_str(&["a"]),
                with_help("a", "add item"),
            ]),
            remove_item: new_binding(vec![
                with_keys_str(&["d"]),
                with_help("d", "remove item"),
            ]),
            toggle_pagination: new_binding(vec![
                with_keys_str(&["p"]),
                with_help("p", "toggle pagination"),
            ]),
            toggle_help: new_binding(vec![
                with_keys_str(&["h"]),
                with_help("h", "toggle help"),
            ]),
        }
    }
}

struct App {
    list: List<ListDefaultItem>,
    keymap: AppKeyMap,
}

impl BubbleTeaModel for App {
    fn init() -> (Self, Option<Cmd>) {
        let items = vec![
            ListDefaultItem::new("Protected Item", "Cannot be removed"),
            ListDefaultItem::new("Task 1", "Complete documentation"),
            ListDefaultItem::new("Task 2", "Review pull requests"),
        ];
        
        let list_model = List::new(items, CustomDelegate, 50, 15)
            .with_show_pagination(true)  // Enable pagination
            .with_show_help(true);       // Enable help display
            
        (Self { 
            list: list_model, 
            keymap: AppKeyMap::default(),
        }, None)
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            // Add new item with 'a'
            if matches_binding(key_msg, &self.keymap.add_item) {
                let new_item = ListDefaultItem::new("New Task", "Added dynamically");
                self.list.push_item(new_item);
                return None;
            }
            // Remove current item with 'd'
            else if matches_binding(key_msg, &self.keymap.remove_item) {
                let cursor = self.list.cursor();
                if cursor < self.list.items_len() {
                    self.list.remove_item(cursor);
                }
                return None;
            }
            // Toggle pagination with 'p'
            else if matches_binding(key_msg, &self.keymap.toggle_pagination) {
                self.list.toggle_pagination();
                return None;
            }
            // Toggle help with 'h'
            else if matches_binding(key_msg, &self.keymap.toggle_help) {
                self.list.toggle_help();
                return None;
            }
        }
        
        self.list.update(msg)
    }

    fn view(&self) -> String {
        let mut output = self.list.view();
        
        output.push_str("\n\nControls:");
        output.push_str("\na - add item, d - remove item");
        output.push_str("\np - toggle pagination, h - toggle help");
        
        output
    }
}
```

**Benefits of Enhanced List API:**

- **Direct Item Manipulation**: Full programmatic control over list contents with `insert_item`, `remove_item`, `move_item`, `push_item`, and `pop_item`
- **UI Component Toggles**: Granular control over which UI elements are displayed (title, status bar, spinner, pagination, help)
- **Styling Property Access**: Direct access to modify list appearance through `styles()`, `styles_mut()`, and builder patterns
- **Enhanced Delegate Callbacks**: Rich interaction model with `on_select`, `on_remove`, and `can_remove` for custom behavior
- **Filter State Management**: Comprehensive API with `is_filtering()`, `clear_filter()`, and `filter_state_info()` for advanced filter handling
- **Non-Breaking Compatibility**: All new features use default implementations ensuring existing code continues to work
- **Complete Feature Parity**: Now matches the full functionality of the original Go bubbles library

### Table

A component for displaying and navigating tabular data with a fixed header.

#### Creating a Table

**`table::new(columns: Vec<Column>) -> Model`**
Creates a new table with the specified column definitions (standard constructor).

**`table::Model::with_options(opts: Vec<TableOption>) -> Model`**
Creates a new table with configuration options (Go-compatible constructor).

#### Constructor Options

| Function                                    | Description                                  |
| ------------------------------------------- | -------------------------------------------- |
| `with_columns(cols: Vec<Column>)`           | Sets the table columns during construction.  |
| `with_rows(rows: Vec<Row>)`                 | Sets the table data rows during construction. |
| `with_height(h: i32)`                       | Sets the table height during construction.   |
| `with_width(w: i32)`                        | Sets the table width during construction.    |
| `with_focused(f: bool)`                     | Sets the initial focus state during construction. |
| `with_styles(s: Styles)`                    | Sets table styling during construction.      |
| `with_key_map(km: TableKeyMap)`             | Sets custom key bindings during construction. |

#### Core Concepts

- **`struct Column { title: String, width: i32 }`**: Defines a table column.
- **`struct Row { cells: Vec<String> }`**: Defines a table row.
- **`type TableOption`**: Configuration option for flexible table construction.

#### Public API

| Method                                      | Description                                  |
| ------------------------------------------- | -------------------------------------------- |
| `with_rows(self, rows: Vec<Row>) -> Self`   | Builder-style method to add rows on creation. |
| `update(&mut self, msg: Msg) -> Option<Cmd>` | Handles navigation.                          |
| `view(&self) -> String`                     | Renders the table.                           |
| `selected_row(&self) -> Option<&Row>`       | Gets the currently selected row.             |
| `move_up(&mut self, n: usize)`              | Moves selection up by n rows.               |
| `move_down(&mut self, n: usize)`            | Moves selection down by n rows.             |
| `goto_top(&mut self)`                       | Moves selection to the first row.           |
| `goto_bottom(&mut self)`                    | Moves selection to the last row.            |
| `set_styles(&mut self, s: Styles)`          | Updates table styles and rebuilds viewport.  |
| `update_viewport(&mut self)`                | Refreshes viewport content.                  |
| `help_view(&self) -> String`                | Returns formatted help text for navigation.  |

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
        let table_model = table::Model::new(columns).with_rows(rows);
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

**Alternative using with_options (Go-compatible pattern):**

```rust
use bubbletea_widgets::table::{Model, with_columns, with_rows, with_height, Column, Row};

let table = Model::with_options(vec![
    with_columns(vec![
        Column::new("City", 15),
        Column::new("Population", 10),
    ]),
    with_rows(vec![
        Row::new(vec!["Tokyo".into(), "37M".into()]),
        Row::new(vec!["Delhi".into(), "32M".into()]),
    ]),
    with_height(15),
]);
```

### FilePicker

A component for navigating the filesystem and selecting a file or directory with comprehensive filtering and configuration options.

#### Creating a FilePicker

**`filepicker::Model::init() -> (Model, Option<Cmd>)`**
Initializes a new file picker, reading the current directory.

**`filepicker::new() -> Model`**
Creates a new file picker instance with default configuration.

#### Public Structs

**`ErrorMsg`**
Represents an error that occurred during file operations.

**`ReadDirMsg`**
Message type for directory reading operations.

#### Public API

| Method                                         | Description                                                                  |
| ---------------------------------------------- | ---------------------------------------------------------------------------- |
| `update(&mut self, msg: Msg) -> Option<Cmd>`   | Handles navigation and selection.                                            |
| `view(&self) -> String`                        | Renders the file list with current configuration.                           |
| `did_select_file(&self, msg: &Msg) -> (bool, String)` | Returns whether a user has selected a file and the file path. Only returns `true` for files that can actually be selected. |
| `did_select_disabled_file(&self, msg: &Msg) -> (bool, String)` | Returns whether a user tried to select a disabled file and the file path. |
| `set_height(&mut self, height: usize)`         | Sets the height of the file picker when auto_height is disabled.            |
| `read_dir(&mut self)`                          | Manually reads the current directory and populates the files list.          |
| `read_dir_cmd(&self) -> Cmd`                   | Creates a command to read the current directory asynchronously.             |

#### Configuration Fields

| Field                    | Type                                           | Description                                    |
| ------------------------ | ---------------------------------------------- | ---------------------------------------------- |
| `file_allowed`           | `bool`                                        | Whether files can be selected                  |
| `dir_allowed`            | `bool`                                        | Whether directories can be selected            |
| `show_hidden`            | `bool`                                        | Whether to display hidden files                |
| `show_permissions`       | `bool`                                        | Whether to display file permissions            |
| `show_size`              | `bool`                                        | Whether to display file sizes                  |
| `auto_height`            | `bool`                                        | Whether to automatically adjust height         |
| `height`                 | `usize`                                       | Fixed height when auto_height is false         |
| `current_directory`      | `PathBuf`                                     | The directory currently being browsed          |
| `allowed_types`          | `Vec<String>`                                 | File extensions that can be selected           |
| `file_selected`          | `String`                                      | Name of the most recently selected file        |
| `cursor`                 | `String`                                      | The cursor string to display (e.g., "> ")     |
| `error`                  | `Option<String>`                              | Error message for failed directory operations  |
| `keymap`                 | `FilepickerKeyMap`                            | Key bindings configuration                     |
| `styles`                 | `Styles`                                      | Visual styling configuration                   |

#### Key Features

- **Cross-platform hidden file detection**: Windows FILE_ATTRIBUTE_HIDDEN + Unix dotfiles
- **Enhanced symlink resolution**: Proper handling of symbolic links with fallback
- **Configurable filtering**: Custom functions for file and directory validation
- **Responsive sizing**: Auto-height with min/max constraints
- **Permission display**: Optional file permission information
- **Size display**: Optional file size formatting

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
        if let (true, path) = self.file_picker.did_select_file(&msg) {
            self.selected_file = Some(path);
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

#### Running the Example

A complete, runnable filepicker example is available:

```bash
cargo run --manifest-path examples/filepicker/Cargo.toml
```

This example demonstrates:
- Keyboard navigation (arrow keys, j/k, h/l)
- File filtering by allowed types (.mod, .sum, .go, .txt, .md, .rs, .toml)
- Permission and size display
- Proper error handling and status messages
- Escape key, 'q', or Ctrl+C to quit
- Enter to select files

### Cursor

A low-level component used inside other components like `TextInput`. It manages the visual state and blinking of the text cursor. It is not typically used standalone.

#### Public API

| Method                           | Description                                            |
| -------------------------------- | ------------------------------------------------------ |
| `set_mode(&mut self, mode: Mode)` | Changes the cursor's behavior (`Blink`, `Static`, `Hide`). |
| `focus(&mut self) -> Option<Cmd>` | Activates the cursor and starts blinking.              |
| `blur(&mut self)`                | Deactivates the cursor.                                |
| `view(&self) -> String`          | Renders the cursor.                                    |