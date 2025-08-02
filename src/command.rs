//! This module provides functions for creating and managing commands.
//! Commands are asynchronous operations that can produce messages to update the model.

use crate::event::{
    ClearScreenMsg, DisableBracketedPasteMsg, DisableMouseMsg, DisableReportFocusMsg,
    EnableBracketedPasteMsg, EnableMouseAllMotionMsg, EnableMouseCellMotionMsg,
    EnableReportFocusMsg, EnterAltScreenMsg, ExitAltScreenMsg, HideCursorMsg, InterruptMsg, Msg,
    PrintMsg, PrintfMsg, QuitMsg, RequestWindowSizeMsg, ShowCursorMsg, SuspendMsg,
};
use std::future::Future;
use std::pin::Pin;
use std::process::Command as StdCommand;
use std::time::Duration;
use tokio::process::Command as TokioCommand;
use tokio::time::interval;

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

/// Creates a command that quits the application.
///
/// This command sends a `QuitMsg` to the program, which will initiate the
/// shutdown process.
pub fn quit() -> Cmd {
    Box::pin(async { Some(Box::new(QuitMsg) as Msg) })
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
/// The commands in the batch will be executed in parallel and all messages
/// from the commands will be collected and returned as a BatchMsgInternal.
pub fn batch(cmds: Vec<Cmd>) -> Cmd {
    Box::pin(async move {
        use futures::future::join_all;
        
        let results = join_all(cmds).await;
        let messages: Vec<Msg> = results.into_iter().flatten().collect();
        
        if messages.is_empty() {
            None
        } else {
            Some(Box::new(crate::event::BatchMsgInternal { messages }) as Msg)
        }
    })
}

/// Creates a command that executes a sequence of commands sequentially.
///
/// The commands in the sequence will be executed one after another. If any of
/// the commands produce a message, the first message received will be returned.
/// This behavior might be refined in future versions to handle multiple
/// messages from a sequence.
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

/// Creates a command that produces a message at a regular interval.
///
/// This command will send a message produced by the provided closure `f`
/// after every `duration`.
///
/// # Arguments
///
/// * `duration` - The duration between messages.
/// * `f` - A closure that takes a `Duration` and returns a `Msg`.
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

/// Creates a command that produces a message repeatedly at a regular interval.
///
/// This command will continuously send messages produced by the provided closure `f`
/// after every `duration` until the program exits.
///
/// # Arguments
///
/// * `duration` - The duration between messages.
/// * `f` - A closure that takes a `Duration` and returns a `Msg`.
pub fn every<F>(duration: Duration, f: F) -> Cmd
where
    F: Fn(Duration) -> Msg + Send + 'static,
{
    Box::pin(async move {
        Some(Box::new(crate::event::EveryMsgInternal {
            duration,
            func: Box::new(f),
        }) as Msg)
    })
}

/// Creates a command that executes an external process.
///
/// This command spawns an external process and returns a message produced by
/// the provided closure `f` with the process's output.
///
/// # Arguments
///
/// * `cmd` - The `std::process::Command` to execute.
/// * `f` - A closure that takes a `Result<std::process::Output, std::io::Error>`
///         and returns a `Msg`.
pub fn exec_process<F>(cmd: StdCommand, f: F) -> Cmd
where
    F: Fn(Result<std::process::Output, std::io::Error>) -> Msg + Send + 'static,
{
    Box::pin(async move {
        let output = TokioCommand::from(cmd).output().await;
        Some(f(output))
    })
}

/// Creates a command that enters the alternate screen buffer.
///
/// This command sends an `EnterAltScreenMsg` to the program, which will cause
/// the terminal to switch to the alternate screen buffer.
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
pub fn window_size() -> Cmd {
    Box::pin(async { Some(Box::new(RequestWindowSizeMsg) as Msg) })
}

/// Creates a command that prints a line to the terminal.
///
/// This command sends a `PrintMsg` to the program, which will print the
/// provided string to the terminal.
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
/// the terminal window's title.
pub fn set_window_title(title: String) -> Cmd {
    Box::pin(async move { Some(Box::new(crate::event::SetWindowTitleMsg(title)) as Msg) })
}
