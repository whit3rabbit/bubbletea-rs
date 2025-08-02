use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_rs::command::{batch, tick, window_size};
use crossterm::event::MouseEventKind;
use std::time::Duration;

const FPS: u32 = 60;
const ASTERISK: &str = "*";

#[derive(Debug, Clone)]
struct FrameMsg;

#[derive(Debug, Clone)]
struct InitMsg;

fn animate() -> Cmd {
    tick(Duration::from_millis(1000 / FPS as u64), |_| Box::new(FrameMsg) as Msg)
}

fn init_cmd() -> Cmd {
    Box::pin(async move {
        Some(Box::new(InitMsg) as Msg)
    })
}

#[derive(Debug)]
pub struct DebugModel {
    frame_count: u32,
    got_window_size: bool,
    width: usize,
    height: usize,
    x: f64,
    y: f64,
    messages: Vec<String>,
}

impl Model for DebugModel {
    fn init() -> (Self, Option<Cmd>) {
        let m = Self {
            frame_count: 0,
            got_window_size: false,
            width: 80,
            height: 24,
            x: 40.0,
            y: 12.0,
            messages: vec!["Initialized".to_string()],
        };
        
        (m, Some(batch(vec![
            init_cmd(),
            window_size(),
            animate(),
        ])))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Log all messages
        let msg_type = if msg.downcast_ref::<InitMsg>().is_some() {
            "InitMsg"
        } else if msg.downcast_ref::<FrameMsg>().is_some() {
            "FrameMsg"
        } else if msg.downcast_ref::<bubbletea_rs::WindowSizeMsg>().is_some() {
            "WindowSizeMsg"
        } else if msg.downcast_ref::<bubbletea_rs::MouseMsg>().is_some() {
            "MouseMsg"
        } else if msg.downcast_ref::<KeyMsg>().is_some() {
            "KeyMsg"
        } else {
            "Unknown"
        };
        
        self.messages.push(format!("Frame {}: Got {}", self.frame_count, msg_type));
        if self.messages.len() > 10 {
            self.messages.remove(0);
        }
        
        if msg.downcast_ref::<InitMsg>().is_some() {
            return None;
        }
        
        if let Some(_k) = msg.downcast_ref::<KeyMsg>() {
            return Some(quit());
        }

        if let Some(ws) = msg.downcast_ref::<bubbletea_rs::WindowSizeMsg>() {
            self.got_window_size = true;
            self.width = ws.width as usize;
            self.height = ws.height as usize;
            self.x = ws.width as f64 / 2.0;
            self.y = ws.height as f64 / 2.0;
            return None;
        }

        if let Some(mouse) = msg.downcast_ref::<bubbletea_rs::MouseMsg>() {
            if matches!(mouse.button, MouseEventKind::Moved) {
                self.x = mouse.x as f64;
                self.y = mouse.y as f64;
            }
            return None;
        }

        if msg.downcast_ref::<FrameMsg>().is_some() {
            self.frame_count += 1;
            // Simple movement for testing
            self.x = 40.0 + (self.frame_count as f64 * 0.1).sin() * 20.0;
            return Some(animate());
        }

        None
    }

    fn view(&self) -> String {
        let mut output = String::new();
        
        // Status info
        output.push_str(&format!("Frame: {} | Size: {}x{} | WindowSize received: {}\n", 
            self.frame_count, self.width, self.height, self.got_window_size));
        output.push_str(&format!("Position: ({:.1}, {:.1})\n", self.x, self.y));
        output.push_str("─".repeat(self.width).as_str());
        output.push('\n');
        
        // Draw a simple grid with the ellipse position
        for y in 0..20 {
            for x in 0..self.width {
                if (x as f64 - self.x).abs() < 2.0 && (y as f64 - self.y).abs() < 2.0 {
                    output.push('*');
                } else if x == 0 || x == self.width - 1 || y == 0 || y == 19 {
                    output.push('│');
                } else {
                    output.push(' ');
                }
            }
            output.push('\n');
        }
        
        output.push_str("─".repeat(self.width).as_str());
        output.push('\n');
        
        // Message log
        output.push_str("Recent messages:\n");
        for msg in &self.messages {
            output.push_str(&format!("  {}\n", msg));
        }
        
        output
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use bubbletea_rs::MouseMotion;
    
    let program = Program::<DebugModel>::builder()
        .alt_screen(false)
        .mouse_motion(MouseMotion::Cell)
        .signal_handler(true)
        .build()?;

    program.run().await?;
    Ok(())
}