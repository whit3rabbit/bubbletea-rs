//! This module defines the various message types used in `bubbletea-rs`.
//! Messages are events that trigger updates in your application's model.
//! They are typically sent by commands or the input handler.

use std::any::Any;
use tokio::sync::mpsc;
use std::sync::OnceLock;

/// A message represents any event that can trigger a model update.
///
/// `Msg` is a type alias for `Box<dyn Any + Send>`, allowing it to hold
/// any type that implements `Any` and `Send`. This provides flexibility
/// in defining custom message types for your application.
pub type Msg = Box<dyn Any + Send>;

/// Global event sender set by Program on startup so commands can emit messages
/// back into the event loop from background tasks.
pub static EVENT_SENDER: OnceLock<mpsc::UnboundedSender<Msg>> = OnceLock::new();

/// A message indicating a keyboard input event.
#[derive(Debug, Clone)]
pub struct KeyMsg {
    /// The `crossterm::event::KeyCode` representing the key pressed.
    pub key: crossterm::event::KeyCode,
    /// The `crossterm::event::KeyModifiers` active during the key press.
    pub modifiers: crossterm::event::KeyModifiers,
}

/// A message indicating a mouse input event.
#[derive(Debug, Clone)]
pub struct MouseMsg {
    /// The column coordinate of the mouse event.
    pub x: u16,
    /// The row coordinate of the mouse event.
    pub y: u16,
    /// The `crossterm::event::MouseEventKind` representing the type of mouse event.
    pub button: crossterm::event::MouseEventKind,
    /// The `crossterm::event::KeyModifiers` active during the mouse event.
    pub modifiers: crossterm::event::KeyModifiers,
}

/// A message indicating a change in the terminal window size.
#[derive(Debug, Clone)]
pub struct WindowSizeMsg {
    /// The new width of the terminal window.
    pub width: u16,
    /// The new height of the terminal window.
    pub height: u16,
}

/// A message to signal the application to quit.
///
/// Sending this message to the `Program` will initiate a graceful shutdown.
#[derive(Debug, Clone)]
pub struct QuitMsg;

/// A message to signal an application interruption.
///
/// This is typically sent when an interrupt signal (e.g., Ctrl+C) is received.
#[derive(Debug, Clone)]
pub struct InterruptMsg;

/// A message to signal the application to suspend.
///
/// This can be used to temporarily pause the application, for example, when
/// another process needs control of the terminal.
#[derive(Debug, Clone)]
pub struct SuspendMsg;

/// A message to signal the application to resume after suspension.
#[derive(Debug, Clone)]
pub struct ResumeMsg;

/// A message indicating that the terminal gained focus.
#[derive(Debug, Clone)]
pub struct FocusMsg;

/// A message indicating that the terminal lost focus.
#[derive(Debug, Clone)]
pub struct BlurMsg;

/// An internal message type used to batch multiple messages together.
/// This is not exposed as a public API.
#[derive(Debug)]
pub struct BatchMsgInternal {
    pub messages: Vec<Msg>,
}

/// A message to signal the terminal to enter the alternate screen buffer.
#[derive(Debug, Clone)]
pub struct EnterAltScreenMsg;

/// A message to signal the terminal to exit the alternate screen buffer.
#[derive(Debug, Clone)]
pub struct ExitAltScreenMsg;

/// A message to signal the terminal to enable mouse cell motion reporting.
#[derive(Debug, Clone)]
pub struct EnableMouseCellMotionMsg;

/// A message to signal the terminal to enable all mouse motion reporting.
#[derive(Debug, Clone)]
pub struct EnableMouseAllMotionMsg;

/// A message to signal the terminal to disable mouse reporting.
#[derive(Debug, Clone)]
pub struct DisableMouseMsg;

/// A message to signal the terminal to enable bracketed paste mode.
#[derive(Debug, Clone)]
pub struct EnableBracketedPasteMsg;

/// A message to signal the terminal to disable bracketed paste mode.
#[derive(Debug, Clone)]
pub struct DisableBracketedPasteMsg;

/// A message to signal the terminal to enable focus reporting.
#[derive(Debug, Clone)]
pub struct EnableReportFocusMsg;

/// A message to signal the terminal to disable focus reporting.
#[derive(Debug, Clone)]
pub struct DisableReportFocusMsg;

/// A message to signal the terminal to show the cursor.
#[derive(Debug, Clone)]
pub struct ShowCursorMsg;

/// A message to signal the terminal to hide the cursor.
#[derive(Debug, Clone)]
pub struct HideCursorMsg;

/// A message to signal the terminal to clear the screen.
#[derive(Debug, Clone)]
pub struct ClearScreenMsg;

/// A message to signal the terminal to request its current window size.
///
/// The terminal will respond with a `WindowSizeMsg` containing its dimensions.
#[derive(Debug, Clone)]
pub struct RequestWindowSizeMsg;

/// A message to print a line to the terminal.
#[derive(Debug, Clone)]
pub struct PrintMsg(pub String);

/// A message to print formatted text to the terminal.
#[derive(Debug, Clone)]
pub struct PrintfMsg(pub String);

/// A message to set the terminal window title.
#[derive(Debug, Clone)]
pub struct SetWindowTitleMsg(pub String);

/// An internal message used to start a recurring timer.
/// This is not exposed as a public API.
pub struct EveryMsgInternal {
    pub duration: std::time::Duration,
    pub func: Box<dyn Fn(std::time::Duration) -> Msg + Send>,
}

impl std::fmt::Debug for EveryMsgInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EveryMsgInternal")
            .field("duration", &self.duration)
            .field("func", &"<closure>")
            .finish()
    }
}
