//! Terminal management for bubbletea-rs.

use crate::Error;
use crossterm::{
    cursor::{Hide, Show},
    event::{
        DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture,
    },
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};
use std::sync::Arc;
use tokio::io::AsyncWrite;
use tokio::sync::Mutex;

/// A trait for abstracting terminal operations.
#[async_trait::async_trait]
pub trait TerminalInterface {
    fn new(output_writer: Option<Arc<Mutex<dyn AsyncWrite + Send + Unpin>>>) -> Result<Self, Error>
    where
        Self: Sized;
    async fn enter_raw_mode(&mut self) -> Result<(), Error>;
    async fn exit_raw_mode(&mut self) -> Result<(), Error>;
    async fn enter_alt_screen(&mut self) -> Result<(), Error>;
    async fn exit_alt_screen(&mut self) -> Result<(), Error>;
    async fn enable_mouse(&mut self) -> Result<(), Error>;
    async fn enable_mouse_cell_motion(&mut self) -> Result<(), Error>;
    async fn enable_mouse_all_motion(&mut self) -> Result<(), Error>;
    async fn disable_mouse(&mut self) -> Result<(), Error>;
    async fn enable_focus_reporting(&mut self) -> Result<(), Error>;
    async fn disable_focus_reporting(&mut self) -> Result<(), Error>;
    async fn enable_bracketed_paste(&mut self) -> Result<(), Error>;
    async fn disable_bracketed_paste(&mut self) -> Result<(), Error>;
    async fn show_cursor(&mut self) -> Result<(), Error>;
    async fn hide_cursor(&mut self) -> Result<(), Error>;
    async fn clear(&mut self) -> Result<(), Error>;
    async fn render(&mut self, content: &str) -> Result<(), Error>;
    fn size(&self) -> Result<(u16, u16), Error>;
}

/// Terminal state manager.
pub struct Terminal {
    raw_mode: bool,
    alt_screen: bool,
    mouse_enabled: bool,
    focus_reporting: bool,
    cursor_visible: bool,
    output_writer: Option<Arc<Mutex<dyn AsyncWrite + Send + Unpin>>>,
    /// Reusable buffer for string operations to minimize allocations
    render_buffer: String,
}

impl Terminal {
    pub fn new(
        output_writer: Option<Arc<Mutex<dyn AsyncWrite + Send + Unpin>>>,
    ) -> Result<Self, Error> {
        Ok(Self {
            raw_mode: false,
            alt_screen: false,
            mouse_enabled: false,
            focus_reporting: false,
            cursor_visible: true,
            output_writer,
            render_buffer: String::with_capacity(8192), // Pre-allocate 8KB buffer
        })
    }
}

#[async_trait::async_trait]
impl TerminalInterface for Terminal {
    fn new(output_writer: Option<Arc<Mutex<dyn AsyncWrite + Send + Unpin>>>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            raw_mode: false,
            alt_screen: false,
            mouse_enabled: false,
            focus_reporting: false,
            cursor_visible: true,
            output_writer,
            render_buffer: String::with_capacity(8192),
        })
    }

    async fn enter_raw_mode(&mut self) -> Result<(), Error> {
        if !self.raw_mode {
            terminal::enable_raw_mode()?;
            self.raw_mode = true;
        }
        Ok(())
    }

    async fn exit_raw_mode(&mut self) -> Result<(), Error> {
        if self.raw_mode {
            terminal::disable_raw_mode()?;
            self.raw_mode = false;
        }
        Ok(())
    }

    async fn enter_alt_screen(&mut self) -> Result<(), Error> {
        if !self.alt_screen {
            execute!(io::stdout(), EnterAlternateScreen)?;
            // Clear the alternate screen buffer immediately after entering
            execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
            io::stdout().flush()?;
            self.alt_screen = true;
        }
        Ok(())
    }

    async fn exit_alt_screen(&mut self) -> Result<(), Error> {
        if self.alt_screen {
            execute!(io::stdout(), LeaveAlternateScreen)?;
            io::stdout().flush()?;
            self.alt_screen = false;
        }
        Ok(())
    }

    async fn enable_mouse(&mut self) -> Result<(), Error> {
        if !self.mouse_enabled {
            execute!(io::stdout(), EnableMouseCapture)?;
            self.mouse_enabled = true;
        }
        Ok(())
    }

    async fn enable_mouse_cell_motion(&mut self) -> Result<(), Error> {
        self.enable_mouse().await
    }

    async fn enable_mouse_all_motion(&mut self) -> Result<(), Error> {
        self.enable_mouse().await
    }

    async fn disable_mouse(&mut self) -> Result<(), Error> {
        if self.mouse_enabled {
            execute!(io::stdout(), DisableMouseCapture)?;
            self.mouse_enabled = false;
        }
        Ok(())
    }

    async fn enable_focus_reporting(&mut self) -> Result<(), Error> {
        if !self.focus_reporting {
            execute!(io::stdout(), EnableFocusChange)?;
            self.focus_reporting = true;
        }
        Ok(())
    }

    async fn disable_focus_reporting(&mut self) -> Result<(), Error> {
        if self.focus_reporting {
            execute!(io::stdout(), DisableFocusChange)?;
            self.focus_reporting = false;
        }
        Ok(())
    }

    async fn enable_bracketed_paste(&mut self) -> Result<(), Error> {
        execute!(io::stdout(), EnableBracketedPaste)?;
        Ok(())
    }

    async fn disable_bracketed_paste(&mut self) -> Result<(), Error> {
        execute!(io::stdout(), DisableBracketedPaste)?;
        Ok(())
    }

    async fn show_cursor(&mut self) -> Result<(), Error> {
        if !self.cursor_visible {
            execute!(io::stdout(), Show)?;
            self.cursor_visible = true;
        }
        Ok(())
    }

    async fn hide_cursor(&mut self) -> Result<(), Error> {
        if self.cursor_visible {
            execute!(io::stdout(), Hide)?;
            self.cursor_visible = false;
        }
        Ok(())
    }

    async fn clear(&mut self) -> Result<(), Error> {
        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    async fn render(&mut self, content: &str) -> Result<(), Error> {
        use crossterm::cursor::MoveTo;
        use crossterm::terminal::{Clear, ClearType};

        if let Some(writer) = &mut self.output_writer {
            use tokio::io::AsyncWriteExt;

            // Pre-allocate buffer for efficient rendering
            self.render_buffer.clear();

            // Reserve space for the clear sequence plus content
            let estimated_size = 8 + content.len() + content.chars().filter(|&c| c == '\n').count();
            self.render_buffer.reserve(estimated_size);

            // Add clear sequence
            self.render_buffer.push_str("\x1b[H\x1b[2J");

            // Efficiently replace newlines by iterating through chars
            for ch in content.chars() {
                if ch == '\n' {
                    self.render_buffer.push_str("\r\n");
                } else {
                    self.render_buffer.push(ch);
                }
            }

            writer
                .lock()
                .await
                .write_all(self.render_buffer.as_bytes())
                .await?;
            writer.lock().await.flush().await?;
        } else {
            // Move cursor to top-left and clear entire screen
            execute!(io::stdout(), MoveTo(0, 0))?;
            execute!(io::stdout(), Clear(ClearType::All))?;

            // Pre-allocate buffer for efficient rendering
            self.render_buffer.clear();

            // Reserve space for content plus newline replacements
            let estimated_size = content.len() + content.chars().filter(|&c| c == '\n').count();
            self.render_buffer.reserve(estimated_size);

            // Efficiently replace newlines by iterating through chars
            for ch in content.chars() {
                if ch == '\n' {
                    self.render_buffer.push_str("\r\n");
                } else {
                    self.render_buffer.push(ch);
                }
            }

            print!("{}", self.render_buffer);
            io::stdout().flush()?;
        }
        Ok(())
    }

    fn size(&self) -> Result<(u16, u16), Error> {
        let (width, height) = terminal::size()?;
        Ok((width, height))
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        if !self.cursor_visible {
            let _ = execute!(io::stdout(), Show);
        }
        if self.mouse_enabled {
            let _ = execute!(io::stdout(), DisableMouseCapture);
        }
        if self.focus_reporting {
            let _ = execute!(io::stdout(), DisableFocusChange);
        }
        if self.alt_screen {
            let _ = execute!(io::stdout(), LeaveAlternateScreen);
            let _ = io::stdout().flush();
        }
        if self.raw_mode {
            let _ = terminal::disable_raw_mode();
        }
    }
}

pub struct DummyTerminal {
    output_writer: Option<Arc<Mutex<dyn AsyncWrite + Send + Unpin>>>,
}

#[async_trait::async_trait]
impl TerminalInterface for DummyTerminal {
    fn new(
        output_writer: Option<Arc<Mutex<dyn AsyncWrite + Send + Unpin>>>,
    ) -> Result<Self, Error> {
        Ok(Self { output_writer })
    }
    async fn enter_raw_mode(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn exit_raw_mode(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn enter_alt_screen(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn exit_alt_screen(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn enable_mouse(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn enable_mouse_cell_motion(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn enable_mouse_all_motion(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn disable_mouse(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn enable_focus_reporting(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn disable_focus_reporting(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn enable_bracketed_paste(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn disable_bracketed_paste(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn show_cursor(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn hide_cursor(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn clear(&mut self) -> Result<(), Error> {
        Ok(())
    }
    async fn render(&mut self, content: &str) -> Result<(), Error> {
        if let Some(writer) = &mut self.output_writer {
            use tokio::io::AsyncWriteExt;
            writer.lock().await.write_all(content.as_bytes()).await?;
            writer.lock().await.flush().await?;
        }
        Ok(())
    }
    fn size(&self) -> Result<(u16, u16), Error> {
        Ok((0, 0))
    }
}
