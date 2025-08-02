use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_rs::command::{batch, tick, window_size};
use crossterm::event::MouseEventKind;
use std::time::Duration;

// Port of Bubble Tea's cellbuffer example.
// Draws an animated ellipse that springs toward the mouse position.

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

#[derive(Debug, Clone)]
struct Spring {
    fps: f64,
    frequency: f64,
    damping: f64,
}

impl Spring {
    fn new(fps: u32, frequency: f64, damping: f64) -> Self {
        Self { fps: fps as f64, frequency, damping }
    }

    // Critically-damped-ish spring integrator similar to harmonica.NewSpring.Update
    // Returns (next_position, next_velocity)
    fn update(&self, pos: f64, vel: f64, target: f64) -> (f64, f64) {
        let dt = 1.0 / self.fps;
        let k = self.frequency * self.frequency; // stiffness
        let c = 2.0 * self.damping * self.frequency; // damping coeff
        let acc = -k * (pos - target) - c * vel;
        let vel_next = vel + acc * dt;
        let pos_next = pos + vel_next * dt;
        (pos_next, vel_next)
    }
}

#[derive(Debug, Clone)]
struct CellBuffer {
    cells: Vec<String>,
    stride: usize,
}

impl CellBuffer {
    fn new() -> Self { Self { cells: Vec::new(), stride: 0 } }

    fn init(&mut self, w: usize, h: usize) {
        if w == 0 { return; }
        self.stride = w;
        self.cells = vec![" ".to_string(); w * h];
        self.wipe();
    }

    fn set(&mut self, x: isize, y: isize) {
        let w = self.width() as isize;
        let h = self.height() as isize;
        if x < 0 || y < 0 || x >= w || y >= h { return; }
        let i = (y as usize) * self.stride + (x as usize);
        if i < self.cells.len() {
            self.cells[i] = ASTERISK.to_string();
        }
    }

    fn wipe(&mut self) {
        for c in &mut self.cells { *c = " ".to_string(); }
    }

    fn width(&self) -> usize { self.stride }

    fn height(&self) -> usize {
        if self.stride == 0 { return 0; }
        self.cells.len() / self.stride
    }

    fn ready(&self) -> bool { !self.cells.is_empty() }
}

impl std::fmt::Display for CellBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.cells.len() {
            if i > 0 && i % self.stride == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", self.cells[i])?;
        }
        Ok(())
    }
}

fn draw_ellipse(cb: &mut CellBuffer, xc: f64, yc: f64, rx: f64, ry: f64) {
    let mut dx;
    let mut dy;
    let mut d1;
    let mut d2;
    let mut x = 0.0f64;
    let mut y = ry;

    d1 = ry * ry - rx * rx * ry + 0.25 * rx * rx;
    dx = 2.0 * ry * ry * x;
    dy = 2.0 * rx * rx * y;

    while dx < dy {
        cb.set((x + xc) as isize, (y + yc) as isize);
        cb.set((-x + xc) as isize, (y + yc) as isize);
        cb.set((x + xc) as isize, (-y + yc) as isize);
        cb.set((-x + xc) as isize, (-y + yc) as isize);
        if d1 < 0.0 {
            x += 1.0;
            dx = dx + (2.0 * ry * ry);
            d1 = d1 + dx + (ry * ry);
        } else {
            x += 1.0;
            y -= 1.0;
            dx = dx + (2.0 * ry * ry);
            dy = dy - (2.0 * rx * rx);
            d1 = d1 + dx - dy + (ry * ry);
        }
    }

    d2 = (ry * ry) * ((x + 0.5) * (x + 0.5)) + (rx * rx) * ((y - 1.0) * (y - 1.0)) - (rx * rx * ry * ry);

    while y >= 0.0 {
        cb.set((x + xc) as isize, (y + yc) as isize);
        cb.set((-x + xc) as isize, (y + yc) as isize);
        cb.set((x + xc) as isize, (-y + yc) as isize);
        cb.set((-x + xc) as isize, (-y + yc) as isize);
        if d2 > 0.0 {
            y -= 1.0;
            dy = dy - (2.0 * rx * rx);
            d2 = d2 + (rx * rx) - dy;
        } else {
            y -= 1.0;
            x += 1.0;
            dx = dx + (2.0 * ry * ry);
            dy = dy - (2.0 * rx * rx);
            d2 = d2 + dx - dy + (rx * rx);
        }
    }
}

#[derive(Debug)]
pub struct CellBufferModel {
    cells: CellBuffer,
    spring: Spring,
    target_x: f64,
    target_y: f64,
    x: f64,
    y: f64,
    x_vel: f64,
    y_vel: f64,
}

impl Model for CellBufferModel {
    fn init() -> (Self, Option<Cmd>) {
        let mut m = Self {
            cells: CellBuffer::new(),
            spring: Spring::new(FPS, 7.5, 0.15),
            target_x: 40.0,  // Default center position
            target_y: 12.0,
            x: 40.0,
            y: 12.0,
            x_vel: 0.0,
            y_vel: 0.0,
        };
        
        // Initialize with default size if terminal size detection fails
        // This ensures the animation works even without WindowSizeMsg
        m.cells.init(80, 24);  // Common default terminal size
        
        // Request window size and schedule first frame
        // Also send an init message to force initial render
        (m, Some(batch(vec![
            init_cmd(),
            window_size(),
            animate(),
        ])))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle init message - force initial render
        if msg.downcast_ref::<InitMsg>().is_some() {
            return None;
        }
        
        if let Some(_k) = msg.downcast_ref::<KeyMsg>() {
            // Any key press quits
            return Some(quit());
        }

        if let Some(ws) = msg.downcast_ref::<bubbletea_rs::WindowSizeMsg>() {
            // Update target to new center if this is the first real size
            let was_default = self.cells.width() == 80 && self.cells.height() == 24;
            
            self.cells.init(ws.width as usize, ws.height as usize);
            
            if was_default {
                // Move to actual center now that we have real dimensions
                self.target_x = ws.width as f64 / 2.0;
                self.target_y = ws.height as f64 / 2.0;
                self.x = self.target_x;
                self.y = self.target_y;
            }
            return None;
        }

        if let Some(mouse) = msg.downcast_ref::<bubbletea_rs::MouseMsg>() {
            if !self.cells.ready() { return None; }
            // Support terminals that report only drag/press events in alt screen
            if matches!(mouse.button, MouseEventKind::Moved | MouseEventKind::Down(_) | MouseEventKind::Drag(_)) {
                self.target_x = mouse.x as f64;
                self.target_y = mouse.y as f64;
            }
            return None;
        }

        if msg.downcast_ref::<FrameMsg>().is_some() {
            if !self.cells.ready() { return Some(animate()); }
            self.cells.wipe();
            let (nx, nvx) = self.spring.update(self.x, self.x_vel, self.target_x);
            let (ny, nvy) = self.spring.update(self.y, self.y_vel, self.target_y);
            self.x = nx; self.x_vel = nvx;
            self.y = ny; self.y_vel = nvy;
            draw_ellipse(&mut self.cells, self.x, self.y, 16.0, 8.0);
            return Some(animate());
        }

        None
    }

    fn view(&self) -> String {
        if !self.cells.ready() {
            // More visible loading message with some content
            let border = "─".repeat(78);
            return format!(
                "┌{}┐\n\
                 │{:^78}│\n\
                 │{:^78}│\n\
                 │{:^78}│\n\
                 └{}┘",
                border,
                "CELLBUFFER EXAMPLE",
                "Initializing...",
                "Move mouse to control ellipse, press any key to quit",
                border
            );
        }
        self.cells.to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use bubbletea_rs::MouseMotion;
    
    // Use alt screen; some terminals (e.g., Ghostty) only send full mouse
    // reporting while in the alternate screen buffer.
    let program = Program::<CellBufferModel>::builder()
        .alt_screen(true)
        .mouse_motion(MouseMotion::Cell)
        .signal_handler(true)
        .build()?;

    program.run().await?;
    Ok(())
}