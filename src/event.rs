//! This module defines the various message types used in `bubbletea-rs`.
//! Messages are events that trigger updates in your application's model.
//! They are typically sent by commands or the input handler.

use std::any::Any;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// A message represents any event that can trigger a model update.
///
/// `Msg` is a type alias for `Box<dyn Any + Send>`, allowing it to hold
/// any type that implements `Any` and `Send`. This provides flexibility
/// in defining custom message types for your application.
pub type Msg = Box<dyn Any + Send>;

/// Event sender abstraction that can be either bounded or unbounded.
#[derive(Clone)]
pub enum EventSender {
    Unbounded(mpsc::UnboundedSender<Msg>),
    Bounded(mpsc::Sender<Msg>),
}

impl EventSender {
    /// Send a message through the channel.
    pub fn send(&self, msg: Msg) -> Result<(), crate::Error> {
        match self {
            // Unbounded send fails only when the receiver is closed.
            EventSender::Unbounded(tx) => tx
                .send(msg)
                .map_err(|_| crate::Error::ChannelClosed),
            // Bounded send can fail due to Full (backpressure) or Closed.
            EventSender::Bounded(tx) => tx.try_send(msg).map_err(Into::into),
        }
    }

    /// Check if the sender is closed.
    pub fn is_closed(&self) -> bool {
        match self {
            EventSender::Unbounded(tx) => tx.is_closed(),
            EventSender::Bounded(tx) => tx.is_closed(),
        }
    }

    /// Create an EventSender from an UnboundedSender (for backward compatibility).
    pub fn from_unbounded(tx: mpsc::UnboundedSender<Msg>) -> Self {
        EventSender::Unbounded(tx)
    }

    /// Create an EventSender from a bounded Sender (for testing).
    pub fn from_bounded(tx: mpsc::Sender<Msg>) -> Self {
        EventSender::Bounded(tx)
    }
}

impl From<mpsc::UnboundedSender<Msg>> for EventSender {
    fn from(tx: mpsc::UnboundedSender<Msg>) -> Self {
        EventSender::Unbounded(tx)
    }
}

impl From<mpsc::Sender<Msg>> for EventSender {
    fn from(tx: mpsc::Sender<Msg>) -> Self {
        EventSender::Bounded(tx)
    }
}

/// Event receiver abstraction that can be either bounded or unbounded.
pub enum EventReceiver {
    Unbounded(mpsc::UnboundedReceiver<Msg>),
    Bounded(mpsc::Receiver<Msg>),
}

impl EventReceiver {
    /// Receive the next message from the channel.
    pub async fn recv(&mut self) -> Option<Msg> {
        match self {
            EventReceiver::Unbounded(rx) => rx.recv().await,
            EventReceiver::Bounded(rx) => rx.recv().await,
        }
    }
}

/// Global event sender set by Program on startup so commands can emit messages
/// back into the event loop from background tasks.
pub static EVENT_SENDER: OnceLock<EventSender> = OnceLock::new();

/// Global timer ID generator for unique timer identification.
static TIMER_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Generates a unique timer ID.
pub fn next_timer_id() -> u64 {
    TIMER_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

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
    pub cancellation_token: CancellationToken,
    pub timer_id: u64,
}

impl std::fmt::Debug for EveryMsgInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EveryMsgInternal")
            .field("duration", &self.duration)
            .field("timer_id", &self.timer_id)
            .field("func", &"<closure>")
            .finish()
    }
}

/// A message to cancel a specific timer.
#[derive(Debug, Clone)]
pub struct CancelTimerMsg {
    pub timer_id: u64,
}

/// A message to cancel all active timers.
#[derive(Debug, Clone)]
pub struct CancelAllTimersMsg;
