//! Stopwatch Example
//!
//! A direct port of the Go Bubble Tea stopwatch example demonstrating:
//! - `bubbletea-widgets::stopwatch` for count-up timers
//! - `bubbletea-widgets::key` for organized key binding management  
//! - `bubbletea-widgets::help` for automatic help text generation
//! - Start/stop/reset functionality
//!
//! This example closely mirrors `bubbletea/examples/stopwatch/main.go` behavior:
//! - Stopwatch that starts automatically with millisecond precision
//! - Toggle start/stop with 's' (key binding changes based on state)
//! - Reset to 00:00 with 'r'
//! - Quit with 'q' or Ctrl+C

use bubbletea_rs::{quit, Cmd, KeyMsg, Model as BubbleTeaModel, Msg, Program};
use bubbletea_widgets::key::{new_binding, with_keys_str, with_help, matches_binding, Binding, KeyMap};
use bubbletea_widgets::stopwatch::{new_with_interval, Model as StopwatchModel};
use bubbletea_widgets::help::{Model as HelpModel, KeyMap as HelpKeyMap};
use std::time::Duration;

/// Formats a duration to match Go's time.Duration.String() format for stopwatch display
fn format_duration_like_go(d: Duration) -> String {
    let total_secs = d.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    let millis = d.subsec_millis();
    
    if hours > 0 {
        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
    } else {
        format!("{:02}:{:02}.{:03}", minutes, seconds, millis)
    }
}

/// Main model matching Go's model struct
pub struct Model {
    stopwatch: StopwatchModel,
    keymap: Keymap,
    help: HelpModel,
    quitting: bool,
}

/// Key bindings struct matching Go's keymap
#[derive(Debug, Clone)]
pub struct Keymap {
    pub start: Binding,
    pub stop: Binding,  
    pub reset: Binding,
    pub quit: Binding,
}

impl Keymap {
    pub fn new() -> Self {
        Self {
            start: new_binding(vec![
                with_keys_str(&["s"]),
                with_help("s", "start"),
            ]),
            stop: new_binding(vec![
                with_keys_str(&["s"]),
                with_help("s", "stop"),
            ]),
            reset: new_binding(vec![
                with_keys_str(&["r"]),
                with_help("r", "reset"),
            ]),
            quit: new_binding(vec![
                with_keys_str(&["ctrl+c", "q"]),
                with_help("q", "quit"),
            ]),
        }
    }
}

// Implement KeyMap trait for Keymap to provide help information
impl KeyMap for Keymap {
    fn short_help(&self) -> Vec<&Binding> {
        // Return bindings that are currently enabled
        let mut bindings = Vec::new();
        if self.start.enabled() {
            bindings.push(&self.start);
        }
        if self.stop.enabled() {
            bindings.push(&self.stop);
        }
        bindings.push(&self.reset);
        bindings.push(&self.quit);
        bindings
    }
    
    fn full_help(&self) -> Vec<Vec<&Binding>> {
        // Not used in this example - short help only
        vec![self.short_help()]
    }
}

impl Model {
    pub fn new() -> Self {
        let mut keymap = Keymap::new();
        
        // Match Go's initial state: start is disabled since stopwatch starts running
        keymap.start.set_enabled(false);
        
        // Create help model
        let mut help = HelpModel::new();
        help.show_all = false; // Use short help like Go version
        
        Self {
            // Match Go's NewWithInterval(time.Millisecond) 
            stopwatch: new_with_interval(Duration::from_millis(1)),
            keymap,
            help,
            quitting: false,
        }
    }
}

// Implement HelpKeyMap trait to connect our keymap to the help widget
impl HelpKeyMap for Model {
    fn short_help(&self) -> Vec<&Binding> {
        self.keymap.short_help()
    }
    
    fn full_help(&self) -> Vec<Vec<&Binding>> {
        self.keymap.full_help()
    }
}

impl BubbleTeaModel for Model {
    fn init() -> (Self, Option<Cmd>) {
        let model = Self::new();
        // Match Go's m.stopwatch.Init()
        let cmd = model.stopwatch.init();
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle key messages first (matching Go's switch order)
        if let Some(key) = msg.downcast_ref::<KeyMsg>() {
            if matches_binding(key, &self.keymap.quit) {
                self.quitting = true;
                return Some(quit());
            } else if matches_binding(key, &self.keymap.reset) {
                // Match Go's m.stopwatch.Reset()
                return Some(self.stopwatch.reset());
            } else if matches_binding(key, &self.keymap.start) || matches_binding(key, &self.keymap.stop) {
                // Match Go's key enabling logic
                self.keymap.stop.set_enabled(!self.stopwatch.running());
                self.keymap.start.set_enabled(self.stopwatch.running());
                // Match Go's m.stopwatch.Toggle()
                return Some(self.stopwatch.toggle());
            }
        }
        
        // Handle stopwatch messages (ticks, etc.)
        // Match Go's: m.stopwatch, cmd = m.stopwatch.Update(msg)
        self.stopwatch.update(msg)
    }

    fn view(&self) -> String {
        // Match Go's View() method exactly
        
        // Note: you could further customize the time output by getting the
        // duration from m.stopwatch.Elapsed(), which returns a time.Duration, and
        // skip m.stopwatch.View() altogether.
        let mut s = format_duration_like_go(self.stopwatch.elapsed());
        s.push('\n');
        
        if !self.quitting {
            s = format!("Elapsed: {}", s);
            s.push_str(&self.help.view(self));
        }
        
        s
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<Model>::builder()
        .signal_handler(true)
        .build()?;

    program.run().await?;
    Ok(())
}