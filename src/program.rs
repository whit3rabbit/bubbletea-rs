//! This module defines the `Program` struct and its associated `ProgramBuilder`,
//! which are responsible for coordinating the entire `bubbletea-rs` application lifecycle.
//! The `Program` sets up the terminal, handles input, executes commands, and renders
//! the model's view.

use crate::{Error, InputHandler, InputSource, Model, Msg, QuitMsg, Terminal, TerminalInterface};
use futures::{future::FutureExt, select};
use std::marker::PhantomData;
use std::panic;
use std::sync::OnceLock;
use tokio::sync::mpsc;

static ORIGINAL_PANIC_HOOK: OnceLock<
    Box<dyn Fn(&panic::PanicHookInfo<'_>) + Send + Sync + 'static>,
> = OnceLock::new();

/// Defines the different modes for mouse motion reporting.
#[derive(Debug, Clone, Copy)]
pub enum MouseMotion {
    /// No mouse motion events are reported.
    None,
    /// Mouse motion events are reported when the mouse moves over a different cell.
    Cell,
    /// Mouse motion events are reported for every pixel movement.
    All,
}

use std::sync::Arc;
use tokio::io::AsyncWrite;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

/// Configuration options for a `Program`.
///
/// This struct holds various settings that control the behavior of the `Program`,
/// such as terminal features, rendering options, and panic/signal handling.
pub struct ProgramConfig {
    /// Whether to use the alternate screen buffer.
    pub alt_screen: bool,
    /// The mouse motion reporting mode.
    pub mouse_motion: MouseMotion,
    /// Whether to report focus events.
    pub report_focus: bool,
    /// The target frames per second for rendering.
    pub fps: u32,
    /// Whether to disable the renderer entirely.
    pub without_renderer: bool,
    /// Whether to catch panics and convert them into `ProgramPanic` errors.
    pub catch_panics: bool,
    /// Whether to enable signal handling (e.g., Ctrl+C).
    pub signal_handler: bool,
    /// Whether to enable bracketed paste mode.
    pub bracketed_paste: bool,
    /// Optional custom output writer.
    pub output_writer: Option<Arc<Mutex<dyn AsyncWrite + Send + Unpin>>>,
    /// Optional cancellation token for external control.
    pub cancellation_token: Option<CancellationToken>,
    /// Optional message filter function.
    pub message_filter: Option<Box<dyn Fn(Msg) -> Option<Msg> + Send>>,
    /// Optional custom input source.
    pub input_source: Option<InputSource>,
}

impl std::fmt::Debug for ProgramConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgramConfig")
            .field("alt_screen", &self.alt_screen)
            .field("mouse_motion", &self.mouse_motion)
            .field("report_focus", &self.report_focus)
            .field("fps", &self.fps)
            .field("without_renderer", &self.without_renderer)
            .field("catch_panics", &self.catch_panics)
            .field("signal_handler", &self.signal_handler)
            .field("bracketed_paste", &self.bracketed_paste)
            .field("cancellation_token", &self.cancellation_token)
            .finish()
    }
}

impl Default for ProgramConfig {
    /// Returns the default `ProgramConfig`.
    ///
    /// By default, the program does not use the alternate screen, has no mouse
    /// motion reporting, does not report focus, targets 60 FPS, enables rendering,
    /// catches panics, handles signals, and disables bracketed paste.
    fn default() -> Self {
        Self {
            alt_screen: false,
            mouse_motion: MouseMotion::None,
            report_focus: false,
            fps: 60,
            without_renderer: false,
            catch_panics: true,
            signal_handler: true,
            bracketed_paste: false,
            output_writer: None,
            cancellation_token: None,
            message_filter: None,
            input_source: None,
        }
    }
}

/// A builder for creating and configuring `Program` instances.
///
/// The `ProgramBuilder` provides a fluent API for setting various configuration
/// options before building the final `Program`.
pub struct ProgramBuilder<M: Model> {
    config: ProgramConfig,
    _phantom: PhantomData<M>,
}

impl<M: Model> ProgramBuilder<M> {
    /// Creates a new `ProgramBuilder` with default configuration.
    pub(crate) fn new() -> Self {
        Self {
            config: ProgramConfig::default(),
            _phantom: PhantomData,
        }
    }

    /// Sets whether to use the alternate screen buffer.
    ///
    /// When enabled, the application will run in an alternate screen buffer,
    /// preserving the main terminal content.
    pub fn alt_screen(mut self, enabled: bool) -> Self {
        self.config.alt_screen = enabled;
        self
    }

    /// Sets the mouse motion reporting mode.
    ///
    /// # Arguments
    ///
    /// * `motion` - The desired `MouseMotion` mode.
    pub fn mouse_motion(mut self, motion: MouseMotion) -> Self {
        self.config.mouse_motion = motion;
        self
    }

    /// Sets whether to report focus events.
    ///
    /// When enabled, the application will receive `FocusMsg` and `BlurMsg`
    /// when the terminal gains or loses focus.
    pub fn report_focus(mut self, enabled: bool) -> Self {
        self.config.report_focus = enabled;
        self
    }

    /// Sets the target frames per second for rendering.
    ///
    /// This controls how often the `view` method of the model is called and
    /// the terminal is updated.
    ///
    /// # Arguments
    ///
    /// * `fps` - The target frames per second.
    pub fn with_fps(mut self, fps: u32) -> Self {
        self.config.fps = fps;
        self
    }

    /// Disables the renderer.
    ///
    /// When disabled, the `view` method will not be called and no output
    /// will be rendered to the terminal. This is useful for testing or
    /// headless operations.
    pub fn without_renderer(mut self) -> Self {
        self.config.without_renderer = true;
        self
    }

    /// Sets whether to catch panics.
    ///
    /// When enabled, application panics will be caught and converted into
    /// `ProgramPanic` errors, allowing for graceful shutdown.
    pub fn catch_panics(mut self, enabled: bool) -> Self {
        self.config.catch_panics = enabled;
        self
    }

    /// Sets whether to enable signal handling.
    ///
    /// When enabled, the `Program` will listen for OS signals (e.g., Ctrl+C)
    /// and attempt a graceful shutdown.
    pub fn signal_handler(mut self, enabled: bool) -> Self {
        self.config.signal_handler = enabled;
        self
    }

    /// Sets whether to enable bracketed paste mode.
    ///
    /// When enabled, pasted text will be wrapped in special escape sequences,
    /// allowing the application to distinguish pasted input from typed input.
    pub fn bracketed_paste(mut self, enabled: bool) -> Self {
        self.config.bracketed_paste = enabled;
        self
    }

    /// Configures the program to use the default terminal input (stdin).
    pub fn input_tty(self) -> Self {
        // No-op for now, as stdin is used by default
        self
    }

    /// Sets a custom input reader for the program.
    ///
    /// # Arguments
    ///
    /// * `reader` - A custom input stream that implements `tokio::io::AsyncRead + Send + Unpin`.
    pub fn input(mut self, reader: impl tokio::io::AsyncRead + Send + Unpin + 'static) -> Self {
        self.config.input_source = Some(InputSource::Custom(Box::pin(reader)));
        self
    }

    /// Sets a custom output writer for the program.
    ///
    /// # Arguments
    ///
    /// * `writer` - A custom output stream that implements `tokio::io::AsyncWrite + Send + Unpin`.
    pub fn output(mut self, writer: impl AsyncWrite + Send + Unpin + 'static) -> Self {
        self.config.output_writer = Some(Arc::new(Mutex::new(Box::new(writer))));
        self
    }

    /// Sets an external cancellation token for the program.
    ///
    /// When the token is cancelled, the program's event loop will gracefully shut down.
    ///
    /// # Arguments
    ///
    /// * `token` - The `CancellationToken` to use for external cancellation.
    pub fn context(mut self, token: CancellationToken) -> Self {
        self.config.cancellation_token = Some(token);
        self
    }

    /// Sets a message filter function.
    ///
    /// The provided closure will be called for each incoming message, allowing
    /// for message transformation or filtering.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a `Msg` and returns an `Option<Msg>`.
    pub fn filter(mut self, f: impl Fn(Msg) -> Option<Msg> + Send + 'static) -> Self {
        self.config.message_filter = Some(Box::new(f));
        self
    }

    /// Builds the `Program` instance with the configured options.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Program` instance or an `Error` if building fails.
    pub fn build(self) -> Result<Program<M>, Error> {
        Program::new(self.config)
    }
}

/// The main `Program` struct that coordinates the application.
///
/// The `Program` is responsible for setting up the terminal, managing the
/// event loop, executing commands, and rendering the model's view.
pub struct Program<M: Model> {
    /// The configuration for this `Program` instance.
    pub config: ProgramConfig,
    event_tx: mpsc::UnboundedSender<Msg>,
    event_rx: mpsc::UnboundedReceiver<Msg>,
    terminal: Option<Box<dyn TerminalInterface + Send>>,
    _phantom: PhantomData<M>,
}

impl<M: Model> Program<M> {
    /// Creates a new `ProgramBuilder` for configuring and building a `Program`.
    pub fn builder() -> ProgramBuilder<M> {
        ProgramBuilder::new()
    }

    /// Creates a new `Program` instance with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The `ProgramConfig` to use for this program.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Program` instance or an `Error` if initialization fails.
    fn new(config: ProgramConfig) -> Result<Self, Error> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let terminal = if config.without_renderer {
            None
        } else {
            let output_writer_for_terminal = config.output_writer.clone();
            Some(Box::new(Terminal::new(output_writer_for_terminal)?)
                as Box<dyn TerminalInterface + Send>)
        };

        // Expose the event sender globally for command helpers
        let _ = crate::event::EVENT_SENDER.set(event_tx.clone());

        Ok(Self {
            config,
            event_tx,
            event_rx,
            terminal,
            _phantom: PhantomData,
        })
    }

    /// Runs the `bubbletea-rs` application.
    ///
    /// This method initializes the terminal, starts the event loop, and manages
    /// the application's lifecycle. It will continue to run until a `QuitMsg`
    /// is received or an unrecoverable error occurs.
    ///
    /// # Returns
    ///
    /// A `Result` containing the final `Model` state or an `Error` if the program
    /// terminates abnormally.
    pub async fn run(mut self) -> Result<M, Error> {
        // Set up panic hook
        if self.config.catch_panics {
            let event_tx = self.event_tx.clone();
            ORIGINAL_PANIC_HOOK.get_or_init(|| panic::take_hook());

            panic::set_hook(Box::new(move |panic_info| {
                let payload = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                    s.clone()
                } else {
                    "<unknown panic>".to_string()
                };
                let _ = event_tx.send(Box::new(crate::Error::ProgramPanic(payload)) as Msg);

                // Call the original hook if it exists
                if let Some(hook) = ORIGINAL_PANIC_HOOK.get() {
                    hook(panic_info);
                }
            }));
        }

        // Setup terminal
        if let Some(terminal) = &mut self.terminal {
            terminal.enter_raw_mode().await?;
            if self.config.alt_screen {
                terminal.enter_alt_screen().await?;
            }
            match self.config.mouse_motion {
                MouseMotion::Cell => terminal.enable_mouse_cell_motion().await?,
                MouseMotion::All => terminal.enable_mouse_all_motion().await?,
                MouseMotion::None => (),
            }
            if self.config.report_focus {
                terminal.enable_focus_reporting().await?;
            }
            if self.config.bracketed_paste {
                terminal.enable_bracketed_paste().await?;
            }
            terminal.hide_cursor().await?;
        }

        let (mut model, mut cmd) = M::init();

        // Setup input handling - either terminal input or custom input source
        if self.terminal.is_some() || self.config.input_source.is_some() {
            let input_source = self.config.input_source.take();
            let input_handler = if let Some(source) = input_source {
                InputHandler::with_source(self.event_tx.clone(), source)
            } else {
                InputHandler::new(self.event_tx.clone())
            };
            tokio::spawn(async move {
                let _ = input_handler.run().await;
            });
        }

        let result = loop {
            if let Some(c) = cmd.take() {
                let event_tx = self.event_tx.clone();
                tokio::spawn(async move {
                    if let Some(msg) = c.await {
                        let _ = event_tx.send(msg);
                    }
                });
            }

            select! {
                _ = self.config.cancellation_token.as_ref().map_or(futures::future::pending().left_future(), |token| token.cancelled().right_future()).fuse() => {
                    break Ok(model); // External cancellation
                }
                event = self.event_rx.recv().fuse() => {
                    if let Some(mut msg) = event {
                        if let Some(filter_fn) = &self.config.message_filter {
                            if let Some(filtered_msg) = filter_fn(msg) {
                                msg = filtered_msg;
                            } else {
                                continue; // Message was filtered out
                            }
                        }
                        // Check for special internal messages
                        let mut should_quit = false;

                        // Handle special internal messages that need to consume the message
                        if msg.is::<crate::event::ClearScreenMsg>() {
                            if let Some(terminal) = &mut self.terminal {
                                let _ = terminal.clear().await;
                            }
                            continue; // handled; don't pass to the model
                        } else if msg.is::<crate::event::EnterAltScreenMsg>() {
                            if let Some(terminal) = &mut self.terminal {
                                let _ = terminal.enter_alt_screen().await;
                            }
                            // Intentionally do not continue; allow render below to redraw view
                        } else if msg.is::<crate::event::ExitAltScreenMsg>() {
                            if let Some(terminal) = &mut self.terminal {
                                let _ = terminal.exit_alt_screen().await;
                            }
                            // Intentionally do not continue; allow render below to redraw view
                        } else if msg.is::<crate::event::EveryMsgInternal>() {
                            // We need to consume the message to get ownership of the function
                            if let Ok(every_msg) = msg.downcast::<crate::event::EveryMsgInternal>() {
                                let duration = every_msg.duration;
                                let func = every_msg.func;
                                let event_tx = self.event_tx.clone();

                                tokio::spawn(async move {
                                    let mut ticker = tokio::time::interval(duration);
                                    ticker.tick().await; // First tick completes immediately

                                    loop {
                                        ticker.tick().await;
                                        let msg = func(duration);
                                        if event_tx.send(msg).is_err() {
                                            break; // Receiver dropped
                                        }
                                    }
                                });
                                continue; // Don't pass this to the model
                            }
                        } else if msg.is::<crate::event::BatchMsgInternal>() {
                            if let Ok(batch_msg) = msg.downcast::<crate::event::BatchMsgInternal>() {
                                // Process each message in the batch and accumulate resulting cmds
                                let mut next_cmds: Vec<crate::command::Cmd> = Vec::new();
                                for batch_item in batch_msg.messages {
                                    if batch_item.downcast_ref::<QuitMsg>().is_some() {
                                        should_quit = true;
                                    }
                                    if let Some(new_cmd) = model.update(batch_item) {
                                        next_cmds.push(new_cmd);
                                    }
                                }
                                if !next_cmds.is_empty() {
                                    cmd = Some(crate::command::batch(next_cmds));
                                }
                            }
                        } else {
                            // Handle regular messages
                            let is_quit = msg.downcast_ref::<QuitMsg>().is_some();
                            cmd = model.update(msg);
                            if is_quit {
                                should_quit = true;
                            }
                        }
                        if should_quit {
                            break Ok(model);
                        }
                        if let Some(terminal) = &mut self.terminal {
                            let view = model.view();
                            terminal.render(&view).await?;
                        }
                    } else {
                        break Err(Error::ChannelReceive);
                    }
                }
                _ = async {
                    if self.config.signal_handler {
                        tokio::signal::ctrl_c().await.ok();
                    } else {
                        futures::future::pending::<()>().await;
                    }
                }.fuse() => {
                    let _ = self.event_tx.send(Box::new(crate::InterruptMsg));
                }
            }
        };

        // Restore terminal state on exit
        if let Some(terminal) = &mut self.terminal {
            let _ = terminal.show_cursor().await;
            let _ = terminal.disable_mouse().await;
            let _ = terminal.disable_focus_reporting().await;
            if self.config.alt_screen {
                let _ = terminal.exit_alt_screen().await;
            }
            let _ = terminal.exit_raw_mode().await;
        }

        result
    }

    /// Returns a sender that can be used to send messages to the `Program`'s event loop.
    ///
    /// This is useful for sending messages from outside the `Model`'s `update` method,
    /// for example, from asynchronous tasks or other threads.
    ///
    /// # Returns
    ///
    /// An `mpsc::UnboundedSender<Msg>` that can be used to send messages.
    pub fn sender(&self) -> mpsc::UnboundedSender<Msg> {
        self.event_tx.clone()
    }

    /// Sends a message to the `Program`'s event loop.
    ///
    /// This is a convenience method that wraps the `sender()` method.
    ///
    /// # Arguments
    ///
    /// * `msg` - The `Msg` to send.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or a `SendError` if the message could not be sent.
    pub fn send(&self, msg: Msg) -> Result<(), Error> {
        self.event_tx.send(msg).map_err(|_| Error::SendError)
    }

    /// Sends a `QuitMsg` to the `Program`'s event loop, initiating a graceful shutdown.
    pub fn quit(&self) {
        let _ = self.event_tx.send(Box::new(QuitMsg));
    }

    /// Sends a `QuitMsg` to the `Program`'s event loop, initiating a forceful shutdown.
    ///
    /// Currently, this is identical to `quit()`, but may be extended in the future
    /// to handle more aggressive termination.
    pub fn kill(&self) {
        let _ = self.event_tx.send(Box::new(QuitMsg));
    }

    /// Waits for the `Program` to finish execution.
    ///
    /// This method blocks until the program's event loop has exited.
    /// Note: This is currently a no-op since the Program is consumed by run().
    /// In a real implementation, you'd need to track the program's state separately.
    pub async fn wait(&self) {
        // Since the Program is consumed by run(), we can't really wait for it.
        // This would need a different architecture to implement properly,
        // similar to how Go's context.Context works with goroutines.
        tokio::task::yield_now().await;
    }

    /// Releases control of the terminal.
    ///
    /// This method restores the terminal to its original state, disabling raw mode,
    /// exiting alternate screen, disabling mouse and focus reporting, and showing the cursor.
    pub async fn release_terminal(&mut self) -> Result<(), Error> {
        if let Some(terminal) = &mut self.terminal {
            terminal.exit_raw_mode().await?;
            terminal.exit_alt_screen().await?;
            terminal.disable_mouse().await?;
            terminal.disable_focus_reporting().await?;
            terminal.show_cursor().await?;
        }
        Ok(())
    }

    /// Restores control of the terminal.
    ///
    /// This method re-initializes the terminal based on the `ProgramConfig`,
    /// enabling raw mode, entering alternate screen, enabling mouse and focus reporting,
    /// and hiding the cursor.
    pub async fn restore_terminal(&mut self) -> Result<(), Error> {
        if let Some(terminal) = &mut self.terminal {
            terminal.enter_raw_mode().await?;
            if self.config.alt_screen {
                terminal.enter_alt_screen().await?;
            }
            match self.config.mouse_motion {
                MouseMotion::Cell => terminal.enable_mouse_cell_motion().await?,
                MouseMotion::All => terminal.enable_mouse_all_motion().await?,
                MouseMotion::None => (),
            }
            if self.config.report_focus {
                terminal.enable_focus_reporting().await?;
            }
            if self.config.bracketed_paste {
                terminal.enable_bracketed_paste().await?;
            }
            terminal.hide_cursor().await?;
        }
        Ok(())
    }

    /// Prints a line to the terminal without going through the renderer.
    ///
    /// This is useful for debugging or for outputting messages that shouldn't
    /// be part of the managed UI.
    pub async fn println(&mut self, s: String) -> Result<(), Error> {
        if let Some(_terminal) = &mut self.terminal {
            use std::io::Write;
            println!("{}", s);
            std::io::stdout().flush()?;
        }
        Ok(())
    }

    /// Prints formatted text to the terminal without going through the renderer.
    ///
    /// This is useful for debugging or for outputting messages that shouldn't
    /// be part of the managed UI.
    pub async fn printf(&mut self, s: String) -> Result<(), Error> {
        if let Some(_terminal) = &mut self.terminal {
            use std::io::Write;
            print!("{}", s);
            std::io::stdout().flush()?;
        }
        Ok(())
    }
}
