//! This module provides the input handling system for `bubbletea-rs`.
//! It is responsible for reading terminal events and converting them into messages
//! that can be processed by the application's model.

//! This module provides the input handling system for `bubbletea-rs`.
//! It is responsible for reading terminal events and converting them into messages
//! that can be processed by the application's model.

use crate::{Error, KeyMsg, MouseMsg, Msg, WindowSizeMsg};
use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};
use futures::StreamExt;
use std::pin::Pin;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio::sync::mpsc;

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
    /// The sender half of an unbounded MPSC channel used to send messages
    /// to the `Program`'s event loop.
    pub event_tx: mpsc::UnboundedSender<Msg>,

    /// The input source to read from.
    pub input_source: InputSource,
}

impl InputHandler {
    /// Creates a new `InputHandler` with the given message sender using terminal input.
    ///
    /// # Arguments
    ///
    /// * `event_tx` - An `mpsc::UnboundedSender<Msg>` to send processed events.
    pub fn new(event_tx: mpsc::UnboundedSender<Msg>) -> Self {
        Self {
            event_tx,
            input_source: InputSource::Terminal,
        }
    }

    /// Creates a new `InputHandler` with a custom input source.
    ///
    /// # Arguments
    ///
    /// * `event_tx` - An `mpsc::UnboundedSender<Msg>` to send processed events.
    /// * `input_source` - The `InputSource` to read from.
    pub fn with_source(event_tx: mpsc::UnboundedSender<Msg>, input_source: InputSource) -> Self {
        Self {
            event_tx,
            input_source,
        }
    }

    /// Runs the input handler loop asynchronously.
    ///
    /// This method continuously reads events from the configured input source.
    /// It converts events into `bubbletea-rs` `Msg` types and sends them through
    /// the `event_tx` channel. The loop breaks if sending fails (e.g., if the
    /// receiver is dropped) or if an I/O error occurs.
    pub async fn run(self) -> Result<(), Error> {
        let event_tx = self.event_tx;
        match self.input_source {
            InputSource::Terminal => Self::run_terminal_input(event_tx).await,
            InputSource::Custom(reader) => Self::run_custom_input(event_tx, reader).await,
        }
    }

    /// Runs the terminal input handler using crossterm's event stream.
    async fn run_terminal_input(event_tx: mpsc::UnboundedSender<Msg>) -> Result<(), Error> {
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
                Ok(Event::Paste(_)) => {
                    continue;
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
    /// This method reads line-based input from the custom reader and converts
    /// each line into a KeyMsg with the line content as individual characters.
    /// This is a simplified approach for demonstration purposes.
    async fn run_custom_input(
        event_tx: mpsc::UnboundedSender<Msg>,
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
