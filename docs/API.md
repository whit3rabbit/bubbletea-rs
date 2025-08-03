# bubbletea-rs API Documentation

This document summarizes the public API surface of the `bubbletea-rs` crate and provides a Go → Rust translation table for developers familiar with the Go Bubble Tea framework.

- Crate entry: `src/lib.rs`
- Primary modules: `command`, `error`, `event`, `gradient`, `input`, `logging` (feature-gated), `memory`, `model`, `program`, `terminal`
- Common prelude: `bubbletea_rs::prelude::{Cmd, Error, Model, Msg, Program, KeyMsg, MouseMsg, QuitMsg, KillMsg, WindowSizeMsg}`

Original Go Bubbletea documentation: [bubbletea](https://pkg.go.dev/github.com/charmbracelet/bubbletea)

## Table of Contents

- [Module Overview and Public API](#module-overview-and-public-api)
- [Prelude](#prelude)
- [Go → Rust Translation Guide](#go--rust-translation-guide)
- [Detailed Signatures](#detailed-signatures)
- [Common Usage Examples](#common-usage-examples)
- [Go → Rust Differences (Expanded)](#go--rust-differences-expanded)
- [Go → Rust API Reference Tables](#go--rust-api-reference-tables)
- [Additional Go → Rust Mappings](#additional-go--rust-mappings-from-go-index-non-deprecated)
- [Go options sweep](#go-options-sweep-non-deprecated)
- [Notes on Program::wait](#notes-on-programwait)
- [Migration from Go to Rust](#migration-from-go-to-rust)
- [Appendix: Side-by-side Go vs Rust Examples](#appendix-side-by-side-go-vs-rust-examples)

---

## Module Overview and Public API

Below is an organized overview of the main public items, focusing on what is re-exported from `lib.rs` and the core responsibilities of each module.

### <a id="sig-command"></a> command
Command constructors and helpers. Commands (`Cmd`) are async actions that can optionally return a message (`Msg`).

Public re-exports:
- Functions (commands):
  - `quit()`, `kill()`, `interrupt()`
  - Terminal control: `enter_alt_screen()`, `exit_alt_screen()`, `hide_cursor()`, `show_cursor()`, `clear_screen()`, `set_window_title(title: impl Into<String>)`
  - Mouse & focus reporting: `enable_mouse_cell_motion()`, `enable_mouse_all_motion()`, `disable_mouse()`, `enable_report_focus()`, `disable_report_focus()`, `enable_bracketed_paste()`, `disable_bracketed_paste()`
  - <a id="cmd-timers"></a> Timers: `tick(dur: Duration, msg: impl FnOnce() -> Msg)`, `every(dur: Duration, make_msg: impl Fn() -> Msg)`, `every_with_id(id: impl Into<String>, dur: Duration, make_msg: impl Fn() -> Msg)`, `cancel_timer(id: impl Into<String>)`, `cancel_all_timers()`
  - Rendering helpers: `printf(fmt: &str, ...)`, `println(line: impl AsRef<str>)`
  - <a id="cmd-exec"></a> External process: `exec_process(cmd: &str, args: &[&str])`
  - <a id="cmd-batch"></a> Batching/sequencing: `batch(cmds: impl IntoIterator<Item = Cmd>)`, `sequence(cmds: impl IntoIterator<Item = Cmd>)`
- Types: `Cmd`, `Batch`

Notes:
- `Cmd` is a pinned boxed future yielding `Option<Msg>`.
- Use `batch` to run commands concurrently; `sequence` to run in order.

### error
Unified error type for the crate.

Public re-exports:
- `Error` (enum): covers program panics/kills/interrupts, I/O, terminal errors, channel errors, configuration errors, command execution errors, and conversions from async runtimes (e.g., Tokio join errors).

### event
Message definitions and sender/receiver abstractions used by the runtime and input.

Public re-exports (selected):
- Core message alias: `Msg` (boxed `dyn Any + Send`)
- User-input and system messages: `KeyMsg`, `MouseMsg`, `WindowSizeMsg`, `FocusMsg`, `BlurMsg`
- Program control messages: `QuitMsg`, `KillMsg`, `InterruptMsg`, `SuspendMsg`, `ResumeMsg`
- Terminal control messages: `EnterAltScreenMsg`, `ExitAltScreenMsg`, `HideCursorMsg`, `ShowCursorMsg`, `ClearScreenMsg`, `SetWindowTitleMsg`, `EnableMouseCellMotionMsg`, `EnableMouseAllMotionMsg`, `DisableMouseMsg`, `EnableReportFocusMsg`, `DisableReportFocusMsg`, `EnableBracketedPasteMsg`, `DisableBracketedPasteMsg`, `RequestWindowSizeMsg`, `PrintMsg`, `PrintfMsg`
- Event channels: `EventSender`, `EventReceiver`
- Internal batching message: `BatchMsgInternal`

### input
Facilities to read events from the terminal or custom async input sources.

Public re-exports:
- `InputHandler`: reads terminal events or custom sources and emits `Msg` variants
- `InputSource`: abstraction for plugging in custom line-based async sources

### model
The MVU trait your application implements.

Public re-exports:
- `Model` (trait):
  - `fn init() -> (Self, Option<Cmd>)`
  - `fn update(&mut self, msg: Msg) -> Option<Cmd>`
  - `fn view(&self) -> String`

Notes:
- `Msg` is dynamically typed. Use `downcast_ref::<T>()` on `Msg` to handle your message types.
- `init` can schedule a startup command; `update` may schedule follow-up work.

### <a id="sig-program"></a> program
Program orchestration, configuration, and execution.

Public re-exports:
- `Program<M: Model>`: drives the event loop, input, update, and rendering
- `ProgramBuilder<M: Model>`: fluent builder to construct `Program`
- `ProgramConfig`: runtime configuration (terminal modes, rendering, panic handling, signal handling, etc.)
- `MouseMotion`: mouse reporting mode enum (e.g., cell vs. all motion)

Types:
- `type MessageFilter<M> = Box<dyn Fn(&M, Msg) -> Option<Msg> + Send>`

Typical usage:
```rust
let program = Program::<MyModel>::builder()
    // .with_config(ProgramConfig { ... })
    .build()?;
program.run().await?;
```

### terminal
Terminal abstraction and implementation.

Public re-exports:
- `TerminalInterface` (trait): async operations for raw mode, alt screen, mouse/focus reporting, bracketed paste, cursor, clearing, rendering, size querying
- `Terminal`: default terminal implementation
- `DummyTerminal`: no-op implementation for tests or non-TTY

### gradient
ANSI gradient utilities for colorful UI text.

Public re-exports:
- `charm_default_gradient(start: usize, end: usize) -> Vec<(u8,u8,u8)>`
- `gradient_filled_segment(text: &str, colors: &[(u8,u8,u8)]) -> String`
- `gradient_filled_segment_with_buffer(text: &str, colors: &[(u8,u8,u8)], buffer: &mut String)`
- `lerp_rgb(a: (u8,u8,u8), b: (u8,u8,u8), t: f32) -> (u8,u8,u8)`

### memory
Counters and health checks for program memory/resource usage.

Public re-exports:
- `MemoryMonitor`, `MemorySnapshot`, `MemoryHealth`

### logging (feature = "logging")
Optional logging helper.

Public re-exports when enabled:
- `log_to_file(path: impl AsRef<std::path::Path>) -> Result<(), Error>`

---

## Prelude

For convenience, import the prelude to get the most common items:

```rust
use bubbletea_rs::prelude::{Cmd, Error, Model, Msg, Program, KeyMsg, MouseMsg, QuitMsg, KillMsg, WindowSizeMsg};
```

---

## Go → Rust Translation Guide

This section maps Go Bubble Tea concepts to their `bubbletea-rs` equivalents and highlights any behavioral differences.

### Core Concepts
- Go `tea.Model` → Rust `Model` trait
  - Go: `Init() tea.Cmd`, `Update(tea.Msg) (tea.Model, tea.Cmd)`, `View() string`
  - Rust: `fn init() -> (Self, Option<Cmd>)`, `fn update(&mut self, Msg) -> Option<Cmd>`, `fn view(&self) -> String`
  - Note: Rust mutates `self` in-place and returns an optional `Cmd`.

- Go `tea.Msg` → Rust `Msg`
  - Go: interface
  - Rust: `Box<dyn Any + Send>`; downcast at handling sites.

- Go `tea.Cmd` → Rust `Cmd`
  - Go: `func() tea.Msg`
  - Rust: `Pin<Box<dyn Future<Output = Option<Msg>> + Send>>`
  - Both represent deferred/async work that can emit a message.

- Go `tea.Program` → Rust `Program`/`ProgramBuilder`
  - Go: `tea.NewProgram(m, options...)`
  - Rust: `Program::<M>::builder().build()?` with optional `ProgramConfig`.

### Common Commands
- Quit: Go `tea.Quit` → Rust `quit()`
- Kill: Go `tea.Kill` → Rust `kill()`
- Interrupt: Go `tea.Interrupt` → Rust `interrupt()`
- Suspend/Resume (terminal): Go has `tea.Suspend`/`tea.Resume` messages in some contexts → Rust `suspend()`/`resume()` as messages and via commands that toggle terminal modes
- Alt screen: Go `tea.EnterAltScreen`/`ExitAltScreen` → Rust `enter_alt_screen()`/`exit_alt_screen()`
- Cursor: Go `tea.HideCursor`/`ShowCursor` → Rust `hide_cursor()`/`show_cursor()`
- Clear screen: Go `tea.ClearScreen` → Rust `clear_screen()`
- Window title: Go `tea.SetWindowTitle(title)` → Rust `set_window_title(title)`
- Printing: Go `tea.Println`, `tea.Printf` → Rust `println()`, `printf()`
- Mouse reporting: Go `tea.EnableMouseCellMotion`/`EnableMouseAllMotion`/`DisableMouse` → Rust equivalents with same names
- Focus reporting: Go `tea.EnableReportFocus`/`DisableReportFocus` → Rust equivalents with same names
- Bracketed paste: Go `tea.EnableBracketedPaste`/`DisableBracketedPaste` → Rust equivalents with same names
- Ticking/timers:
  - Go `tea.Tick` and common patterns using `time.Tick`/`time.After` → Rust `tick()`, `every()`, `every_with_id()`, with `cancel_timer()`/`cancel_all_timers()`

### Messages
- Keyboard: Go `tea.KeyMsg` → Rust `KeyMsg`
- Mouse: Go `tea.MouseMsg` → Rust `MouseMsg` (configure via `MouseMotion` in `ProgramConfig` or with mouse commands)
- Window size: Go `tea.WindowSizeMsg` → Rust `WindowSizeMsg`
- Focus/blur: Go `tea.FocusMsg`/`BlurMsg` → Rust `FocusMsg`/`BlurMsg`
- Program control: Go `tea.QuitMsg`/`InterruptMsg` → Rust `QuitMsg`/`InterruptMsg`; `SuspendMsg`/`ResumeMsg` also available in Rust; Immediate termination: Go `tea.Kill`/`Program.Kill()` → Rust `kill()`/`Program::kill()` sending `KillMsg`

### Errors
- Go typically returns `error` values from `program.Run()` → Rust uses `Result<_, Error>` where `Error` is a comprehensive enum (`error` module)

### Terminal Abstraction
- Go uses the underlying TTY through Bubble Tea internals → Rust exposes `TerminalInterface` and concrete `Terminal`/`DummyTerminal` for advanced control and testing.

### Feature Differences and Notes
- Rust `Msg` is type-erased; define and downcast your own message structs/enums to use strong typing across your app.
- Rust offers built-in gradient utilities (`gradient` module), which are not part of Go Bubble Tea core.
- Logging via `log_to_file` is feature-gated; enable the `logging` feature to use it.
- `Program::kill()` and `kill()` command added for immediate termination, emitting `KillMsg`.

---

## Detailed Signatures

This section lists concrete function/type signatures as implemented.

### command
- Types
  - `type Cmd = Pin<Box<dyn Future<Output = Option<Msg>> + Send>>`

- <a id="cmd-core"></a> Core control
  - <a id="cmd-quit"></a> `fn quit() -> Cmd`
  - <a id="cmd-kill"></a> `fn kill() -> Cmd`
  - <a id="cmd-interrupt"></a> `fn interrupt() -> Cmd`
  - <a id="cmd-suspend"></a> `fn suspend() -> Cmd`

- Batching/sequencing
  - <a id="cmd-batch"></a> `fn batch(cmds: Vec<Cmd>) -> Cmd`
  - <a id="cmd-sequence"></a> `fn sequence(cmds: Vec<Cmd>) -> Cmd`

- <a id="cmd-timers"></a> Timers
  - <a id="cmd-tick"></a> `fn tick<F>(duration: Duration, f: F) -> Cmd`
    - where `F: Fn(Duration) -> Msg + Send + 'static`
  - <a id="cmd-every"></a> `fn every<F>(duration: Duration, f: F) -> Cmd`
    - where `F: Fn(Duration) -> Msg + Send + 'static`
  - <a id="cmd-every-with-id"></a> `fn every_with_id<F>(duration: Duration, f: F) -> (Cmd, u64)`
    - where `F: Fn(Duration) -> Msg + Send + 'static`
  - <a id="cmd-cancel-timer"></a> `fn cancel_timer(timer_id: u64) -> Cmd`
  - <a id="cmd-cancel-all-timers"></a> `fn cancel_all_timers() -> Cmd`

- <a id="cmd-exec"></a> External process
  - <a id="cmd-exec-process"></a> `fn exec_process<F>(cmd: std::process::Command, f: F) -> Cmd`
    - where `F: Fn(Result<std::process::Output, std::io::Error>) -> Msg + Send + 'static`

- <a id="cmd-terminal"></a> Terminal control and queries
  - <a id="cmd-enter-alt-screen"></a> `fn enter_alt_screen() -> Cmd`
  - <a id="cmd-exit-alt-screen"></a> `fn exit_alt_screen() -> Cmd`
  - <a id="cmd-hide-cursor"></a> `fn hide_cursor() -> Cmd`
  - <a id="cmd-show-cursor"></a> `fn show_cursor() -> Cmd`
  - <a id="cmd-clear-screen"></a> `fn clear_screen() -> Cmd`
  - <a id="cmd-window-size"></a> `fn window_size() -> Cmd`
  - <a id="cmd-set-window-title"></a> `fn set_window_title(title: String) -> Cmd`

- <a id="cmd-modes"></a> Mouse/focus/paste modes
  - <a id="cmd-enable-mouse-cell"></a> `fn enable_mouse_cell_motion() -> Cmd`
  - <a id="cmd-enable-mouse-all"></a> `fn enable_mouse_all_motion() -> Cmd`
  - <a id="cmd-disable-mouse"></a> `fn disable_mouse() -> Cmd`
  - <a id="cmd-enable-report-focus"></a> `fn enable_report_focus() -> Cmd`
  - <a id="cmd-disable-report-focus"></a> `fn disable_report_focus() -> Cmd`
  - <a id="cmd-enable-bracketed-paste"></a> `fn enable_bracketed_paste() -> Cmd`
  - <a id="cmd-disable-bracketed-paste"></a> `fn disable_bracketed_paste() -> Cmd`

- <a id="cmd-print"></a> Printing
  - <a id="cmd-println"></a> `fn println(s: String) -> Cmd`
  - <a id="cmd-printf"></a> `fn printf(s: String) -> Cmd`

### model
- Trait
  - `trait Model: Sized + Send + 'static {
       fn init() -> (Self, Option<Cmd>);
       fn update(&mut self, msg: Msg) -> Option<Cmd>;
       fn view(&self) -> String;
     }`

### <a id="sig-program"></a> program
- Types
  - `struct Program<M: Model> { /* runtime-managed */ }`
  - `struct ProgramBuilder<M: Model> { /* builder */ }`
  - `struct ProgramConfig { /* terminal/render/signal options */ }`
  - `enum MouseMotion { None, Cell, All }` (see `program.rs`)

#### <a id="programconfig-fields"></a> ProgramConfig fields (Rust) and defaults

- `alt_screen: bool` (default: `false`)
- `mouse_motion: MouseMotion` (default: `MouseMotion::None`)
- `report_focus: bool` (default: `false`)
- `fps: u32` (default: `60`)
- `without_renderer: bool` (default: `false`)
- `catch_panics: bool` (default: `true`)
- `signal_handler: bool` (default: `true`)
- `bracketed_paste: bool` (default: `false`)
- `output_writer: Option<Arc<Mutex<dyn AsyncWrite + Send + Unpin>>>` (default: `None`)
- `cancellation_token: Option<CancellationToken>` (default: `None`)
- `message_filter: Option<MessageFilter<M>>` (default: `None`)
- `input_source: Option<InputSource>` (default: `None`)
- `event_channel_buffer: Option<usize>` (default: `Some(1000)`)
- `memory_monitoring: bool` (default: `false`)

See also: [Go options sweep](#go-options-sweep-non-deprecated) for Go → Rust option equivalences.

#### <a id="programbuilder-setters"></a> ProgramBuilder setters (selected)

- `.alt_screen(bool)`
- `.mouse_motion(MouseMotion)`
- `.report_focus(bool)`
- `.with_fps(u32)`
- `.without_renderer()`
- `.catch_panics(bool)`
- `.signal_handler(bool)`
- `.bracketed_paste(bool)`
- `.input_tty()` / `.input(reader)`
- `.output(writer)`
- `.context(CancellationToken)`
- `.filter(|Msg| -> Option<Msg> { ... })`
- `.with_environment(std::collections::HashMap<String, String>)`

Go upstream references (for option names; version-dependent):
- WithAltScreen: https://github.com/charmbracelet/bubbletea/search?q=WithAltScreen
- WithMouseCellMotion / WithMouseAllMotion: https://github.com/charmbracelet/bubbletea/search?q=WithMouseCellMotion
- WithFPS: https://github.com/charmbracelet/bubbletea/search?q=WithFPS
- WithoutRenderer: https://github.com/charmbracelet/bubbletea/search?q=WithoutRenderer
- WithoutSignalHandler: https://github.com/charmbracelet/bubbletea/search?q=WithoutSignalHandler
- WithOutput: https://github.com/charmbracelet/bubbletea/search?q=WithOutput
- WithInput: https://github.com/charmbracelet/bubbletea/search?q=WithInput
- WithContext: https://github.com/charmbracelet/bubbletea/search?q=WithContext
- WithFilter: https://github.com/charmbracelet/bubbletea/search?q=WithFilter

See also:
- [ProgramConfig: Fields and Go Option Equivalents](#programconfig-fields-and-go-option-equivalents)
- [Appendix: Side-by-side Go vs Rust Examples](#appendix-side-by-side-go-vs-rust-examples)

### input
- `struct InputHandler { /* reads TTY/custom sources */ }`
- `enum InputSource { /* source variants */ }`

### terminal
- `trait TerminalInterface { /* async terminal ops */ }`
- `struct Terminal;`
- `struct DummyTerminal;`

### event (selected)
- `type Msg = Box<dyn Any + Send>`
- Key types: `KeyMsg`, `MouseMsg`, `WindowSizeMsg`, `FocusMsg`, `BlurMsg`
- Control: `QuitMsg`, `KillMsg`, `InterruptMsg`, `SuspendMsg`, `ResumeMsg`
- Terminal: `EnterAltScreenMsg`, `ExitAltScreenMsg`, `HideCursorMsg`, `ShowCursorMsg`, `ClearScreenMsg`, `SetWindowTitleMsg`, `EnableMouseCellMotionMsg`, `EnableMouseAllMotionMsg`, `DisableMouseMsg`, `EnableReportFocusMsg`, `DisableReportFocusMsg`, `EnableBracketedPasteMsg`, `DisableBracketedPasteMsg`, `RequestWindowSizeMsg`, `PrintMsg(String)`, `PrintfMsg(String)`
- Channels: `EventSender`, `EventReceiver`

### memory
- `struct MemoryMonitor { /* counters & peak memory */ }`
  - `fn new() -> Self`
  - `fn timer_added(&self)` / `fn timer_removed(&self)` / `fn get_active_timers(&self) -> u64`
  - `fn task_spawned(&self)` / `fn task_completed(&self)` / `fn get_active_tasks(&self) -> u64`
  - `fn set_channel_depth(&self, depth: u64)` / `fn get_channel_depth(&self) -> u64`
  - `fn message_processed(&self)` / `fn get_messages_processed(&self) -> u64`
  - `fn update_peak_memory(&self, bytes: u64)` / `fn get_peak_memory_bytes(&self) -> u64`
  - `fn snapshot(&self) -> MemorySnapshot`
  - `fn reset(&self)`
  - `fn check_health(&self) -> MemoryHealth`
- `struct MemorySnapshot { active_timers: u64, active_tasks: u64, channel_depth: u64, messages_processed: u64, peak_memory_bytes: u64 }`
- `struct MemoryHealth { is_healthy: bool, issues: Vec<String>, snapshot: MemorySnapshot }`

---

## Common Usage Examples

### Doctest: gradient::lerp_rgb
```rust
use bubbletea_rs::gradient::lerp_rgb;

// At t = 0.5 between black and white, we expect mid-gray (127,127,127) or (128,128,128)
let c = lerp_rgb((0,0,0), (255,255,255), 0.5);
assert!(c == (127,127,127) || c == (128,128,128));
```

### Ticking once and on an interval
```rust
use bubbletea_rs::{Cmd, Msg, tick, every};
use std::time::Duration;

// Single tick after 500ms
fn on_init_single_tick() -> Option<Cmd> {
    Some(tick(Duration::from_millis(500), |_d| Box::new("tick".to_string()) as Msg))
}

// Repeating message every 1s
fn on_init_every() -> Option<Cmd> {
    Some(every(Duration::from_secs(1), |_d| Box::new("heartbeat".to_string()) as Msg))
}
```

Rust (apply environment via ProgramBuilder::with_environment):

```rust
use std::collections::HashMap;
use std::process::Command;
use bubbletea::command::exec_process;
use bubbletea::Program;

#[derive(Debug)]
struct GotEnv(String);

fn configure_program_env() -> Program<MyModel> {
    let mut env = HashMap::new();
    env.insert("MY_VAR".to_string(), "hello".to_string());

    // Building the Program sets the environment for all exec_process commands
    Program::builder::<MyModel>()
        .with_environment(env)
        .without_renderer() // optional for headless/testing
        .build()
        .expect("program build")
}

fn read_var() -> Cmd {
    // On Unix-like systems, use `sh -c` to print an env var.
    // On Windows, use `cmd /C echo %MY_VAR%` (not shown here).
    exec_process(Command::new("sh").arg("-c").arg("printf %s \"$MY_VAR\""), |out| {
        match out {
            Ok(o) => Some(Box::new(GotEnv(String::from_utf8_lossy(&o.stdout).to_string())) as Msg),
            Err(e) => Some(Box::new(GotEnv(format!("ERR:{e}"))) as Msg),
        }
    })
}
```

### Creating and cancelling a repeating timer
```rust
use bubbletea_rs::{Cmd, Msg, every_with_id, cancel_timer};
use std::time::Duration;

fn start_timer() -> (Cmd, u64) {
    every_with_id(Duration::from_millis(200), |_d| Box::new("tick".to_string()) as Msg)
}

fn stop_timer(id: u64) -> Cmd {
    cancel_timer(id)
}
```

### Batch vs. sequence
```rust
use bubbletea_rs::{Cmd, batch, sequence, println};

fn run_batch(cmds: Vec<Cmd>) -> Cmd {
    batch(cmds)
}

fn run_sequence(cmds: Vec<Cmd>) -> Cmd {
    sequence(cmds)
}
```

### Terminal toggles and printing
```rust
use bubbletea_rs::{enter_alt_screen, exit_alt_screen, hide_cursor, show_cursor, clear_screen, set_window_title, println, printf};

fn enter_ui() -> Vec<Cmd> {
    vec![enter_alt_screen(), hide_cursor(), set_window_title("My App".to_string())]
}

fn exit_ui() -> Vec<Cmd> {
    vec![show_cursor(), clear_screen(), exit_alt_screen()]
}

fn print_examples() -> Vec<Cmd> {
    vec![println("Hello".to_string()), printf(format!("Value: {}", 42))]
}
```

### Running an external process
```rust
use bubbletea_rs::{exec_process, Msg};
use std::process::Command;

fn list_dir() -> Cmd {
    let mut cmd = Command::new("ls");
    exec_process(cmd, |res| match res {
        Ok(output) => Box::new(String::from_utf8_lossy(&output.stdout).to_string()) as Msg,
        Err(e) => Box::new(format!("exec error: {}", e)) as Msg,
    })
}
```

---

## Go → Rust Differences (Expanded)

- Update semantics
  - Go returns a new `Model` from `Update`. Rust mutates `&mut self` and only returns `Option<Cmd>`.

- Messages as type-erased payloads
  - Go uses interfaces; Rust uses `Box<dyn Any + Send>` for `Msg`. You downcast to your types when handling.

- Sequencing behavior
  - `sequence` currently emits a batched message containing all produced messages (`BatchMsgInternal`). Future refinements may change multi-message handling.

- Timers and cancellation
  - Rust provides `every_with_id` returning `(Cmd, u64)` for explicit cancellation via `cancel_timer`. Go patterns differ (commonly using contexts or channels).

- Extra utilities
  - Rust crate includes `gradient` utilities not present in Go core.

- Feature-gated logging
  - `log_to_file` is available behind the `logging` feature.

### Quit vs Kill semantics

- Go
  - `tea.Quit` triggers a graceful shutdown. `Program.Quit()` stops after allowing pending processing to wrap up.
  - `tea.Kill` (or `Program.Kill()`) triggers immediate termination.

- Rust (`bubbletea-rs`)
  - `quit()` command sends `QuitMsg`. The event loop marks `should_quit` and exits gracefully, returning `Ok(model)`.
  - `kill()` command and `Program::kill()` send `KillMsg`. The event loop stops as soon as possible (also from inside `BatchMsgInternal` and after the message filter), returning `Err(Error::ProgramKilled)`.
  - Terminal restoration and task cleanup still run on exit, ensuring the TTY is left in a sane state.

---

## Go → Rust API Reference Tables

The following tables map common Go Bubble Tea APIs to their `bubbletea-rs` equivalents, with brief notes on differences.

### Core Concepts

| Go (bubbletea) | Rust (bubbletea-rs) | Notes |
|---|---|---|
| `tea.Model` | `Model` trait | Rust mutates `&mut self` and returns `Option<Cmd>`; Go returns `(Model, Cmd)` from `Update`. |
| `tea.Msg` | `Msg = Box<dyn Any + Send>` | Rust messages are type-erased; downcast to your types when handling. |
| `tea.Cmd` | `Cmd = Pin<Box<dyn Future<Output = Option<Msg>> + Send>>` | Both represent deferred work that can emit a message. |
| `tea.Program` | `Program`, `ProgramBuilder` | Rust uses a builder and `ProgramConfig` for options. |

### Program Construction & Configuration

| Go (bubbletea) | Rust (bubbletea-rs) | Notes |
|---|---|---|
| `tea.NewProgram(m, opts...)` | `Program::<M>::builder()./* config */.build()?` | Builder pattern vs variadic options. |
| `tea.WithAltScreen()` | `enter_alt_screen()` command or config | Rust provides both runtime commands and config; behavior similar. |
| `tea.WithMouseAllMotion()` | `enable_mouse_all_motion()` / `MouseMotion::All` | Rust exposes explicit command and config enum. |
| `tea.WithMouseCellMotion()` | `enable_mouse_cell_motion()` / `MouseMotion::Cell` | Same as above. |
| `tea.WithoutSignalHandler()` | `ProgramConfig` equivalent flag | Configure via `ProgramConfig`; names may differ. |
| Other options | `ProgramConfig { ... }` | See `program.rs` for exact fields. |

### Commands (selected)

| Go (bubbletea) | Rust (bubbletea-rs) | Notes |
|---|---|---|
| `tea.Quit` | `quit()` | Initiate program shutdown. |
| `tea.Kill` / `Program.Kill()` | `kill()` / `Program::kill()` | Immediate termination; Rust returns `Err(Error::ProgramKilled)`. |
| (signal interrupt handling) | `interrupt()` | Rust convenience to signal external interruption; Go typically handles OS signals internally. |
| `tea.EnterAltScreen` | `enter_alt_screen()` | Switch to alternate screen. |
| `tea.ExitAltScreen` | `exit_alt_screen()` | Return from alternate screen. |
| `tea.HideCursor` | `hide_cursor()` | Hide terminal cursor. |
| `tea.ShowCursor` | `show_cursor()` | Show terminal cursor. |
| `tea.ClearScreen` | `clear_screen()` | Clear terminal display. |
| `tea.SetWindowTitle(title)` | `set_window_title(title: String)` | Update terminal/window title. |
| `tea.Println`, `tea.Printf` | `println(String)`, `printf(String)` | Rust variant takes already-formatted `String`. |
| `tea.EnableMouseAllMotion` | `enable_mouse_all_motion()` | Enable all mouse motion. |
| `tea.EnableMouseCellMotion` | `enable_mouse_cell_motion()` | Enable cell-based mouse motion. |
| `tea.DisableMouse` | `disable_mouse()` | Disable mouse reporting. |
| `tea.EnableReportFocus` | `enable_report_focus()` | Enable focus in/out messages. |
| `tea.DisableReportFocus` | `disable_report_focus()` | Disable focus messages. |
| `tea.EnableBracketedPaste` | `enable_bracketed_paste()` | Enable bracketed paste. |
| `tea.DisableBracketedPaste` | `disable_bracketed_paste()` | Disable bracketed paste. |
| `tea.Tick(d, fn)` | `tick(d, fn)` | Single delayed emission; closures differ by language. |
| Common interval patterns | `every(d, fn)`, `every_with_id(d, fn)` | Rust provides interval helpers and IDs for cancellation. |
| (cancel via context/pattern) | `cancel_timer(id)`, `cancel_all_timers()` | Rust exposes explicit cancellation commands. |
| (run external cmd via `exec.Cmd`) | `exec_process(std::process::Command, fn)` | Pass `std::process::Command` and a formatter closure. |

### Messages (selected)

| Go (bubbletea) | Rust (bubbletea-rs) | Notes |
|---|---|---|
| `tea.KeyMsg` | `KeyMsg` | Keyboard input. |
| `tea.MouseMsg` | `MouseMsg` | Requires enabling mouse reporting. |
| `tea.WindowSizeMsg` | `WindowSizeMsg` | Sent on resize or via `window_size()` request. |
| `tea.FocusMsg` / `tea.BlurMsg` | `FocusMsg` / `BlurMsg` | Terminal focus change events. |
| `tea.Println`/`Printf` (as commands) | `PrintMsg` / `PrintfMsg` | Rust separates message structs used by commands. |
| (quit via command) | `QuitMsg` (internal), `InterruptMsg` | In Rust, commands emit messages; names exposed for completeness. |

### Terminal & Input

| Go (bubbletea) | Rust (bubbletea-rs) | Notes |
|---|---|---|
| (internal terminal management) | `TerminalInterface`, `Terminal`, `DummyTerminal` | Go hides terminal details; Rust exposes a trait and implementations. |
| (internal input loop) | `InputHandler`, `InputSource` | Rust allows custom async input sources alongside TTY. |

### Errors, Memory, Logging, Gradient

| Go (bubbletea) | Rust (bubbletea-rs) | Notes |
|---|---|---|
| `error` from `Program.Run()` | `Result<_, Error>` | Rust uses a comprehensive `Error` enum. |
| (no direct equivalent) | `MemoryMonitor`, `MemorySnapshot`, `MemoryHealth` | Extra diagnostics utilities in Rust. |
| (not core) | `log_to_file` (feature) | Feature-gated logging helper in Rust. |
| (not core) | `gradient::*` | Gradient/ANSI utilities shipped with the Rust crate. |

---

## Migration from Go to Rust

This checklist highlights key differences and how to adapt common Bubble Tea patterns from Go to bubbletea-rs.

- Commands are async futures in Rust:
  - Go: functions returning `tea.Msg` run by the runtime
  - Rust: `Cmd = Pin<Box<dyn Future<Output = Option<Msg>> + Send>>`
  - Use helpers: [Timers](#cmd-timers), [Batch/Sequence](#cmd-batch), [Exec](#cmd-exec)

- Paste messages are not surfaced:
  - Go: `PasteMsg` (and discussions around `PasteStart`/`PasteEnd`)
  - Rust: no paste message emitted; `Event::Paste` is ignored even with bracketed paste enabled
  - Workaround: provide a [custom input source](#input) to capture paste text if needed

- Filtering messages:
  - Go: `WithFilter(func(Model, Msg) (Model, Cmd))`
  - Rust: `ProgramBuilder::filter(fn(Msg) -> Option<Msg>)` filters messages only (no model access)

- Timers and ticking patterns:
  - Go: `tea.Tick`, `tea.Every`
  - Rust: [`tick`](#cmd-timers), [`every`](#cmd-timers), plus `every_with_id`, `cancel_timer`

- Program methods differences:
  - `Program::wait(&self)`: async no-op placeholder; not the same semantics as Go’s `Wait`
  - `quit()`, `interrupt()`, `suspend()`: use command helpers to emit control messages

- Terminal modes and rendering:
  - Go options → Rust builder methods: see [ProgramConfig fields](#programconfig-fields) and [builder setters](#programbuilder-setters)
  - Many modes can also be toggled at runtime via [terminal commands](#cmd-terminal) and [modes](#cmd-modes)

- Printing and window title:
  - Go: `tea.Println`, `tea.Printf`, `tea.SetWindowTitle`
  - Rust: [`println`, `printf`](#cmd-print), [`set_window_title`](#cmd-terminal)

- Messages and downcasting:
  - Go: `type switch` on `tea.Msg`
  - Rust: `Msg` is `Box<dyn Any + Send>`; use `msg.is::<T>()` or `msg.downcast_ref::<T>()`

- Extras in Rust:
  - Memory monitor utilities (`memory` module)
  - Feature-gated `log_to_file`; enable the `logging` feature if needed

---

## Appendix: Side-by-side Go vs Rust Examples

Below are concise examples of common tasks in Go Bubble Tea and their Rust equivalents.

### Quit the program

Go (bubbletea):
```go
import tea "github.com/charmbracelet/bubbletea"

type model struct{}

func (model) Init() tea.Cmd { return nil }

func (model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
    switch msg.(type) {
    case tea.KeyMsg:
        return model{}, tea.Quit
    }
    return model{}, nil
}

func (model) View() string { return "bye" }

func main() {
    if err := tea.NewProgram(model{}).Start(); err != nil { panic(err) }
}
```

Rust (bubbletea-rs):
```rust
use bubbletea_rs::{Model, Program, Msg, Cmd, quit};

struct M;

impl Model for M {
    fn init() -> (Self, Option<Cmd>) { (Self, None) }
    fn update(&mut self, _msg: Msg) -> Option<Cmd> { Some(quit()) }
    fn view(&self) -> String { "bye".into() }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Program::<M>::builder().build()?.run().await?;
    Ok(())
}
```

### Enter/Exit alternate screen and hide/show cursor

Go (bubbletea):
```go
import tea "github.com/charmbracelet/bubbletea"

func main() {
    p := tea.NewProgram(model{}, tea.WithAltScreen())
    if err := p.Start(); err != nil { panic(err) }
}
```

Rust (bubbletea-rs) using commands:
```rust
use bubbletea_rs::{enter_alt_screen, exit_alt_screen, hide_cursor, show_cursor, Cmd};

fn enter_ui() -> Vec<Cmd> { vec![enter_alt_screen(), hide_cursor()] }
fn exit_ui() -> Vec<Cmd> { vec![show_cursor(), exit_alt_screen()] }
```

Rust (bubbletea-rs) via builder config:
```rust
use bubbletea_rs::{Program, Model};

struct M; impl Model for M {
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) { (Self, None) }
    fn update(&mut self, _msg: bubbletea_rs::Msg) -> Option<bubbletea_rs::Cmd> { None }
    fn view(&self) -> String { String::new() }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<M>::builder()
        .alt_screen(true)
        .build()?;
    program.run().await?;
    Ok(())
}
```

### Ticking and repeating intervals

Go (bubbletea):
```go
import (
  tea "github.com/charmbracelet/bubbletea"
  "time"
)

func (model) Init() tea.Cmd {
    return tea.Tick(time.Second, func(t time.Time) tea.Msg { return "tick" })
}

func (model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
    switch msg.(type) {
    case string: // "tick"
        return model{}, tea.Tick(time.Second, func(time.Time) tea.Msg { return "tick" })
    }
    return model{}, nil
}
```

Rust (bubbletea-rs):
```rust
use bubbletea_rs::{Cmd, Msg, tick, every};
use std::time::Duration;

fn init_once() -> Option<Cmd> {
    Some(tick(Duration::from_secs(1), |_d| Box::new("tick".to_string()) as Msg))
}

fn on_tick_every() -> Option<Cmd> {
    Some(every(Duration::from_secs(1), |_d| Box::new("tick".to_string()) as Msg))
}
```

---

## ProgramConfig: Fields and Go Option Equivalents

Exact fields exposed by `ProgramConfig` and their closest Go Bubble Tea options. Field names are authoritative on the Rust side; Go option names are indicative (confirm against the Go Bubble Tea docs for your version).

| Rust `ProgramConfig` field | Type | Closest Go option(s) | Notes |
|---|---|---|---|
| `alt_screen` | `bool` | `tea.WithAltScreen()` | Use alt screen buffer. Also available via `enter_alt_screen()` command in Rust. |
| `mouse_motion` | `MouseMotion` (`None`/`Cell`/`All`) | `tea.WithMouseCellMotion()`, `tea.WithMouseAllMotion()` | Controls reporting granularity. Commands also available in Rust. |
| `report_focus` | `bool` | (option enabling focus reporting; may vary) | Enables `FocusMsg`/`BlurMsg`. Commands also available. |
| `fps` | `u32` | `tea.WithFPS()` (version-dependent) | Target render FPS in Rust. |
| `without_renderer` | `bool` | `tea.WithoutRenderer()` (version-dependent) | Disable renderer for headless/testing. |
| `catch_panics` | `bool` | (no direct) | Rust-only: convert panics to errors where possible. |
| `signal_handler` | `bool` | `tea.WithoutSignalHandler()` (inverse) | Enable/disable internal signal handling. |
| `bracketed_paste` | `bool` | Command: `tea.EnableBracketedPaste` | Command equivalents exist in Rust too. |
| `output_writer` | `Option<Arc<Mutex<dyn AsyncWrite + ...>>>` | `tea.WithOutput(io.Writer)` | Custom output sink. |
| `cancellation_token` | `Option<CancellationToken>` | `tea.WithContext(ctx)` (version-dependent) | External cooperative cancellation. |
| `message_filter` | `Option<Box<dyn Fn(Msg) -> Option<Msg> + Send>>` | `tea.WithFilter(fn)` (semantics differ) | Rust filter operates on `Msg`; return `None` to drop. |
| `input_source` | `Option<InputSource>` | `tea.WithInput(io.Reader)` / `tea.WithInputTTY()` | Custom async input instead of TTY. |
| `event_channel_buffer` | `Option<usize>` | (no direct) | Configure internal event channel capacity. |
| `memory_monitoring` | `bool` | (no direct) | Enable internal memory usage monitoring helpers. |

Builder methods that set these:

- `.alt_screen(bool)`
- `.mouse_motion(MouseMotion)`
- `.report_focus(bool)`
- `.with_fps(u32)`
- `.without_renderer()`
- `.catch_panics(bool)`
- `.signal_handler(bool)`
- `.bracketed_paste(bool)`
- `.input_tty()` / `.input(reader)`
- `.output(writer)`
- `.context(CancellationToken)`
- `.filter(|Msg| -> Option<Msg> { ... })`

---

## Additional Go → Rust Mappings (from Go Index, non-deprecated)

### Program methods

| Go | Rust | Notes |
|---|---|---|
| `(*Program).Run()` | `Program::run(self) -> Result<M, Error>` | Returns final model or error. |
| `(*Program).Send(msg)` | `Program::send(&self, Msg)` / `Program::sender()` | Send/inject messages from outside. |
| `(*Program).Quit()` | `Program::quit(&self)` | Convenience to request shutdown. |
| `(*Program).Kill()` | `Program::kill(&self)` | Forceful shutdown; maps to sending `QuitMsg` currently. |
| `(*Program).Println(...)` | `Program::println(&mut self, String)` | Unmanaged output, like Go. |
| `(*Program).Printf(...)` | `Program::printf(&mut self, String)` | Unmanaged formatted output. |
| `(*Program).ReleaseTerminal()` | `Program::release_terminal(&mut self)` | Restore original terminal state. |
| `(*Program).RestoreTerminal()` | `Program::restore_terminal(&mut self)` | Re-acquire terminal per config. |
 | `(*Program).Wait()` | `Program::wait(&self)` | Async no-op placeholder in Rust; see docs. |

 Notes:
 - See: [Detailed Signatures → program](#sig-program) for type definitions and builder methods.

### Commands (completing coverage)

| Go | Rust | Notes |
|---|---|---|
| `tea.WindowSize()` | [`window_size()`](#cmd-window-size) | Requests current terminal size; yields `WindowSizeMsg`. |
| `tea.ClearScreen()` | [`clear_screen()`](#cmd-clear-screen) | Clear terminal before next update. |
| `tea.SetWindowTitle(title)` | [`set_window_title(title)`](#cmd-set-window-title) | Set terminal/window title. |
| `tea.EnableBracketedPaste()` | [`enable_bracketed_paste()`](#cmd-enable-bracketed-paste) | Enable bracketed paste. |
| `tea.DisableBracketedPaste()` | [`disable_bracketed_paste()`](#cmd-disable-bracketed-paste) | Disable bracketed paste. |
| `tea.EnableReportFocus()` | [`enable_report_focus()`](#cmd-enable-report-focus) | Enable focus events. |
| `tea.DisableReportFocus()` | [`disable_report_focus()`](#cmd-disable-report-focus) | Disable focus events. |
| `tea.EnableMouseAllMotion()` | [`enable_mouse_all_motion()`](#cmd-enable-mouse-all) | See `MouseMotion::All`. |
| `tea.EnableMouseCellMotion()` | [`enable_mouse_cell_motion()`](#cmd-enable-mouse-cell) | See `MouseMotion::Cell`. |
| `tea.DisableMouse()` | [`disable_mouse()`](#cmd-disable-mouse) | Disable mouse reporting. |
| `tea.EnterAltScreen()` | [`enter_alt_screen()`](#cmd-enter-alt-screen) | Enter alt screen. |
| `tea.ExitAltScreen()` | [`exit_alt_screen()`](#cmd-exit-alt-screen) | Exit alt screen. |
| `tea.HideCursor()` | [`hide_cursor()`](#cmd-hide-cursor) | Hide cursor. |
| `tea.ShowCursor()` | [`show_cursor()`](#cmd-show-cursor) | Show cursor. |
| `tea.Interrupt()` | [`interrupt()`](#cmd-interrupt) | Signal external interruption. |
| `tea.Quit()` | [`quit()`](#cmd-quit) | Graceful shutdown. |
| `tea.Suspend()` | [`suspend()`](#cmd-suspend) | Suspend program (release terminal). |
| `tea.Println`, `tea.Printf` | [`println(String)`](#cmd-println), [`printf(String)`](#cmd-printf) | Unmanaged output via command helpers. |
| `tea.Tick(dur, fn)` | [`tick(Duration, fn)`](#cmd-tick) | One-shot timer; callback receives `Duration`. |
| `tea.Every(dur, fn)` | [`every(Duration, fn)`](#cmd-every) | Repeating timer; callback receives `Duration`. |
| `tea.Batch(cmds...)` | [`batch(Vec<Cmd>)`](#cmd-batch) | Run commands concurrently; returns when all complete. |
| `tea.Sequence(cmds...)` | [`sequence(Vec<Cmd>)`](#cmd-sequence) | Run commands in order; each waits for previous. |

 Notes:
 - Go `Cmd.Exec` vs Rust: there’s no direct `exec` that blocks the program; use `exec_process(Command, cb)` or spawn tasks.
 - See: [Detailed Signatures → command](#sig-command) for Rust function signatures and constraints.
 - Quick links: [Timers](#cmd-timers), [Batch/Sequence](#cmd-batch), [Terminal controls](#cmd-terminal), [Modes (mouse/focus/paste)](#cmd-modes), [Printing](#cmd-print), [Exec process](#cmd-exec)

### Messages (selected)

| Go (bubbletea) | Rust (bubbletea-rs) | Notes |
|---|---|---|
| `tea.KeyMsg` | `KeyMsg` | Keyboard input. |
| `tea.MouseMsg` | `MouseMsg` | Requires enabling mouse reporting. |
| `tea.WindowSizeMsg` | `WindowSizeMsg` | Sent on resize or via `window_size()` request. |
| `tea.FocusMsg` / `tea.BlurMsg` | `FocusMsg` / `BlurMsg` | Terminal focus change events. |
| `tea.Println`/`Printf` (as commands) | `PrintMsg` / `PrintfMsg` | Rust separates message structs used by commands. |
| (quit via command) | `QuitMsg` (internal), `InterruptMsg` | In Rust, commands emit messages; names exposed for completeness. |

### Terminal & Input

| Go (bubbletea) | Rust (bubbletea-rs) | Notes |
|---|---|---|
| (internal terminal management) | `TerminalInterface`, `Terminal`, `DummyTerminal` | Go hides terminal details; Rust exposes a trait and implementations. |
| (internal input loop) | `InputHandler`, `InputSource` | Rust allows custom async input sources alongside TTY. |

### Errors, Memory, Logging, Gradient

| Go (bubbletea) | Rust (bubbletea-rs) | Notes |
|---|---|---|
| `error` from `Program.Run()` | `Result<_, Error>` | Rust uses a comprehensive `Error` enum. |
| (no direct equivalent) | `MemoryMonitor`, `MemorySnapshot`, `MemoryHealth` | Extra diagnostics utilities in Rust. |
| (not core) | `log_to_file` (feature) | Feature-gated logging helper in Rust. |
| (not core) | `gradient::*` | Gradient/ANSI utilities shipped with the Rust crate. |

---

## Appendix: Side-by-side Go vs Rust Examples

Below are concise examples of common tasks in Go Bubble Tea and their Rust equivalents.

### Quit the program

Go (bubbletea):
```go
import tea "github.com/charmbracelet/bubbletea"

type model struct{}

func (model) Init() tea.Cmd { return nil }

func (model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
    switch msg.(type) {
    case tea.KeyMsg:
        return model{}, tea.Quit
    }
    return model{}, nil
}

func (model) View() string { return "bye" }

func main() {
    if err := tea.NewProgram(model{}).Start(); err != nil { panic(err) }
}
```

Rust (bubbletea-rs):
```rust
use bubbletea_rs::{Model, Program, Msg, Cmd, quit};

struct M;

impl Model for M {
    fn init() -> (Self, Option<Cmd>) { (Self, None) }
    fn update(&mut self, _msg: Msg) -> Option<Cmd> { Some(quit()) }
    fn view(&self) -> String { "bye".into() }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Program::<M>::builder().build()?.run().await?;
    Ok(())
}
```

### Enter/Exit alternate screen and hide/show cursor

Go (bubbletea):
```go
import tea "github.com/charmbracelet/bubbletea"

func main() {
    p := tea.NewProgram(model{}, tea.WithAltScreen())
    if err := p.Start(); err != nil { panic(err) }
}
```

Rust (bubbletea-rs) using commands:
```rust
use bubbletea_rs::{enter_alt_screen, exit_alt_screen, hide_cursor, show_cursor, Cmd};

fn enter_ui() -> Vec<Cmd> { vec![enter_alt_screen(), hide_cursor()] }
fn exit_ui() -> Vec<Cmd> { vec![show_cursor(), exit_alt_screen()] }
```

Rust (bubbletea-rs) via builder config:
```rust
use bubbletea_rs::{Program, Model};

struct M; impl Model for M {
    fn init() -> (Self, Option<bubbletea_rs::Cmd>) { (Self, None) }
    fn update(&mut self, _msg: bubbletea_rs::Msg) -> Option<bubbletea_rs::Cmd> { None }
    fn view(&self) -> String { String::new() }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<M>::builder()
        .alt_screen(true)
        .build()?;
    program.run().await?;
    Ok(())
}
```

### Ticking and repeating intervals

Go (bubbletea):
```go
import (
  tea "github.com/charmbracelet/bubbletea"
  "time"
)

func (model) Init() tea.Cmd {
    return tea.Tick(time.Second, func(t time.Time) tea.Msg { return "tick" })
}

func (model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
    switch msg.(type) {
    case string: // "tick"
        return model{}, tea.Tick(time.Second, func(time.Time) tea.Msg { return "tick" })
    }
    return model{}, nil
}
```

Rust (bubbletea-rs):
```rust
use bubbletea_rs::{Cmd, Msg, tick, every};
use std::time::Duration;

fn init_once() -> Option<Cmd> {
    Some(tick(Duration::from_secs(1), |_d| Box::new("tick".to_string()) as Msg))
}

fn on_tick_every() -> Option<Cmd> {
    Some(every(Duration::from_secs(1), |_d| Box::new("tick".to_string()) as Msg))
}
```

### Window resize handling

Go (Bubble Tea):

```go
func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
    switch msg := msg.(type) {
    case tea.WindowSizeMsg:
        m.w = msg.Width
        m.h = msg.Height
    }
    return m, nil
}
```

Rust (bubbletea-rs):

```rust
use bubbletea::{prelude::*, command::*};

struct Model { w: u16, h: u16 }

impl ModelTrait for Model {
    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(ws) = msg.downcast_ref::<WindowSizeMsg>() {
            self.w = ws.width;
            self.h = ws.height;
        }
        None
    }
}
```

### Focus/Blur events

Go:

```go
switch msg := msg.(type) {
case tea.FocusMsg:
    m.focused = true
case tea.BlurMsg:
    m.focused = false
}
```

Rust:

```rust
if msg.is::<FocusMsg>() { m.focused = true; }
if msg.is::<BlurMsg>() { m.focused = false; }
```

### Mouse events (basic pattern)

Go:

```go
case tea.MouseMsg:
    // inspect msg.Action, msg.X, msg.Y as needed
```

Rust:

```rust
if let Some(mouse) = msg.downcast_ref::<MouseMsg>() {
    // inspect fields on `mouse` as needed (e.g., position/buttons)
}
```

### Running an external process and emitting a message

Go (conceptual `Cmd` pattern):

```go
func fetch() tea.Cmd {
    return func() tea.Msg { return gotData("ok") }
}
```

Rust (`exec_process` minimal):

```rust
use std::process::Command;
use bubbletea::command::exec_process;

#[derive(Debug)]
struct GotData(String);

fn fetch() -> Cmd {
    exec_process(Command::new("echo").arg("ok"), |out| {
        Some(Box::new(GotData(String::from_utf8_lossy(&out.stdout).trim().to_string())) as Msg)
    })
}
```

Note: prefer batching/sequence helpers if you need to mix process output with other commands.

### Bracketed paste: enable/disable

Go (enable/disable in Init/Update):

```go
func (m model) Init() tea.Cmd {
    return tea.Batch(
        tea.EnableBracketedPaste(),
        tea.EnableReportFocus(),
    )
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
    switch msg := msg.(type) {
    case tea.FocusMsg:
        m.focused = true
    case tea.BlurMsg:
        m.focused = false
    }
    return m, nil
}
```

Rust (enable/disable in init/update):

```rust
use bubbletea::command::*;

fn init() -> Cmd {
    batch(vec![
        enable_bracketed_paste(),
        enable_report_focus(),
    ])
}

impl ModelTrait for Model {
    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if msg.is::<FocusMsg>() { self.focused = true; }
        if msg.is::<BlurMsg>()  { self.focused = false; }
        None
    }
}
```

Notes:
- Enabling bracketed paste changes how terminal sends paste; bubbletea-rs does not currently emit a dedicated PasteMsg. Handle pasted content via input events as appropriate in your app.

### Focus reporting: toggle at runtime

Go:

```go
var reporting bool

func toggleFocus() tea.Cmd {
    if reporting {
        return tea.DisableReportFocus()
    }
    return tea.EnableReportFocus()
}
```

Rust:

```rust
fn toggle_focus_reporting(reporting: bool) -> Cmd {
    if reporting { disable_report_focus() } else { enable_report_focus() }
}
```

### End-to-end: batch a timer + external process

Go (conceptual):

```go
func Init() tea.Cmd {
    return tea.Batch(
        tea.Tick(time.Second, func(time.Time) tea.Msg { return ticked{} }),
        func() tea.Msg { return gotData("ok") },
    )
}
```

Rust:

```rust
use std::process::Command;
use bubbletea::command::*;

#[derive(Debug)]
struct Ticked;
#[derive(Debug)]
struct GotData(String);

fn init() -> Cmd {
    batch(vec![
        tick(std::time::Duration::from_secs(1), || Some(Box::new(Ticked) as Msg)),
        exec_process(Command::new("echo").arg("ok"), |out| {
            Some(Box::new(GotData(String::from_utf8_lossy(&out.stdout).trim().to_string())) as Msg)
        }),
    ])
}

impl ModelTrait for Model {
    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if msg.is::<Ticked>() {
            self.ticks += 1;
        } else if let Some(g) = msg.downcast_ref::<GotData>() {
            self.data = g.0.clone();
        }
        None
    }
}
```

Tip: use `sequence(vec![...])` if later commands should run only after earlier ones complete.

---

## Go options sweep (non-deprecated)

- WithInput/WithOutput → Rust `ProgramBuilder`/`ProgramConfig` or runtime `.input(...)`/`.output(...)` (covered above).
- WithAltScreen → Rust `enter_alt_screen()`/`exit_alt_screen()` commands; can be invoked at startup via `batch([...])`.
- WithMouseCellMotion/AllMotion/DisableMouse → Rust `enable_mouse_cell_motion()`/`enable_mouse_all_motion()`/`disable_mouse()`.
- WithReportFocus → Rust `enable_report_focus()`/`disable_report_focus()`.
- WithBracketedPaste → Rust `enable_bracketed_paste()`/`disable_bracketed_paste()`.
- WithFilter → Rust `.filter(|Msg| -> Option<Msg>)` on `ProgramBuilder` (semantics differ; filters only on `Msg`).
- WithEnvironment → Rust `.with_environment(HashMap<String, String>)` on `ProgramBuilder`.

---

## Notes on Program::wait

`Program::wait(&self)` in Rust is a lightweight await point used to coordinate with the async runtime; it does not block the UI loop like a join. Prefer driving your app via `Program::run()` and use commands/tasks for long-running work.
