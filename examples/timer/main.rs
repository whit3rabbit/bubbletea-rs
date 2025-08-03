//! Timer Example
//!
//! Mirrors `bubbletea/examples/timer/main.go` behavior:
//! - 5s countdown that starts automatically
//! - Toggle start/stop with 's'
//! - Reset to full timeout with 'r'
//! - Quit with 'q', Esc, or Ctrl+C
//! - Renders remaining time and help text

use bubbletea_rs::{quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use crossterm::event::{KeyCode, KeyModifiers};
use std::time::{Duration, Instant};

/// Total timeout duration (5 seconds)
const TIMEOUT: Duration = Duration::from_secs(5);
/// Tick interval (1 millisecond) to match Go example
const INTERVAL: Duration = Duration::from_millis(1);

#[derive(Debug)]
struct TimerTickMsg;

#[derive(Debug)]
pub struct TimerModel {
    remaining: Duration,
    running: bool,
    quitting: bool,
    last_tick: Option<Instant>,
}

impl TimerModel {
    fn new() -> Self {
        Self {
            remaining: TIMEOUT,
            running: true, // starts immediately like the Go example
            quitting: false,
            last_tick: None,
        }
    }

    fn start(&mut self) -> Option<Cmd> {
        if !self.running && self.remaining > Duration::ZERO {
            self.running = true;
            self.last_tick = Some(Instant::now());
            return Some(Self::tick_cmd());
        }
        None
    }

    fn stop(&mut self) -> Option<Cmd> {
        if self.running {
            self.running = false;
        }
        None
    }

    fn toggle(&mut self) -> Option<Cmd> {
        if self.running {
            self.stop()
        } else {
            self.start()
        }
    }

    fn reset(&mut self) -> Option<Cmd> {
        self.remaining = TIMEOUT;
        // If we were running, keep running. If stopped, stay stopped.
        // Do not schedule an extra tick here; normal loop will continue.
        None
    }

    fn tick_cmd() -> Cmd {
        tick(INTERVAL, |_| Box::new(TimerTickMsg) as Msg)
    }

    fn on_tick(&mut self) -> Option<Cmd> {
        if !self.running || self.remaining == Duration::ZERO {
            return None;
        }

        let now = Instant::now();
        let elapsed = match self.last_tick {
            Some(prev) => now.saturating_duration_since(prev),
            None => INTERVAL,
        };
        self.last_tick = Some(now);

        // Subtract elapsed; clamp at zero
        if elapsed >= self.remaining {
            self.remaining = Duration::ZERO;
        } else {
            self.remaining -= elapsed;
        }

        if self.remaining == Duration::ZERO {
            // Timeout reached: quit
            self.quitting = true;
            return Some(quit());
        }

        // Schedule next tick while running
        Some(Self::tick_cmd())
    }

    fn timed_out(&self) -> bool {
        self.remaining == Duration::ZERO
    }

    fn view_time(&self) -> String {
        // Render like a simple timer: seconds.milliseconds with left padding similar to Go's view
        let secs = self.remaining.as_secs();
        let millis = (self.remaining.subsec_millis()) as u64;
        format!("{:>2}.{:03}s", secs, millis)
    }
}

impl Model for TimerModel {
    fn init() -> (Self, Option<Cmd>) {
        let mut m = TimerModel::new();
        // Start ticking immediately if running
        m.last_tick = Some(Instant::now());
        let cmd = if m.running {
            Some(TimerModel::tick_cmd())
        } else {
            None
        };
        (m, cmd)
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Tick
        if msg.downcast_ref::<TimerTickMsg>().is_some() {
            return self.on_tick();
        }

        // Keyboard handling
        if let Some(key) = msg.downcast_ref::<KeyMsg>() {
            match key.key {
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quitting = true;
                    return Some(quit());
                }
                KeyCode::Char('r') => {
                    self.reset();
                }
                KeyCode::Char('s') => {
                    return self.toggle();
                }
                _ => {}
            }
        }

        None
    }

    fn view(&self) -> String {
        let mut s = if self.timed_out() {
            "All done!".to_string()
        } else {
            self.view_time()
        };
        s.push('\n');
        if !self.quitting {
            s = format!("Exiting in {}\n{}", s, self.help_view());
        }
        s
    }
}

impl TimerModel {
    fn help_view(&self) -> String {
        // We don't have the bubbles help/keymap port yet, so render a simple help line
        // Start is disabled while running (mirrors Go's enabled state semantics)
        let start_label = if self.running { "start" } else { "start" }; // label remains 'start'
        let stop_label = if self.running { "stop" } else { "stop" };
        let mut keys = vec![format!(
            "s {}",
            if self.running {
                stop_label
            } else {
                start_label
            }
        )];
        keys.push("r reset".into());
        keys.push("q quit".into());
        format!("  {}", keys.join(" • "))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<TimerModel>::builder()
        .signal_handler(true)
        .build()?;

    program.run().await?;
    Ok(())
}
