//! Error types for bubbletea-rs.
//!
//! This module defines the custom error types used throughout the `bubbletea-rs` library.
//! All errors are unified under the `Error` enum, providing a consistent way to handle
//! various failure conditions, from I/O issues to program-specific panics.
//!
//! # Error Handling Philosophy
//!
//! The `bubbletea-rs` library follows Rust's idiomatic error handling patterns. All
//! fallible operations return `Result<T, Error>` where `Error` is the main error type
//! defined in this module. The library uses the `thiserror` crate to provide clear
//! error messages and convenient error conversions.
//!
//! # Common Usage Patterns
//!
//! ## Basic Error Handling
//!
//! ```no_run
//! use bubbletea_rs::{Program, Model, Msg, Error, Cmd};
//!
//! # struct MyModel;
//! # impl Model for MyModel {
//! #     fn init() -> (Self, Option<Cmd>) { (MyModel, None) }
//! #     fn update(&mut self, _msg: Msg) -> Option<Cmd> { None }
//! #     fn view(&self) -> String { String::new() }
//! # }
//! async fn run_program() -> Result<(), Error> {
//!     let program = Program::<MyModel>::builder().build()?;
//!     program.run().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Pattern Matching on Errors
//!
//! ```no_run
//! use bubbletea_rs::Error;
//!
//! fn handle_error(err: Error) {
//!     match err {
//!         Error::Interrupted => {
//!             println!("Program was interrupted by user");
//!         }
//!         Error::ProgramKilled => {
//!             println!("Program was explicitly killed");
//!         }
//!         Error::Io(io_err) => {
//!             eprintln!("I/O error occurred: {}", io_err);
//!         }
//!         _ => {
//!             eprintln!("Unexpected error: {}", err);
//!         }
//!     }
//! }
//! ```
//!
//! ## Converting Between Error Types
//!
//! The library provides automatic conversions from common error types:
//!
//! ```no_run
//! use bubbletea_rs::Error;
//! use std::io;
//!
//! fn io_operation() -> Result<String, Error> {
//!     // std::io::Error is automatically converted to Error::Io
//!     let contents = std::fs::read_to_string("file.txt")?;
//!     Ok(contents)
//! }
//! ```

use thiserror::Error;

/// The main error type for `bubbletea-rs` operations.
///
/// This enum encapsulates all possible errors that can occur within the library,
/// providing detailed information about the cause of the error.
///
/// # Examples
///
/// ## Creating errors manually
///
/// ```
/// use bubbletea_rs::Error;
///
/// // Create a configuration error
/// let err = Error::Configuration("Invalid input device".to_string());
///
/// // Create a program panic error
/// let panic_err = Error::ProgramPanic("Model update failed".to_string());
/// ```
///
/// ## Working with Results
///
/// ```no_run
/// use bubbletea_rs::{Error, Program, Model, Msg, Cmd};
///
/// # struct MyModel;
/// # impl Model for MyModel {
/// #     fn init() -> (Self, Option<Cmd>) { (MyModel, None) }
/// #     fn update(&mut self, _msg: Msg) -> Option<Cmd> { None }
/// #     fn view(&self) -> String { String::new() }
/// # }
/// fn create_program() -> Result<Program<MyModel>, Error> {
///     Program::<MyModel>::builder().build()
/// }
/// ```
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

    /// Bounded channel is full (backpressure). The message could not be enqueued.
    #[error("Channel is full")]
    ChannelFull,

    /// Channel is closed; no receivers (or senders) are available.
    #[error("Channel is closed")]
    ChannelClosed,
}

/// Implements conversion from `tokio::sync::mpsc::error::SendError<T>` to `Error::ChannelSend`.
///
/// This allows `?` operator to be used with Tokio MPSC send operations, making error
/// propagation seamless when working with channels.
///
/// # Examples
///
/// ```no_run
/// use bubbletea_rs::{Error, Msg};
/// use tokio::sync::mpsc;
///
/// async fn send_message(sender: mpsc::Sender<Msg>, msg: Msg) -> Result<(), Error> {
///     // The ? operator automatically converts SendError to Error::ChannelSend
///     sender.send(msg).await?;
///     Ok(())
/// }
/// ```
impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::ChannelSend
    }
}

/// Implements conversion from `tokio::sync::mpsc::error::TryRecvError` to `Error::ChannelReceive`.
///
/// This allows `?` operator to be used with Tokio MPSC try_recv operations.
///
/// # Examples
///
/// ```no_run
/// use bubbletea_rs::{Error, Msg};
/// use tokio::sync::mpsc;
///
/// fn try_receive(receiver: &mut mpsc::Receiver<Msg>) -> Result<Option<Msg>, Error> {
///     // The ? operator automatically converts TryRecvError to Error::ChannelReceive
///     match receiver.try_recv() {
///         Ok(msg) => Ok(Some(msg)),
///         Err(mpsc::error::TryRecvError::Empty) => Ok(None),
///         Err(e) => Err(e.into()), // Converts to Error::ChannelReceive
///     }
/// }
/// ```
impl From<tokio::sync::mpsc::error::TryRecvError> for Error {
    fn from(_: tokio::sync::mpsc::error::TryRecvError) -> Self {
        Error::ChannelReceive
    }
}

/// Implements conversion from `tokio::sync::oneshot::error::RecvError` to `Error::ChannelReceive`.
///
/// This allows `?` operator to be used with Tokio Oneshot receive operations.
///
/// # Examples
///
/// ```no_run
/// use bubbletea_rs::Error;
/// use tokio::sync::oneshot;
///
/// async fn receive_once(receiver: oneshot::Receiver<String>) -> Result<String, Error> {
///     // The ? operator automatically converts RecvError to Error::ChannelReceive
///     let value = receiver.await?;
///     Ok(value)
/// }
/// ```
impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(_: tokio::sync::oneshot::error::RecvError) -> Self {
        Error::ChannelReceive
    }
}

/// Implements conversion from `tokio::sync::mpsc::error::TrySendError<T>` to
/// channel-related errors that preserve whether the channel was full or closed.
///
/// This conversion distinguishes between `ChannelFull` (backpressure) and `ChannelClosed`
/// errors, providing more specific error information than the generic `ChannelSend`.
///
/// # Examples
///
/// ```no_run
/// use bubbletea_rs::{Error, Msg};
/// use tokio::sync::mpsc;
///
/// fn try_send(sender: &mpsc::Sender<Msg>, msg: Msg) -> Result<(), Error> {
///     // Automatically converts to Error::ChannelFull or Error::ChannelClosed
///     sender.try_send(msg)?;
///     Ok(())
/// }
/// ```
impl<T> From<tokio::sync::mpsc::error::TrySendError<T>> for Error {
    fn from(err: tokio::sync::mpsc::error::TrySendError<T>) -> Self {
        use tokio::sync::mpsc::error::TrySendError;
        match err {
            TrySendError::Full(_) => Error::ChannelFull,
            TrySendError::Closed(_) => Error::ChannelClosed,
        }
    }
}

/// Implements conversion from `String` to `Error::Configuration`.
///
/// This provides a convenient way to create configuration errors from string messages.
///
/// # Examples
///
/// ```
/// use bubbletea_rs::Error;
///
/// fn validate_config(value: u32) -> Result<(), Error> {
///     if value > 100 {
///         return Err(format!("Value {} exceeds maximum of 100", value).into());
///     }
///     Ok(())
/// }
/// ```
impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Configuration(msg)
    }
}

/// Implements conversion from `&str` to `Error::Configuration`.
///
/// This provides a convenient way to create configuration errors from string slices.
///
/// # Examples
///
/// ```
/// use bubbletea_rs::Error;
///
/// fn check_name(name: &str) -> Result<(), Error> {
///     if name.is_empty() {
///         return Err("Name cannot be empty".into());
///     }
///     Ok(())
/// }
/// ```
impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Configuration(msg.to_string())
    }
}
