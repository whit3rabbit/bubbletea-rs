//! Timer Example
//!
//! A direct port of the Go Bubble Tea timer example demonstrating:
//! - `bubbletea-widgets::timer` for precise countdown timers
//! - `bubbletea-widgets::key` for organized key binding management  
//! - `bubbletea-widgets::help` for automatic help text generation
//! - Automatic timer timeout and quit functionality
//!
//! This example closely mirrors `bubbletea/examples/timer/main.go` behavior:
//! - 5s countdown that starts automatically with millisecond precision
//! - Toggle start/stop with 's' (key binding changes based on state)
//! - Reset to full timeout with 'r'
//! - Quit with 'q' or Ctrl+C
//! - Automatic quit when timer reaches zero

use bubbletea_rs::{quit, Cmd, KeyMsg, Model as BubbleTeaModel, Msg, Program};
use bubbletea_widgets::help::{KeyMap as HelpKeyMap, Model as HelpModel};
use bubbletea_widgets::key::{
    matches_binding, new_binding, with_help, with_keys_str, Binding, KeyMap,
};
use bubbletea_widgets::timer::{
    new_with_interval, Model as TimerModel, StartStopMsg, TickMsg, TimeoutMsg,
};
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(5);

/// Formats a duration to match Go's time.Duration.String() format exactly
fn format_duration_like_go(d: Duration) -> String {
    let total_nanos = d.as_nanos();

    if total_nanos == 0 {
        return "0s".to_string();
    }

    if total_nanos >= 1_000_000_000 {
        // Seconds or more
        let secs = d.as_secs_f64();
        if secs >= 60.0 {
            let minutes = (secs / 60.0) as u64;
            let remaining_secs = secs % 60.0;
            if remaining_secs == 0.0 {
                format!("{}m", minutes)
            } else {
                format!("{}m{:.0}s", minutes, remaining_secs)
            }
        } else {
            // For the timer example, we want to show seconds with 3 decimal places for precision
            // This matches the Go timer example which shows "4.999s", "4.998s", etc.
            format!("{:.3}s", secs)
        }
    } else if total_nanos >= 1_000_000 {
        // Milliseconds
        format!("{}ms", d.as_millis())
    } else if total_nanos >= 1_000 {
        // Microseconds
        format!("{}µs", d.as_micros())
    } else {
        // Nanoseconds
        format!("{}ns", total_nanos)
    }
}

/// Main model matching Go's model struct
pub struct Model {
    timer: TimerModel,
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
            start: new_binding(vec![with_keys_str(&["s"]), with_help("s", "start")]),
            stop: new_binding(vec![with_keys_str(&["s"]), with_help("s", "stop")]),
            reset: new_binding(vec![with_keys_str(&["r"]), with_help("r", "reset")]),
            quit: new_binding(vec![
                with_keys_str(&["q", "ctrl+c"]),
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

        // Match Go's initial state: start is disabled since timer starts running
        keymap.start.set_enabled(false);

        // Create help model
        let mut help = HelpModel::new();
        help.show_all = false; // Use short help like Go version

        Self {
            // Match Go's NewWithInterval(timeout, time.Millisecond)
            timer: new_with_interval(TIMEOUT, Duration::from_millis(1)),
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
        // Match Go's m.timer.Init()
        let cmd = model.timer.init();
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle timer messages (matching Go's switch cases)

        // timer.TickMsg case
        if let Some(_tick_msg) = msg.downcast_ref::<TickMsg>() {
            return self.timer.update(msg);
        }

        // timer.StartStopMsg case
        if let Some(_start_stop_msg) = msg.downcast_ref::<StartStopMsg>() {
            let cmd = self.timer.update(msg);
            // Match Go's key enabling logic
            self.keymap.stop.set_enabled(self.timer.running());
            self.keymap.start.set_enabled(!self.timer.running());
            return cmd;
        }

        // timer.TimeoutMsg case
        if let Some(_timeout_msg) = msg.downcast_ref::<TimeoutMsg>() {
            self.quitting = true;
            return Some(quit());
        }

        // tea.KeyMsg case
        if let Some(key) = msg.downcast_ref::<KeyMsg>() {
            if matches_binding(key, &self.keymap.quit) {
                self.quitting = true;
                return Some(quit());
            } else if matches_binding(key, &self.keymap.reset) {
                // Match Go's m.timer.Timeout = timeout
                self.timer.timeout = TIMEOUT;
            } else if matches_binding(key, &self.keymap.start)
                || matches_binding(key, &self.keymap.stop)
            {
                // Match Go's m.timer.Toggle()
                return Some(self.timer.toggle());
            }
        }

        None
    }

    fn view(&self) -> String {
        // Match Go's View() method exactly

        // For a more detailed timer view you could read m.timer.Timeout to get
        // the remaining time as a time.Duration and skip calling m.timer.View()
        // entirely.
        let mut s = if self.timer.timedout() {
            "All done!".to_string()
        } else {
            // Use custom formatting to match Go's 3 decimal places for seconds
            format_duration_like_go(self.timer.timeout)
        };

        s.push('\n');

        if !self.quitting {
            s = format!("Exiting in {}", s);
            s.push_str(&self.help.view(self));
        }

        s
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<Model>::builder().signal_handler(true).build()?;

    program.run().await?;
    Ok(())
}
