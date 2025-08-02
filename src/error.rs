//! Error types for bubbletea-rs.
//! This module defines the custom error types used throughout the `bubbletea-rs` library.
//! All errors are unified under the `Error` enum, providing a consistent way to handle
//! various failure conditions, from I/O issues to program-specific panics.

use thiserror::Error;

/// The main error type for `bubbletea-rs` operations.
///
/// This enum encapsulates all possible errors that can occur within the library,
/// providing detailed information about the cause of the error.
#[derive(Debug, Error)]
pub enum Error {
    /// Represents a program panic, similar to Go's `ErrProgramPanic`.
    /// This error is typically caught by the `Program` and indicates an unrecoverable
    /// error within the model's `update` or `view` methods.
    #[error("Program panic: {0}")]
    ProgramPanic(String),

    /// Indicates that the program was explicitly killed, similar to Go's `ErrProgramKilled`.
    /// This can happen if the `kill()` method is called on the `Program`.
    #[error("Program was killed")]
    ProgramKilled,

    /// Indicates that the program was interrupted, similar to Go's `ErrInterrupted`.
    /// This typically occurs when an interrupt signal (e.g., Ctrl+C) is received.
    #[error("Program was interrupted")]
    Interrupted,

    /// Represents an I/O error, wrapping `std::io::Error`.
    /// This can occur during terminal operations, file access, or network communication.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Represents an error specifically related to terminal operations.
    /// This can include issues with raw mode, alternate screen, or cursor manipulation.
    #[error("Terminal error: {0}")]
    Terminal(String),

    /// Indicates a failure when sending a message through an MPSC channel.
    /// This usually means the receiving end of the channel has been dropped.
    #[error("Channel send error")]
    ChannelSend,

    /// Indicates a failure when receiving a message from an MPSC channel.
    /// This usually means the sending end of the channel has been dropped.
    #[error("Channel receive error")]
    ChannelReceive,

    /// Represents a configuration error, typically from the `ProgramBuilder`.
    /// This can occur if invalid or inconsistent configuration options are provided.
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Represents an error during the execution of a command.
    /// This can include failures when spawning external processes or issues with command output.
    #[error("Command execution error: {0}")]
    CommandExecution(String),

    /// Represents a generic send error, used when a message fails to be sent.
    #[error("Send error")]
    SendError,
}

/// Implements conversion from `tokio::sync::mpsc::error::SendError<T>` to `Error::ChannelSend`.
/// This allows `?` operator to be used with Tokio MPSC send operations.
impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::ChannelSend
    }
}

/// Implements conversion from `tokio::sync::mpsc::error::TryRecvError` to `Error::ChannelReceive`.
/// This allows `?` operator to be used with Tokio MPSC try_recv operations.
impl From<tokio::sync::mpsc::error::TryRecvError> for Error {
    fn from(_: tokio::sync::mpsc::error::TryRecvError) -> Self {
        Error::ChannelReceive
    }
}

/// Implements conversion from `tokio::sync::oneshot::error::RecvError` to `Error::ChannelReceive`.
/// This allows `?` operator to be used with Tokio Oneshot receive operations.
impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(_: tokio::sync::oneshot::error::RecvError) -> Self {
        Error::ChannelReceive
    }
}

/// Implements conversion from `String` to `Error::Configuration`.
/// This provides a convenient way to create configuration errors from string messages.
impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Configuration(msg)
    }
}

/// Implements conversion from `&str` to `Error::Configuration`.
/// This provides a convenient way to create configuration errors from string slices.
impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Configuration(msg.to_string())
    }
}
