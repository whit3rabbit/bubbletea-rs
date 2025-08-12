# `bubbletea-rs` API Reference

Repo: https://github.com/whit3rabbit/bubbletea-rs
Docs: https://docs.rs/bubbletea-rs/

`bubbletea-rs` is a Rust port of the popular Go library [Bubble Tea](https://github.com/charmbracelet/bubbletea), a framework for building interactive, terminal-based applications. It is based on the [Elm Architecture](https://guide.elm-lang.org/architecture/) (also known as the Model-View-Update pattern), which provides a clean and predictable way to manage application state.

## Table of Contents

- [Installation](#installation)
- [The Elm Architecture (Model-View-Update)](#the-elm-architecture-model-view-update)
  - [The `Model` Trait](#the-model-trait)
  - [Messages (`Msg`)](#messages-msg)
  - [Commands (`Cmd`)](#commands-cmd)
  - [A Complete Example: A Simple Counter](#a-complete-example-a-simple-counter)
- [Common Patterns & Best Practices](#common-patterns--best-practices)
  - [Triggering an Initial Render on Startup](#triggering-an-initial-render-on-startup)
  - [Managing Timers: `tick()` vs. `every()`](#managing-timers-tick-vs-every)
  - [Handling Window Resizing](#handling-window-resizing)
- [Running the Application: The `Program`](#running-the-application-the-program)
  - [Creating a Program](#creating-a-program)
  - [Program Configuration](#program-configuration)
- [Handling Side Effects: The `command` Module](#handling-side-effects-the-command-module)
  - [Application Control](#application-control)
  - [Combining Commands](#combining-commands)
  - [Timers](#timers)
  - [External Processes](#external-processes)
  - [Terminal Control](#terminal-control)
  - [Window and Cursor](#window-and-cursor)
- [Event & Message Reference](#event--message-reference)
- [Error Handling](#error-handling)

## Installation

Add `bubbletea-rs` to your `Cargo.toml`. You will also likely need `crossterm` for key codes and `tokio` as the async runtime.

```toml
[dependencies]
bubbletea-rs = "0.0.6"
crossterm = "0.29"
tokio = { version = "1.0", features = ["full"] }
```

The `prelude` module can be used to bring all common items into scope:

```rust
use bubbletea_rs::prelude::*;
```

## The Elm Architecture (Model-View-Update)

`bubbletea-rs` is built around a simple but powerful pattern: Model-View-Update (MVU).

1.  **Model**: The state of your application.
2.  **View**: A function that renders your `Model` into a `String` to be displayed on the screen.
3.  **Update**: A function that takes a `Message` (like a key press) and your `Model`, and returns an updated `Model` and an optional `Command` to run.

### The `Model` Trait

The core of your application is a struct that implements the `Model` trait. This trait defines the three essential parts of the MVU cycle.

| Method                                      | Description                                                          |
| ------------------------------------------- | -------------------------------------------------------------------- |
| `init() -> (Self, Option<Cmd>)`             | Called once at the start. Returns the initial model state and an optional startup command. |
| `update(&mut self, msg: Msg) -> Option<Cmd>` | Called when a message is received. Updates the model state and can return a new command. |
| `view(&self) -> String`                     | Renders the current model state to a string for display.             |

### Messages (`Msg`)

Messages are events that drive your application forward. They can be anything from a key press, a mouse click, a window resize, or a custom message produced by a command. A `Msg` is a type alias for `Box<dyn Any + Send>`, allowing you to use any struct as a message.

### Commands (`Cmd`)

Commands are asynchronous, potentially long-running operations that can produce messages. They are the primary way to handle side effects like I/O, timers, or running external processes without blocking the UI. A `Cmd` is a future that resolves to an `Option<Msg>`.

### A Complete Example: A Simple Counter

```rust
use bubbletea_rs::{Model, Program, Msg, Cmd, KeyMsg};
use crossterm::event::{KeyCode, KeyModifiers};

// 1. Define the Model
struct Counter { value: i32 }

// 2. Implement the Model trait
impl Model for Counter {
    fn init() -> (Self, Option<Cmd>) { (Self { value: 0 }, None) }
    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('q') | KeyCode::Char('c') if key_msg.modifiers == KeyModifiers::CONTROL => {
                    return Some(bubbletea_rs::quit());
                }
                KeyCode::Up | KeyCode::Char('+') => self.value += 1,
                KeyCode::Down | KeyCode::Char('-') => self.value -= 1,
                _ => {}
            }
        }
        None
    }
    fn view(&self) -> String {
        format!("Counter: {}\n\n[↑/+] increment | [↓/-] decrement | [q/ctrl+c] quit", self.value)
    }
}

// 3. Run the Program
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Program::<Counter>::builder().build()?.run().await?;
    Ok(())
}
```

## Common Patterns & Best Practices

### Triggering an Initial Render on Startup

To perform an action or trigger a render immediately after your application starts (without waiting for user input), return a command from `init()` that sends a message back to your `update` function right away.

```rust
use bubbletea_rs::{command, Model, Msg, Cmd};

struct InitialRenderMsg;

fn initial_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitialRenderMsg) as Msg) })
}

struct MyApp;
impl Model for MyApp {
    fn init() -> (Self, Option<Cmd>) {
        (Self, Some(initial_render_cmd()))
    }
    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if msg.downcast_ref::<InitialRenderMsg>().is_some() {
            // Perform initial action here. The view will be re-rendered.
            println!("Initial render triggered!");
        }
        None
    }
    fn view(&self) -> String { "My App".to_string() }
}
```

### Managing Timers: `tick()` vs. `every()`

`bubbletea-rs` provides two ways to create timers. Understanding their difference is crucial for performance.

-   **`command::tick()`**: A **one-shot** timer. It sends a single message after the specified duration. To create a repeating action, you must return a new `tick()` command from your `update` function each time you receive the tick message. This is the **recommended pattern** for animations and polling.
-   **`command::every()`**: A **continuous** timer. It spawns a background task that sends messages repeatedly forever. It should only be called **once** for a given timer.

> **Warning:** Calling `every()` repeatedly in your `update` loop will spawn numerous background tasks, quickly degrading performance and causing memory leaks. **Always use the `tick()` re-arming pattern for repeating actions.**

### Using `batch()` for Smooth Animations

When combining timers with animations, use `command::batch()` to ensure smooth performance. The `batch()` command is **non-blocking** - it spawns all commands immediately and returns instantly, allowing rapid animation frames to process alongside slower timers.

```rust
// ✅ Good: Smooth animation with batched commands
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    if msg.downcast_ref::<TimerMsg>().is_some() {
        let next_timer = command::tick(Duration::from_secs(1), |_| Box::new(TimerMsg) as Msg);
        let animation = self.progress_bar.animate_to_next_step(); // Returns a 16ms animation command
        
        // batch() spawns both immediately - no blocking!
        return Some(command::batch(vec![next_timer, animation]));
    }
    None
}
```

**Correct Usage (The `tick()` Re-arming Pattern):**
```rust
use bubbletea_rs::{command, Model, Msg, Cmd};
use std::time::Duration;

#[derive(Debug)] struct AnimationFrame;

struct AnimationModel;
impl Model for AnimationModel {
    fn init() -> (Self, Option<Cmd>) {
        // Start the first tick.
        (Self, Some(command::tick(Duration::from_millis(100), |_| Box::new(AnimationFrame) as Msg)))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if msg.downcast_ref::<AnimationFrame>().is_some() {
            // Animation logic here...
            
            // Re-arm the timer for the next frame.
            return Some(command::tick(Duration::from_millis(100), |_| Box::new(AnimationFrame) as Msg));
        }
        None
    }
    fn view(&self) -> String { "Animating...".to_string() }
}
```

### Handling Window Resizing

To create a responsive TUI that adapts to terminal size changes:

1.  **Request Initial Size**: In your `init()` function, return the `command::window_size()` command.
2.  **Handle `WindowSizeMsg`**: In your `update()` function, listen for `WindowSizeMsg` messages. When one is received, update your model's state with the new width and height.
3.  **Use Dimensions in `view()`**: Your `view()` function should use the stored width and height to render the layout. Libraries like `lipgloss-rs` are excellent for this.

```rust
use bubbletea_rs::{command, Model, Msg, Cmd, WindowSizeMsg};

struct ResponsiveModel {
    width: u16,
    height: u16,
}

impl Model for ResponsiveModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = Self { width: 0, height: 0 };
        // Request the initial window size.
        (model, Some(command::window_size()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Listen for resize events.
        if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            self.width = size_msg.width;
            self.height = size_msg.height;
        }
        None
    }

    fn view(&self) -> String {
        // Use self.width and self.height to render a responsive layout.
        format!("Terminal is {} columns wide and {} rows high.", self.width, self.height)
    }
}
```

## Running the Application: The `Program`

The `Program` is the runtime that manages your application's lifecycle.

### Creating a Program

You create a `Program` using the `ProgramBuilder`.

**`Program::<M>::builder() -> ProgramBuilder<M>`**
Creates a builder for configuring the program.

**`builder.build() -> Result<Program<M>, Error>`**
Builds the program with the specified configuration.

**`program.run().await -> Result<M, Error>`**
Runs the application to completion.

### Program Configuration

| Method                       | Description                                                     |
| ---------------------------- | --------------------------------------------------------------- |
| `.alt_screen(bool)`          | Use the alternate screen buffer (preserves shell history).      |
| `.mouse_motion(MouseMotion)` | Set the mouse event reporting mode (`None`, `Cell`, or `All`).  |
| `.with_fps(u32)`             | Set the target frames per second for rendering.                 |
| `.catch_panics(bool)`        | Catch panics and convert them to `ProgramPanic` errors. Default `true`. |
| `.input(reader)`             | Use a custom stream for input (e.g., a file) instead of stdin.  |
| `.output(writer)`            | Use a custom stream for output instead of stdout.               |

## Handling Side Effects: The `command` Module

Commands are how you interact with the world outside your `update` function.

### Application Control

| Command          | Description                               |
| ---------------- | ----------------------------------------- |
| `quit()`         | Sends a `QuitMsg` to shut down the program gracefully. |
| `kill()`         | Sends a `KillMsg` to terminate the program immediately. |

### Combining Commands

| Command                 | Description                                    |
| ----------------------- | ---------------------------------------------- |
| `batch(Vec<Cmd>)`       | **Non-blocking.** Spawns all commands immediately and concurrently. Returns instantly without waiting for completion. Perfect for animations with timers. |
| `sequence(Vec<Cmd>)`    | Executes a list of commands sequentially.      |

### Timers

| Command                               | Description                                                                  |
| ------------------------------------- | ---------------------------------------------------------------------------- |
| `tick(duration, Fn)`                  | **One-shot timer.** Produces a single message **after the full duration has passed**. Recommended for repeating actions via re-arming. |
| `every(duration, Fn)`                 | **Continuous timer.** Produces messages repeatedly. Call only once. |
| `every_with_id(duration, Fn)`         | Like `every`, but returns a `(Cmd, u64)` tuple for cancellation. |
| `cancel_timer(u64)`                   | Cancels a specific timer by its ID.                                          |
| `cancel_all_timers()`                 | Cancels all active timers.                                                   |

### External Processes

**`exec_process(std::process::Command, Fn) -> Cmd`**

Executes an external command asynchronously and returns its output as a message.

### Terminal Control

| Command                 | Description                               |
| ----------------------- | ----------------------------------------- |
| `enter_alt_screen()`    | Enters the alternate screen buffer.       |
| `exit_alt_screen()`     | Exits the alternate screen buffer.        |
| `enable_mouse_...()`    | Enables different levels of mouse reporting. |
| `enable_bracketed_paste()`| Enables bracketed paste mode.             |

### Window and Cursor

| Command                 | Description                               |
| ----------------------- | ----------------------------------------- |
| `set_window_title(String)`| Sets the terminal window title.           |
| `window_size()`         | Requests the terminal size. Responds with `WindowSizeMsg`. |
| `show_cursor()` / `hide_cursor()` | Changes cursor visibility. |
| `clear_screen()`        | Clears the terminal screen.               |

## Event & Message Reference

| Message               | Description                                           |
| --------------------- | ----------------------------------------------------- |
| `KeyMsg`              | A keyboard event.                    |
| `MouseMsg`            | A mouse event.                       |
| `WindowSizeMsg`       | The terminal was resized.                    |
| `PasteMsg`            | Text was pasted (if bracketed paste is enabled). |
| `FocusMsg` / `BlurMsg`| The terminal gained or lost focus.          |
| `QuitMsg`             | Gracefully shut down the program.          |
| `KillMsg`             | Terminate the program immediately.         |
| `InterruptMsg`        | An interrupt signal (e.g., Ctrl+C) was caught. |

## Error Handling

The `program.run()` method returns a `Result<M, Error>`. The `Error` enum covers various failure modes.

| Error Variant     | Description                                                     |
| ----------------- | --------------------------------------------------------------- |
| `ProgramPanic(String)` | The application logic (`update` or `view`) panicked. |
| `ProgramKilled`   | The program was terminated by a `kill()` command.             |
| `Interrupted`     | The program was interrupted (e.g., by Ctrl+C).                |
| `Io(std::io::Error)` | An I/O error occurred with the terminal.                      |