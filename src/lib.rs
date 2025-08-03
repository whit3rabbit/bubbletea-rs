//! # bubbletea-rs
//!
//! A comprehensive Rust implementation of the Bubble Tea TUI framework.
//! This library provides developers with the tools to build interactive terminal
//! applications using the Model-View-Update (MVU) architecture pattern.
//!
//! ## Features
//!
//! - **Model-View-Update Architecture**: Clean separation of state, logic, and rendering
//! - **Async Command System**: Non-blocking operations with command-based side effects
//! - **Terminal Interface Abstraction**: Works with real terminals and test environments
//! - **Comprehensive Event Handling**: Keyboard, mouse, window resize, and focus events
//! - **Memory Monitoring**: Built-in memory usage tracking and leak detection
//! - **Gradient Rendering**: Rich color gradients for progress bars and visual elements
//! - **Flexible Input Sources**: Support for different input mechanisms and testing
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use bubbletea_rs::{Model, Program, Msg, Cmd};
//!
//! struct MyModel {
//!     counter: i32,
//! }
//!
//! impl Model for MyModel {
//!     fn init() -> (Self, Option<Cmd>) {
//!         (Self { counter: 0 }, None)
//!     }
//!
//!     fn update(&mut self, _msg: Msg) -> Option<Cmd> {
//!         None
//!     }
//!
//!     fn view(&self) -> String {
//!         format!("Counter: {}", self.counter)
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let program = Program::<MyModel>::builder().build()?;
//!     program.run().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture Overview
//!
//! The library follows the Elm Architecture pattern:
//!
//! 1. **Model**: Your application state implementing the `Model` trait
//! 2. **Messages**: Events that trigger state changes (keyboard, mouse, timers, etc.)
//! 3. **Update**: Process messages and optionally issue commands
//! 4. **View**: Render your model as a string for terminal display
//! 5. **Commands**: Async operations that can produce future messages
//!
//! ## Memory Safety
//!
//! The framework includes built-in memory monitoring to help detect leaks and optimize
//! performance. Use the `MemoryMonitor` to track allocations in your applications.
//!
//! ## Testing
//!
//! Testing is supported through the `DummyTerminal` which allows you to test your
//! applications without requiring an actual terminal interface.

#![warn(missing_docs)]

/// Commands for async operations that produce messages.
pub mod command;
/// Error types and handling.
pub mod error;
/// Event types and message passing system.
pub mod event;
/// Gradient rendering utilities for progress bars and color transitions.
pub mod gradient;
/// Input handling abstraction for different sources.
pub mod input;
/// Logging utilities for debugging and monitoring.
pub mod logging;
/// Memory monitoring and leak detection.
pub mod memory;
/// The core Model trait defining application behavior.
pub mod model;
/// Program runtime and builder for TUI applications.
pub mod program;
/// Terminal interface abstraction and implementations.
pub mod terminal;

pub use command::{
    batch, cancel_all_timers, cancel_timer, clear_screen, disable_bracketed_paste, disable_mouse,
    disable_report_focus, enable_bracketed_paste, enable_mouse_all_motion,
    enable_mouse_cell_motion, enable_report_focus, enter_alt_screen, every, every_with_id,
    exec_process, exit_alt_screen, hide_cursor, interrupt, printf, println, quit, sequence,
    set_window_title, show_cursor, suspend, tick, window_size, Batch, Cmd,
};
pub use error::Error;
pub use event::{
    BatchMsgInternal, BlurMsg, CancelAllTimersMsg, CancelTimerMsg, ClearScreenMsg,
    DisableBracketedPasteMsg, DisableMouseMsg, DisableReportFocusMsg, EnableBracketedPasteMsg,
    EnableMouseAllMotionMsg, EnableMouseCellMotionMsg, EnableReportFocusMsg, EnterAltScreenMsg,
    EventReceiver, EventSender, ExitAltScreenMsg, FocusMsg, HideCursorMsg, InterruptMsg, KeyMsg,
    KillMsg, MouseMsg, Msg, PasteMsg, PrintMsg, PrintfMsg, QuitMsg, RequestWindowSizeMsg,
    ResumeMsg, SetWindowTitleMsg, ShowCursorMsg, SuspendMsg, WindowSizeMsg,
};
pub use gradient::{
    charm_default_gradient, gradient_filled_segment, gradient_filled_segment_with_buffer, lerp_rgb,
};
pub use input::{InputHandler, InputSource};
pub use memory::{MemoryHealth, MemoryMonitor, MemorySnapshot};
pub use model::Model;
pub use program::{MouseMotion, Program, ProgramBuilder, ProgramConfig};
pub use terminal::{DummyTerminal, Terminal, TerminalInterface};

#[cfg(feature = "logging")]
pub use logging::log_to_file;

pub mod prelude {
    //! Convenient re-exports of the most commonly used types.

    pub use crate::{Cmd, Error, Model, Msg, Program};
    pub use crate::{KeyMsg, KillMsg, MouseMsg, PasteMsg, QuitMsg, WindowSizeMsg};

    #[cfg(feature = "logging")]
    pub use crate::log_to_file;
}
