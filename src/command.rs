//! This module provides functions for creating and managing commands.
//! Commands are asynchronous operations that can produce messages to update the model.

use crate::event::{
    next_timer_id, BatchCmdMsg, ClearScreenMsg, DisableBracketedPasteMsg, DisableMouseMsg,
    DisableReportFocusMsg, EnableBracketedPasteMsg, EnableMouseAllMotionMsg,
    EnableMouseCellMotionMsg, EnableReportFocusMsg, EnterAltScreenMsg, ExitAltScreenMsg,
    HideCursorMsg, InterruptMsg, KillMsg, Msg, PrintMsg, PrintfMsg, QuitMsg, RequestWindowSizeMsg,
    ShowCursorMsg, SuspendMsg,
};
use std::future::Future;
use std::pin::Pin;
use std::process::Command as StdCommand;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::process::Command as TokioCommand;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;

/// A command represents an asynchronous operation that may produce a message.
///
/// Commands are typically created by the `init` and `update` methods of your
/// `Model` and are then executed by the `Program`'s event loop.
///
/// The `Cmd` type is a `Pin<Box<dyn Future<Output = Option<Msg>> + Send>>`,
/// which means it's a boxed, pinned future that returns an `Option<Msg>`.
/// If the command produces a message, it will be sent back to the `Program`
/// to be processed by the `update` method.
pub type Cmd = Pin<Box<dyn Future<Output = Option<Msg>> + Send>>;

/// A batch command that executes multiple commands concurrently.
///
/// This struct is used internally by the `batch` function to group multiple
/// commands together for concurrent execution.
#[allow(dead_code)]
pub struct Batch {
    commands: Vec<Cmd>,
}

#[allow(dead_code)]
impl Batch {
    /// Creates a new `Batch` from a vector of `Cmd`s.
    pub(crate) fn new(commands: Vec<Cmd>) -> Self {
        Self { commands }
    }

    /// Consumes the `Batch` and returns the inner vector of `Cmd`s.
    pub(crate) fn into_commands(self) -> Vec<Cmd> {
        self.commands
    }
}

/// Global environment variables to be applied to external process commands.
///
/// Set by `Program::new()` from `ProgramConfig.environment` and read by
/// `exec_process` when spawning commands. If unset, no variables are injected.
pub static COMMAND_ENV: OnceLock<std::collections::HashMap<String, String>> = OnceLock::new();

/// Creates a command that quits the application.
///
/// This command sends a `QuitMsg` to the program, which will initiate the
/// shutdown process.
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg, KeyMsg};
/// use crossterm::event::KeyCode;
///
/// struct MyModel;
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         (Self {}, None)
///     }
///
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         // Quit when 'q' is pressed
///         if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
///             if key_msg.key == KeyCode::Char('q') {
///                 return Some(command::quit());
///             }
///         }
///         None
///     }
///     
///     fn view(&self) -> String {
///         "Press 'q' to quit".to_string()
///     }
/// }
/// ```
pub fn quit() -> Cmd {
    Box::pin(async { Some(Box::new(QuitMsg) as Msg) })
}

/// Creates a command that kills the application immediately.
///
/// This command sends a `KillMsg` to the program, which will cause the event loop
/// to terminate as soon as possible with `Error::ProgramKilled`.
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg};
///
/// struct MyModel {
///     has_error: bool,
/// }
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         (Self { has_error: false }, None)
///     }
///
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         // Force kill on critical error
///         if self.has_error {
///             return Some(command::kill());
///         }
///         None
///     }
///     
///     fn view(&self) -> String {
///         "Running...".to_string()
///     }
/// }
/// ```
pub fn kill() -> Cmd {
    Box::pin(async { Some(Box::new(KillMsg) as Msg) })
}

/// Creates a command that interrupts the application.
///
/// This command sends an `InterruptMsg` to the program, typically used
/// to signal an external interruption (e.g., Ctrl+C).
pub fn interrupt() -> Cmd {
    Box::pin(async { Some(Box::new(InterruptMsg) as Msg) })
}

/// Creates a command that suspends the application.
///
/// This command sends a `SuspendMsg` to the program, which can be used
/// to temporarily pause the application and release terminal control.
pub fn suspend() -> Cmd {
    Box::pin(async { Some(Box::new(SuspendMsg) as Msg) })
}

/// Creates a command that executes a batch of commands concurrently.
///
/// The commands in the batch will be executed in parallel immediately when
/// this command is processed by the program. This is a non-blocking operation
/// that spawns each command in its own task, allowing for smooth animations
/// and responsive user interfaces.
///
/// # Arguments
///
/// * `cmds` - A vector of commands to execute concurrently
///
/// # Returns
///
/// A command that immediately dispatches all provided commands for concurrent execution
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg};
/// use std::time::Duration;
///
/// struct MyModel;
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         let model = Self {};
///         // Execute multiple operations concurrently
///         let cmd = command::batch(vec![
///             command::window_size(),  // Get window dimensions
///             command::tick(Duration::from_secs(1), |_| {
///                 Box::new("InitialTickMsg") as Msg
///             }),
///             command::hide_cursor(),  // Hide the cursor
///         ]);
///         (model, Some(cmd))
///     }
///     
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         None
///     }
///     
///     fn view(&self) -> String {
///         "Loading...".to_string()
///     }
/// }
/// ```
pub fn batch(cmds: Vec<Cmd>) -> Cmd {
    Box::pin(async move {
        // Don't wait for commands - just wrap them for immediate spawning
        Some(Box::new(BatchCmdMsg(cmds)) as Msg)
    })
}

/// Creates a command that executes a sequence of commands sequentially.
///
/// The commands in the sequence will be executed one after another in order.
/// All messages produced by the commands will be collected and returned.
/// This is useful when you need to perform operations that depend on the
/// completion of previous operations.
///
/// # Arguments
///
/// * `cmds` - A vector of commands to execute sequentially
///
/// # Returns
///
/// A command that executes all provided commands in sequence
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg};
///
/// struct MyModel;
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         let model = Self {};
///         // Execute operations in order
///         let cmd = command::sequence(vec![
///             command::enter_alt_screen(),     // First, enter alt screen
///             command::clear_screen(),         // Then clear it
///             command::hide_cursor(),          // Finally hide the cursor
///         ]);
///         (model, Some(cmd))
///     }
///     
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         None
///     }
///     
///     fn view(&self) -> String {
///         "Ready".to_string()
///     }
/// }
/// ```
pub fn sequence(cmds: Vec<Cmd>) -> Cmd {
    Box::pin(async move {
        let mut results = Vec::new();
        for cmd in cmds {
            if let Some(msg) = cmd.await {
                results.push(msg);
            }
        }
        if results.is_empty() {
            None
        } else {
            Some(Box::new(crate::event::BatchMsgInternal { messages: results }) as Msg)
        }
    })
}

/// Creates a command that produces a single message after a delay.
///
/// This command will send a message produced by the provided closure `f`
/// after the specified `duration`. Unlike `every()`, this produces only
/// one message and then completes. It's commonly used for one-shot timers
/// that can be re-armed in the update method.
///
/// Note: Due to tokio's interval implementation, the first tick is consumed
/// to ensure the message is sent after a full duration, not immediately.
///
/// # Arguments
///
/// * `duration` - The duration to wait before sending the message
/// * `f` - A closure that takes a `Duration` and returns a `Msg`
///
/// # Returns
///
/// A command that will produce a single message after the specified duration
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg};
/// use std::time::Duration;
///
/// #[derive(Debug)]
/// struct TickMsg;
///
/// struct MyModel {
///     counter: u32,
/// }
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         let model = Self { counter: 0 };
///         // Start a timer that fires after 1 second
///         let cmd = command::tick(Duration::from_secs(1), |_| {
///             Box::new(TickMsg) as Msg
///         });
///         (model, Some(cmd))
///     }
///
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         if msg.downcast_ref::<TickMsg>().is_some() {
///             self.counter += 1;
///             // Re-arm the timer for another tick
///             return Some(command::tick(Duration::from_secs(1), |_| {
///                 Box::new(TickMsg) as Msg
///             }));
///         }
///         None
///     }
///     
///     fn view(&self) -> String {
///         format!("Counter: {}", self.counter)
///     }
/// }
/// ```
pub fn tick<F>(duration: Duration, f: F) -> Cmd
where
    F: Fn(Duration) -> Msg + Send + 'static,
{
    Box::pin(async move {
        let mut ticker = interval(duration);
        // The first tick completes immediately; advance once to move to the start
        ticker.tick().await; // consume the immediate tick
                             // Now wait for one full duration before emitting
        ticker.tick().await;
        Some(f(duration))
    })
}

/// Creates a command that produces messages repeatedly at a regular interval.
///
/// This command will continuously send messages produced by the provided closure `f`
/// after every `duration` until the program exits or the timer is cancelled.
/// Unlike `tick()`, this creates a persistent timer that keeps firing.
///
/// Warning: Be careful not to call `every()` repeatedly for the same timer,
/// as this will create multiple concurrent timers that can overwhelm the
/// event loop. Instead, call it once and use `cancel_timer()` if needed.
///
/// # Arguments
///
/// * `duration` - The duration between messages
/// * `f` - A closure that takes a `Duration` and returns a `Msg`
///
/// # Returns
///
/// A command that will produce messages repeatedly at the specified interval
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg};
/// use std::time::Duration;
///
/// #[derive(Debug)]
/// struct ClockTickMsg;
///
/// struct MyModel {
///     time_elapsed: Duration,
/// }
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         let model = Self { time_elapsed: Duration::from_secs(0) };
///         // Start a timer that fires every second
///         let cmd = command::every(Duration::from_secs(1), |_| {
///             Box::new(ClockTickMsg) as Msg
///         });
///         (model, Some(cmd))
///     }
///
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         if msg.downcast_ref::<ClockTickMsg>().is_some() {
///             self.time_elapsed += Duration::from_secs(1);
///             // No need to re-arm - it keeps firing automatically
///         }
///         None
///     }
///     
///     fn view(&self) -> String {
///         format!("Time elapsed: {:?}", self.time_elapsed)
///     }
/// }
/// ```
pub fn every<F>(duration: Duration, f: F) -> Cmd
where
    F: Fn(Duration) -> Msg + Send + 'static,
{
    let timer_id = next_timer_id();
    let cancellation_token = CancellationToken::new();

    Box::pin(async move {
        Some(Box::new(crate::event::EveryMsgInternal {
            duration,
            func: Box::new(f),
            cancellation_token,
            timer_id,
        }) as Msg)
    })
}

/// Creates a command that produces messages repeatedly at a regular interval with cancellation support.
///
/// This command will continuously send messages produced by the provided closure `f`
/// after every `duration` until the program exits or the timer is cancelled.
/// The returned timer ID can be used with `cancel_timer()` to stop the timer.
///
/// # Arguments
///
/// * `duration` - The duration between messages
/// * `f` - A closure that takes a `Duration` and returns a `Msg`
///
/// # Returns
///
/// Returns a tuple containing:
/// - The command to start the timer
/// - A timer ID that can be used with `cancel_timer()`
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg};
/// use std::time::Duration;
///
/// #[derive(Debug)]
/// struct AnimationFrameMsg;
///
/// #[derive(Debug)]
/// struct StartAnimationMsg(u64); // Contains timer ID
///
/// struct MyModel {
///     animation_timer_id: Option<u64>,
///     is_animating: bool,
/// }
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         let model = Self {
///             animation_timer_id: None,
///             is_animating: false,
///         };
///         // Start animation timer and get its ID
///         let (cmd, timer_id) = command::every_with_id(
///             Duration::from_millis(16), // ~60 FPS
///             |_| Box::new(AnimationFrameMsg) as Msg
///         );
///         // Send a message with the timer ID so we can store it
///         let batch = command::batch(vec![
///             cmd,
///             Box::pin(async move {
///                 Some(Box::new(StartAnimationMsg(timer_id)) as Msg)
///             }),
///         ]);
///         (model, Some(batch))
///     }
///
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         if let Some(start_msg) = msg.downcast_ref::<StartAnimationMsg>() {
///             self.animation_timer_id = Some(start_msg.0);
///             self.is_animating = true;
///         }
///         None
///     }
///     
///     fn view(&self) -> String {
///         if self.is_animating {
///             "Animating...".to_string()
///         } else {
///             "Stopped".to_string()
///         }
///     }
/// }
/// ```
pub fn every_with_id<F>(duration: Duration, f: F) -> (Cmd, u64)
where
    F: Fn(Duration) -> Msg + Send + 'static,
{
    let timer_id = next_timer_id();
    let cancellation_token = CancellationToken::new();

    let cmd = Box::pin(async move {
        Some(Box::new(crate::event::EveryMsgInternal {
            duration,
            func: Box::new(f),
            cancellation_token,
            timer_id,
        }) as Msg)
    });

    (cmd, timer_id)
}

/// Creates a command that executes an external process.
///
/// This command spawns an external process asynchronously and returns a message
/// produced by the provided closure with the process's output. The process runs
/// in the background and doesn't block the UI.
///
/// # Arguments
///
/// * `cmd` - The `std::process::Command` to execute
/// * `f` - A closure that processes the command output and returns a `Msg`
///
/// # Returns
///
/// A command that executes the external process
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg};
/// use std::process::Command;
///
/// #[derive(Debug)]
/// struct GitStatusMsg(String);
///
/// struct MyModel {
///     git_status: String,
/// }
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         let model = Self { git_status: String::new() };
///         // Run git status command
///         let mut cmd = Command::new("git");
///         cmd.arg("status").arg("--short");
///         
///         let exec_cmd = command::exec_process(cmd, |result| {
///             match result {
///                 Ok(output) => {
///                     let status = String::from_utf8_lossy(&output.stdout).to_string();
///                     Box::new(GitStatusMsg(status)) as Msg
///                 }
///                 Err(e) => {
///                     Box::new(GitStatusMsg(format!("Error: {}", e))) as Msg
///                 }
///             }
///         });
///         (model, Some(exec_cmd))
///     }
///
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         if let Some(GitStatusMsg(status)) = msg.downcast_ref::<GitStatusMsg>() {
///             self.git_status = status.clone();
///         }
///         None
///     }
///     
///     fn view(&self) -> String {
///         format!("Git status:\n{}", self.git_status)
///     }
/// }
/// ```
pub fn exec_process<F>(cmd: StdCommand, f: F) -> Cmd
where
    F: Fn(Result<std::process::Output, std::io::Error>) -> Msg + Send + 'static,
{
    Box::pin(async move {
        // Apply configured environment variables, if any
        let mut cmd = cmd;
        if let Some(env) = crate::command::COMMAND_ENV.get() {
            for (k, v) in env.iter() {
                cmd.env(k, v);
            }
        }
        let output = TokioCommand::from(cmd).output().await;
        Some(f(output))
    })
}

/// Creates a command that enters the alternate screen buffer.
///
/// This command sends an `EnterAltScreenMsg` to the program, which will cause
/// the terminal to switch to the alternate screen buffer. The alternate screen
/// is typically used by full-screen TUI applications to preserve the user's
/// terminal scrollback.
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg};
///
/// struct MyModel;
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         let model = Self {};
///         // Enter alternate screen on startup
///         let cmd = command::batch(vec![
///             command::enter_alt_screen(),
///             command::hide_cursor(),
///         ]);
///         (model, Some(cmd))
///     }
///     
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         None
///     }
///     
///     fn view(&self) -> String {
///         "TUI Application".to_string()
///     }
/// }
/// ```
pub fn enter_alt_screen() -> Cmd {
    Box::pin(async { Some(Box::new(EnterAltScreenMsg) as Msg) })
}

/// Creates a command that exits the alternate screen buffer.
///
/// This command sends an `ExitAltScreenMsg` to the program, which will cause
/// the terminal to switch back from the alternate screen buffer.
pub fn exit_alt_screen() -> Cmd {
    Box::pin(async { Some(Box::new(ExitAltScreenMsg) as Msg) })
}

/// Creates a command that enables mouse cell motion reporting.
///
/// This command sends an `EnableMouseCellMotionMsg` to the program, which will
/// enable mouse events for individual cells in the terminal.
pub fn enable_mouse_cell_motion() -> Cmd {
    Box::pin(async { Some(Box::new(EnableMouseCellMotionMsg) as Msg) })
}

/// Creates a command that enables all mouse motion reporting.
///
/// This command sends an `EnableMouseAllMotionMsg` to the program, which will
/// enable all mouse events in the terminal.
pub fn enable_mouse_all_motion() -> Cmd {
    Box::pin(async { Some(Box::new(EnableMouseAllMotionMsg) as Msg) })
}

/// Creates a command that disables mouse reporting.
///
/// This command sends a `DisableMouseMsg` to the program, which will disable
/// all mouse events in the terminal.
pub fn disable_mouse() -> Cmd {
    Box::pin(async { Some(Box::new(DisableMouseMsg) as Msg) })
}

/// Creates a command that enables focus reporting.
///
/// This command sends an `EnableReportFocusMsg` to the program, which will
/// enable focus events in the terminal.
pub fn enable_report_focus() -> Cmd {
    Box::pin(async { Some(Box::new(EnableReportFocusMsg) as Msg) })
}

/// Creates a command that disables focus reporting.
///
/// This command sends a `DisableReportFocusMsg` to the program, which will
/// disable focus events in the terminal.
pub fn disable_report_focus() -> Cmd {
    Box::pin(async { Some(Box::new(DisableReportFocusMsg) as Msg) })
}

/// Creates a command that enables bracketed paste mode.
///
/// This command sends an `EnableBracketedPasteMsg` to the program, which will
/// enable bracketed paste mode in the terminal. This helps distinguish pasted
/// text from typed text.
pub fn enable_bracketed_paste() -> Cmd {
    Box::pin(async { Some(Box::new(EnableBracketedPasteMsg) as Msg) })
}

/// Creates a command that disables bracketed paste mode.
///
/// This command sends a `DisableBracketedPasteMsg` to the program, which will
/// disable bracketed paste mode in the terminal.
pub fn disable_bracketed_paste() -> Cmd {
    Box::pin(async { Some(Box::new(DisableBracketedPasteMsg) as Msg) })
}

/// Creates a command that shows the terminal cursor.
///
/// This command sends a `ShowCursorMsg` to the program, which will make the
/// terminal cursor visible.
pub fn show_cursor() -> Cmd {
    Box::pin(async { Some(Box::new(ShowCursorMsg) as Msg) })
}

/// Creates a command that hides the terminal cursor.
///
/// This command sends a `HideCursorMsg` to the program, which will make the
/// terminal cursor invisible.
pub fn hide_cursor() -> Cmd {
    Box::pin(async { Some(Box::new(HideCursorMsg) as Msg) })
}

/// Creates a command that clears the terminal screen.
///
/// This command sends a `ClearScreenMsg` to the program, which will clear
/// all content from the terminal screen.
pub fn clear_screen() -> Cmd {
    Box::pin(async { Some(Box::new(ClearScreenMsg) as Msg) })
}

/// Creates a command that requests the current window size.
///
/// This command sends a `RequestWindowSizeMsg` to the program. The terminal
/// will respond with a `WindowSizeMsg` containing its current dimensions.
/// This is useful for responsive layouts that adapt to terminal size.
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg, WindowSizeMsg};
///
/// struct MyModel {
///     width: u16,
///     height: u16,
/// }
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         let model = Self { width: 0, height: 0 };
///         // Get initial window size
///         (model, Some(command::window_size()))
///     }
///
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
///             self.width = size_msg.width;
///             self.height = size_msg.height;
///         }
///         None
///     }
///     
///     fn view(&self) -> String {
///         format!("Window size: {}x{}", self.width, self.height)
///     }
/// }
/// ```
pub fn window_size() -> Cmd {
    Box::pin(async { Some(Box::new(RequestWindowSizeMsg) as Msg) })
}

/// Creates a command that prints a line to the terminal.
///
/// This command sends a `PrintMsg` to the program, which will print the
/// provided string to the terminal. This is useful for debugging or
/// outputting information that should appear outside the normal UI.
///
/// # Arguments
///
/// * `s` - The string to print
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg};
///
/// struct MyModel {
///     debug_mode: bool,
/// }
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         (Self { debug_mode: true }, None)
///     }
///
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         if self.debug_mode {
///             // Note: In practice, msg doesn't implement Debug by default
///             // This is just for demonstration
///             return Some(command::println(
///                 "Received a message".to_string()
///             ));
///         }
///         None
///     }
///     
///     fn view(&self) -> String {
///         "Debug mode active".to_string()
///     }
/// }
/// ```
pub fn println(s: String) -> Cmd {
    Box::pin(async move { Some(Box::new(PrintMsg(s)) as Msg) })
}

/// Creates a command that prints formatted text to the terminal.
///
/// This command sends a `PrintfMsg` to the program, which will print the
/// provided formatted string to the terminal.
pub fn printf(s: String) -> Cmd {
    Box::pin(async move { Some(Box::new(PrintfMsg(s)) as Msg) })
}

/// Creates a command that sets the terminal window title.
///
/// This command sends a `SetWindowTitleMsg` to the program, which will update
/// the terminal window's title. Note that not all terminals support this feature.
///
/// # Arguments
///
/// * `title` - The new window title
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg};
///
/// struct MyModel {
///     app_name: String,
///     document_name: Option<String>,
/// }
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         let model = Self {
///             app_name: "My App".to_string(),
///             document_name: None,
///         };
///         // Set initial window title
///         let cmd = command::set_window_title(model.app_name.clone());
///         (model, Some(cmd))
///     }
///
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         // In a real app, you'd check for document open messages
///         // Update title when document changes
///         if let Some(doc_name) = &self.document_name {
///             let title = format!("{} - {}", doc_name, self.app_name);
///             return Some(command::set_window_title(title));
///         }
///         None
///     }
///     
///     fn view(&self) -> String {
///         match &self.document_name {
///             Some(doc) => format!("Editing: {}", doc),
///             None => "No document open".to_string(),
///         }
///     }
/// }
/// ```
pub fn set_window_title(title: String) -> Cmd {
    Box::pin(async move { Some(Box::new(crate::event::SetWindowTitleMsg(title)) as Msg) })
}

/// Creates a command that cancels a specific timer.
///
/// This command sends a `CancelTimerMsg` to the program, which will stop
/// the timer with the given ID. Use this with timer IDs returned by
/// `every_with_id()` to stop repeating timers.
///
/// # Arguments
///
/// * `timer_id` - The ID of the timer to cancel
///
/// # Returns
///
/// A command that cancels the specified timer
///
/// # Examples
///
/// ```
/// use bubbletea_rs::{command, Model, Msg, KeyMsg};
/// use crossterm::event::KeyCode;
/// use std::time::Duration;
///
/// struct MyModel {
///     timer_id: Option<u64>,
/// }
///
/// impl Model for MyModel {
///     fn init() -> (Self, Option<command::Cmd>) {
///         (Self { timer_id: Some(123) }, None)
///     }
///
///     fn update(&mut self, msg: Msg) -> Option<command::Cmd> {
///         // Cancel timer when user presses 's' for stop
///         if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
///             if key_msg.key == KeyCode::Char('s') {
///                 if let Some(id) = self.timer_id {
///                     self.timer_id = None;
///                     return Some(command::cancel_timer(id));
///                 }
///             }
///         }
///         None
///     }
///     
///     fn view(&self) -> String {
///         if self.timer_id.is_some() {
///             "Timer running. Press 's' to stop.".to_string()
///         } else {
///             "Timer stopped.".to_string()
///         }
///     }
/// }
/// ```
pub fn cancel_timer(timer_id: u64) -> Cmd {
    Box::pin(async move { Some(Box::new(crate::event::CancelTimerMsg { timer_id }) as Msg) })
}

/// Creates a command that cancels all active timers.
///
/// This command sends a `CancelAllTimersMsg` to the program, which will stop
/// all currently running timers.
pub fn cancel_all_timers() -> Cmd {
    Box::pin(async move { Some(Box::new(crate::event::CancelAllTimersMsg) as Msg) })
}
