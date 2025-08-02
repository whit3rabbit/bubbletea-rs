//! # bubbletea-rs
//!
//! A comprehensive Rust implementation of the Bubble Tea TUI framework.
//! This library provides developers with the tools to build interactive terminal
//! applications using the Model-View-Update (MVU) architecture pattern.
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

pub mod command;
pub mod error;
pub mod event;
pub mod input;
pub mod logging;
pub mod model;
pub mod program;
pub mod terminal;
pub mod gradient;

pub use command::{
    batch, clear_screen, disable_bracketed_paste, disable_mouse, disable_report_focus,
    enable_bracketed_paste, enable_mouse_all_motion, enable_mouse_cell_motion, enable_report_focus,
    enter_alt_screen, every, exec_process, exit_alt_screen, hide_cursor, interrupt, printf,
    println, quit, sequence, set_window_title, show_cursor, suspend, tick, window_size, Batch, Cmd,
};
pub use error::Error;
pub use event::{
    BatchMsgInternal, BlurMsg, ClearScreenMsg, DisableBracketedPasteMsg, DisableMouseMsg,
    DisableReportFocusMsg, EnableBracketedPasteMsg, EnableMouseAllMotionMsg,
    EnableMouseCellMotionMsg, EnableReportFocusMsg, EnterAltScreenMsg, ExitAltScreenMsg, FocusMsg,
    HideCursorMsg, InterruptMsg, KeyMsg, MouseMsg, Msg, PrintMsg, PrintfMsg, QuitMsg,
    RequestWindowSizeMsg, ResumeMsg, SetWindowTitleMsg, ShowCursorMsg, SuspendMsg, WindowSizeMsg,
};
pub use input::{InputHandler, InputSource};
pub use model::Model;
pub use program::{MouseMotion, Program, ProgramBuilder, ProgramConfig};
pub use terminal::{DummyTerminal, Terminal, TerminalInterface};

#[cfg(feature = "logging")]
pub use logging::log_to_file;

pub mod prelude {
    //! Convenient re-exports of the most commonly used types.

    pub use crate::{Cmd, Error, Model, Msg, Program};
    pub use crate::{KeyMsg, MouseMsg, QuitMsg, WindowSizeMsg};

    #[cfg(feature = "logging")]
    pub use crate::log_to_file;
}
