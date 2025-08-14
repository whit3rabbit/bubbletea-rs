//! Send Message Example
//!
//! A simple example that shows how to send messages to a Bubble Tea program
//! from outside the program using external tasks.
//!
//! This example demonstrates:
//! - External message sending using tokio tasks and program handles
//! - Manual spinner animation matching Go version
//! - Message buffering and display
//! - Proper styling with lipgloss-extras

use bubbletea_rs::{batch, quit, tick, Cmd, KeyMsg, Model, Msg, Program};
use lipgloss_extras::lipgloss::{Color, Style};
use rand::Rng;
use std::time::Duration;

/// Result message sent from external task
#[derive(Debug, Clone)]
struct ResultMsg {
    duration: Duration,
    food: String,
}

impl ResultMsg {
    fn new(duration: Duration, food: String) -> Self {
        Self { duration, food }
    }

    /// Convert to display string, matching Go version format
    fn to_display_string(&self) -> String {
        if self.duration.is_zero() {
            // Empty result - show dots
            let dot_style = Style::new().foreground(Color::from("241"));
            dot_style.render(&".".repeat(30))
        } else {
            // Show food consumption result
            let duration_style = Style::new().foreground(Color::from("241"));
            format!(
                "🍔 Ate {} {}",
                self.food,
                duration_style.render(&format!("{:?}", self.duration))
            )
        }
    }
}

/// Message for spinner animation ticks
#[derive(Debug)]
struct SpinnerTickMsg;

/// Synthetic message used to trigger the initial render immediately after startup
#[derive(Debug, Clone)]
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

/// The application model
#[derive(Debug)]
struct SendMsgModel {
    spinner_frame: usize,
    results: Vec<ResultMsg>,
    quitting: bool,
}

impl SendMsgModel {
    const NUM_LAST_RESULTS: usize = 5;

    fn new() -> Self {
        Self {
            spinner_frame: 0,
            results: vec![ResultMsg::new(Duration::ZERO, String::new()); Self::NUM_LAST_RESULTS],
            quitting: false,
        }
    }

    /// Get spinner frames (matching Go version)
    fn spinner_frames() -> &'static [&'static str] {
        &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
    }

    /// Get current spinner frame with styling (#63)
    fn current_spinner_frame(&self) -> String {
        let frames = Self::spinner_frames();
        let frame = frames[self.spinner_frame % frames.len()];
        let spinner_style = Style::new().foreground(Color::from("63"));
        spinner_style.render(frame)
    }

    /// Advance to next spinner frame
    fn advance_spinner(&mut self) {
        let frames = Self::spinner_frames();
        self.spinner_frame = (self.spinner_frame + 1) % frames.len();
    }

    /// Add a new result to the buffer
    fn add_result(&mut self, result: ResultMsg) {
        // Shift existing results and add new one (like Go slice append)
        self.results.rotate_left(1);
        self.results[Self::NUM_LAST_RESULTS - 1] = result;
    }
}

impl Model for SendMsgModel {
    fn init() -> (Self, Option<Cmd>) {
        let model = SendMsgModel::new();
        // Start both initial render and spinner animation
        let init_cmd = init_render_cmd();
        (model, Some(init_cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle the initial render trigger message
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            // Start spinner animation and food simulation
            let spinner_cmd = tick(Duration::from_millis(100), |_| {
                Box::new(SpinnerTickMsg) as Msg
            });
            let food_cmd = simulate_food_eating();
            return Some(batch(vec![spinner_cmd, food_cmd]));
        }

        // Handle spinner tick messages
        if msg.downcast_ref::<SpinnerTickMsg>().is_some() {
            if !self.quitting {
                self.advance_spinner();
                return Some(tick(Duration::from_millis(100), |_| {
                    Box::new(SpinnerTickMsg) as Msg
                }));
            }
        }

        // Handle result messages from external task
        if let Some(result_msg) = msg.downcast_ref::<ResultMsg>() {
            self.add_result(result_msg.clone());
            // Continue the food simulation (like Go's goroutine loop)
            if !self.quitting {
                return Some(simulate_food_eating());
            }
            return None;
        }

        // Handle keyboard input - any key quits (matching Go version)
        if let Some(_key_msg) = msg.downcast_ref::<KeyMsg>() {
            self.quitting = true;
            return Some(quit());
        }

        None
    }

    fn view(&self) -> String {
        let mut s = String::new();

        if self.quitting {
            s.push_str("That's all for today!");
        } else {
            s.push_str(&format!("{} Eating food...", self.current_spinner_frame()));
        }

        s.push_str("\n\n");

        // Show results
        for result in &self.results {
            s.push_str(&result.to_display_string());
            s.push('\n');
        }

        if !self.quitting {
            let help_style = Style::new()
                .foreground(Color::from("241"))
                .margin(1, 0, 0, 0);
            s.push_str(&help_style.render("Press any key to exit"));
        }

        if self.quitting {
            s.push('\n');
        }

        // Apply app styling with margins (matching Go version)
        let app_style = Style::new().margin(1, 2, 0, 2);
        app_style.render(&s)
    }
}

/// Get a random food item (matching Go version)
fn random_food() -> String {
    let foods = [
        "an apple",
        "a pear",
        "a gherkin",
        "a party gherkin",
        "a kohlrabi",
        "some spaghetti",
        "tacos",
        "a currywurst",
        "some curry",
        "a sandwich",
        "some peanut butter",
        "some cashews",
        "some ramen",
    ];

    let mut rng = rand::thread_rng();
    foods[rng.gen_range(0..foods.len())].to_string()
}

/// Command to simulate food eating activity (matching Go version's external sending)
fn simulate_food_eating() -> Cmd {
    Box::pin(async move {
        // Random pause between 100-999ms like Go version
        let pause_ms = rand::thread_rng().gen_range(100..=999);
        let pause = Duration::from_millis(pause_ms);
        tokio::time::sleep(pause).await;

        // Send result message
        let food = random_food();
        let result_msg = ResultMsg::new(pause, food);
        Some(Box::new(result_msg) as Msg)
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<SendMsgModel>::builder()
        .alt_screen(true)
        .signal_handler(true)
        .build()?;

    // Run the program
    program.run().await?;

    Ok(())
}
