//! Input handling system for the Bubble Tea TUI framework.
//!
//! This module provides the core input processing functionality for `bubbletea-rs`.
//! It is responsible for reading terminal events (keyboard, mouse, resize, focus, paste)
//! and converting them into messages that can be processed by the application's model
//! following the Model-View-Update (MVU) pattern.
//!
//! # Key Components
//!
//! - [`InputHandler`] - The main event processor that runs the input loop
//! - [`InputSource`] - Enum defining different input sources (terminal or custom)
//!
//! # Examples
//!
//! Basic usage with terminal input:
//!
//! ```rust
//! use bubbletea_rs::input::{InputHandler, InputSource};
//! use tokio::sync::mpsc;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let (tx, rx) = mpsc::unbounded_channel();
//! let input_handler = InputHandler::new(tx);
//!
//! // Start the input processing loop
//! tokio::spawn(async move {
//!     input_handler.run().await
//! });
//! # Ok(())
//! # }
//! ```
//!
//! Using a custom input source:
//!
//! ```rust
//! use bubbletea_rs::input::{InputHandler, InputSource};
//! use tokio::sync::mpsc;
//! use std::pin::Pin;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let (tx, rx) = mpsc::unbounded_channel();
//! let custom_reader = Box::pin(std::io::Cursor::new("hello\n"));
//! let input_source = InputSource::Custom(custom_reader);
//! let input_handler = InputHandler::with_source(tx, input_source);
//!
//! // Process input from the custom source
//! input_handler.run().await?;
//! # Ok(())
//! # }
//! ```

use crate::{Error, KeyMsg, MouseMsg, WindowSizeMsg};
use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};
use futures::StreamExt;
use std::pin::Pin;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};

/// Represents different input sources that the `InputHandler` can read from.
///
/// This enum allows the program to read input from either the standard crossterm
/// event stream (for regular terminal input) or from a custom async reader.
pub enum InputSource {
    /// Standard terminal input using crossterm's event stream.
    /// This is the default and handles keyboard, mouse, and resize events.
    Terminal,

    /// Custom input reader that implements `AsyncRead + Send + Unpin`.
    /// This allows reading input from files, network streams, or other sources.
    /// The custom reader is expected to provide line-based input.
    Custom(Pin<Box<dyn AsyncRead + Send + Unpin>>),
}

/// `InputHandler` is responsible for processing terminal events and sending them
/// as messages to the `Program`'s event loop.
///
/// It continuously reads events from the `crossterm` event stream and converts
/// them into appropriate `Msg` types.
pub struct InputHandler {
    /// The sender half of an MPSC channel used to send messages
    /// to the `Program`'s event loop.
    pub event_tx: crate::event::EventSender,

    /// The input source to read from.
    pub input_source: InputSource,
}

impl InputHandler {
    /// Creates a new `InputHandler` with the given message sender using terminal input.
    ///
    /// This constructor sets up the input handler to read from the standard terminal
    /// using crossterm's event stream. This is the most common usage pattern.
    ///
    /// # Arguments
    ///
    /// * `event_tx` - An `EventSender` to send processed events to the main program loop
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bubbletea_rs::input::InputHandler;
    /// use tokio::sync::mpsc;
    ///
    /// let (tx, rx) = mpsc::unbounded_channel();
    /// let input_handler = InputHandler::new(tx);
    /// ```
    pub fn new<T>(event_tx: T) -> Self
    where
        T: Into<crate::event::EventSender>,
    {
        Self {
            event_tx: event_tx.into(),
            input_source: InputSource::Terminal,
        }
    }

    /// Creates a new `InputHandler` with a custom input source.
    ///
    /// This constructor allows you to specify a custom input source instead of
    /// the default terminal input. This is useful for testing, reading from files,
    /// or processing input from network streams.
    ///
    /// # Arguments
    ///
    /// * `event_tx` - An `EventSender` to send processed events to the main program loop
    /// * `input_source` - The `InputSource` to read from (terminal or custom reader)
    ///
    /// # Examples
    ///
    /// Reading from a file:
    ///
    /// ```rust
    /// use bubbletea_rs::input::{InputHandler, InputSource};
    /// use tokio::sync::mpsc;
    /// use std::pin::Pin;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let (tx, rx) = mpsc::unbounded_channel();
    /// let file_content = std::io::Cursor::new("test input\n");
    /// let custom_source = InputSource::Custom(Box::pin(file_content));
    /// let input_handler = InputHandler::with_source(tx, custom_source);
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_source<T>(event_tx: T, input_source: InputSource) -> Self
    where
        T: Into<crate::event::EventSender>,
    {
        Self {
            event_tx: event_tx.into(),
            input_source,
        }
    }

    /// Runs the input handler loop asynchronously.
    ///
    /// This method continuously reads events from the configured input source
    /// and processes them until the loop terminates. It converts raw terminal
    /// events into typed `Msg` objects and sends them through the event channel
    /// to the main program loop.
    ///
    /// The loop terminates when:
    /// - The event sender channel is closed (receiver dropped)
    /// - An I/O error occurs while reading input
    /// - EOF is reached for custom input sources
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on normal termination, or an `Error` if an I/O error occurs.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// - There's an I/O error reading from the input source
    /// - The underlying crossterm event stream encounters an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bubbletea_rs::input::InputHandler;
    /// use tokio::sync::mpsc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let (tx, mut rx) = mpsc::unbounded_channel();
    /// let input_handler = InputHandler::new(tx);
    ///
    /// // Run the input handler in a separate task
    /// let input_task = tokio::spawn(async move {
    ///     input_handler.run().await
    /// });
    ///
    /// // Process incoming messages
    /// while let Some(msg) = rx.recv().await {
    ///     // Handle the message...
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run(self) -> Result<(), Error> {
        let event_tx = self.event_tx;
        match self.input_source {
            InputSource::Terminal => Self::run_terminal_input(event_tx).await,
            InputSource::Custom(reader) => Self::run_custom_input(event_tx, reader).await,
        }
    }

    /// Runs the terminal input handler using crossterm's event stream.
    ///
    /// This method processes standard terminal events including:
    /// - Keyboard input (keys and modifiers)
    /// - Mouse events (clicks, movements, scrolling)
    /// - Terminal resize events
    /// - Focus gained/lost events
    /// - Paste events (when bracketed paste is enabled)
    ///
    /// # Arguments
    ///
    /// * `event_tx` - Channel sender for dispatching processed events
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the event stream ends normally, or an `Error`
    /// if there's an I/O error reading from the terminal.
    ///
    /// # Errors
    ///
    /// Returns an error if crossterm's event stream encounters an I/O error.
    async fn run_terminal_input(event_tx: crate::event::EventSender) -> Result<(), Error> {
        let mut event_stream = EventStream::new();

        while let Some(event) = event_stream.next().await {
            match event {
                Ok(Event::Key(key_event)) => {
                    let msg = KeyMsg {
                        key: key_event.code,
                        modifiers: key_event.modifiers,
                    };
                    if event_tx.send(Box::new(msg)).is_err() {
                        break;
                    }
                }
                Ok(Event::Mouse(mouse_event)) => {
                    let msg = MouseMsg {
                        x: mouse_event.column,
                        y: mouse_event.row,
                        button: mouse_event.kind,
                        modifiers: mouse_event.modifiers,
                    };
                    if event_tx.send(Box::new(msg)).is_err() {
                        break;
                    }
                }
                Ok(Event::Resize(width, height)) => {
                    let msg = WindowSizeMsg { width, height };
                    if event_tx.send(Box::new(msg)).is_err() {
                        break;
                    }
                }
                Ok(Event::FocusGained) => {
                    let msg = crate::FocusMsg;
                    if event_tx.send(Box::new(msg)).is_err() {
                        break;
                    }
                }
                Ok(Event::FocusLost) => {
                    let msg = crate::BlurMsg;
                    if event_tx.send(Box::new(msg)).is_err() {
                        break;
                    }
                }
                Ok(Event::Paste(pasted_text)) => {
                    let msg = crate::event::PasteMsg(pasted_text);
                    if event_tx.send(Box::new(msg)).is_err() {
                        break;
                    }
                }
                Err(e) => {
                    return Err(Error::Io(e));
                }
            }
        }

        Ok(())
    }

    /// Runs the custom input handler from an async reader.
    ///
    /// This method reads line-based input from a custom async reader and converts
    /// each line into individual `KeyMsg` events. Each character in a line becomes
    /// a separate key event, and the newline is converted to an `Enter` key event.
    ///
    /// This is primarily intended for testing and scenarios where you need to
    /// simulate keyboard input from a file or other source.
    ///
    /// # Arguments
    ///
    /// * `event_tx` - Channel sender for dispatching processed events
    /// * `reader` - The async reader to read input from
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when EOF is reached or the event channel is closed,
    /// or an `Error` if there's an I/O error reading from the source.
    ///
    /// # Errors
    ///
    /// Returns an error if there's an I/O error reading from the async reader.
    ///
    /// # Examples
    ///
    /// The input "hello\n" would generate the following key events:
    /// - `KeyMsg { key: KeyCode::Char('h'), modifiers: KeyModifiers::NONE }`
    /// - `KeyMsg { key: KeyCode::Char('e'), modifiers: KeyModifiers::NONE }`
    /// - `KeyMsg { key: KeyCode::Char('l'), modifiers: KeyModifiers::NONE }`
    /// - `KeyMsg { key: KeyCode::Char('l'), modifiers: KeyModifiers::NONE }`
    /// - `KeyMsg { key: KeyCode::Char('o'), modifiers: KeyModifiers::NONE }`
    /// - `KeyMsg { key: KeyCode::Enter, modifiers: KeyModifiers::NONE }`
    async fn run_custom_input(
        event_tx: crate::event::EventSender,
        reader: Pin<Box<dyn AsyncRead + Send + Unpin>>,
    ) -> Result<(), Error> {
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();
            match buf_reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    // Process each character in the line as a separate key event
                    for ch in line.trim().chars() {
                        let msg = KeyMsg {
                            key: KeyCode::Char(ch),
                            modifiers: KeyModifiers::NONE,
                        };
                        if event_tx.send(Box::new(msg)).is_err() {
                            return Ok(());
                        }
                    }

                    // Send Enter key for the newline
                    if line.ends_with('\n') {
                        let msg = KeyMsg {
                            key: KeyCode::Enter,
                            modifiers: KeyModifiers::NONE,
                        };
                        if event_tx.send(Box::new(msg)).is_err() {
                            return Ok(());
                        }
                    }
                }
                Err(e) => return Err(Error::Io(e)),
            }
        }

        Ok(())
    }
}
