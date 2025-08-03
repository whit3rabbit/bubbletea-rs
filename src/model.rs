//! This module defines the core `Model` trait, which is central to the
//! Model-View-Update (MVU) architecture used in `bubbletea-rs` applications.
//! The `Model` trait provides a clear and consistent interface for managing
//! application state, processing messages, and rendering the user interface.
//!
//! It is designed to be a direct, idiomatic Rust equivalent of Go's `bubbletea`
//! `Model` interface, facilitating migration and understanding for developers
//! familiar with the Go version.

use crate::{Cmd, Msg};

/// The Model trait defines the core interface for bubbletea-rs applications.
///
/// This trait provides a direct 1-to-1 mapping from Go's Model interface
/// with identical method signatures and behavior. Models represent your
/// application's state and logic, following the Model-View-Update pattern.
///
/// # Trait Bounds
///
/// - `Send`: Ensures the model can be safely transferred between threads
/// - `Sized`: Ensures the model has a known size at compile time
/// - `'static`: Ensures the model doesn't contain non-static references
///
/// These bounds are required for async safety and Tokio integration.
///
/// # Example
///
/// ```rust
/// use bubbletea_rs::{Model, Msg, Cmd, KeyMsg};
///
/// struct Counter {
///     value: i32,
/// }
///
/// impl Model for Counter {
///     fn init() -> (Self, Option<Cmd>) {
///         (Self { value: 0 }, None)
///     }
///     
///     fn update(&mut self, msg: Msg) -> Option<Cmd> {
///         if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
///             match key_msg.key {
///                 crossterm::event::KeyCode::Up => self.value += 1,
///                 crossterm::event::KeyCode::Down => self.value -= 1,
///                 _ => {}
///             }
///         }
///         None
///     }
///     
///     fn view(&self) -> String {
///         format!("Counter: {} (↑/↓ to change)", self.value)
///     }
/// }
/// ```
pub trait Model: Send + Sized + 'static {
    /// Initialize the model with its initial state and optional startup command.
    ///
    /// This method is called once when the application starts and should return
    /// the initial state of your model along with an optional command to execute
    /// immediately after initialization.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - `Self`: The initialized model with its starting state
    /// - `Option<Cmd>`: An optional command to run immediately (e.g., loading data)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use bubbletea_rs::{Model, Cmd};
    /// # struct MyModel { count: i32 }
    /// # impl Model for MyModel {
    /// fn init() -> (Self, Option<Cmd>) {
    ///     // Start with a count of 0 and no initial command
    ///     (MyModel { count: 0 }, None)
    /// }
    /// # fn update(&mut self, msg: bubbletea_rs::Msg) -> Option<Cmd> { None }
    /// # fn view(&self) -> String { String::new() }
    /// # }
    /// ```
    fn init() -> (Self, Option<Cmd>);

    /// Update the model in response to a received message.
    ///
    /// This method is called whenever a message is received by your application.
    /// It should update the model's state based on the message content and
    /// optionally return a command to execute as a side effect.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to process. Use `msg.downcast_ref::<T>()` to check
    ///   for specific message types.
    ///
    /// # Returns
    ///
    /// An optional command to execute after the update. Return `None` if no
    /// side effects are needed.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use bubbletea_rs::{Model, Msg, Cmd, KeyMsg};
    /// # struct MyModel { count: i32 }
    /// # impl Model for MyModel {
    /// # fn init() -> (Self, Option<Cmd>) { (MyModel { count: 0 }, None) }
    /// fn update(&mut self, msg: Msg) -> Option<Cmd> {
    ///     if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
    ///         match key_msg.key {
    ///             crossterm::event::KeyCode::Up => {
    ///                 self.count += 1;
    ///                 // No command needed for this update
    ///                 None
    ///             }
    ///             _ => None,
    ///         }
    ///     } else {
    ///         None
    ///     }
    /// }
    /// # fn view(&self) -> String { String::new() }
    /// # }
    /// ```
    fn update(&mut self, msg: Msg) -> Option<Cmd>;

    /// Render the current model state as a string for terminal display.
    ///
    /// This method is called whenever the terminal needs to be redrawn.
    /// It should return a string representation of the current model state
    /// that will be displayed to the user.
    ///
    /// # Returns
    ///
    /// A `String` containing the rendered view. This can include:
    /// - ANSI escape codes for colors and styling
    /// - Newlines for multi-line layouts
    /// - Unicode characters for advanced formatting
    ///
    /// # Performance Notes
    ///
    /// This method may be called frequently during redraws, so avoid
    /// expensive computations. Consider caching formatted strings if
    /// the rendering is complex.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use bubbletea_rs::{Model, Msg, Cmd};
    /// # struct MyModel { count: i32, name: String }
    /// # impl Model for MyModel {
    /// # fn init() -> (Self, Option<Cmd>) { (MyModel { count: 0, name: "App".to_string() }, None) }
    /// # fn update(&mut self, msg: Msg) -> Option<Cmd> { None }
    /// fn view(&self) -> String {
    ///     format!(
    ///         "Welcome to {}!\n\nCount: {}\n\nPress ↑/↓ to change",
    ///         self.name,
    ///         self.count
    ///     )
    /// }
    /// # }
    /// ```
    fn view(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{KeyMsg, QuitMsg};
    use crossterm::event::{KeyCode, KeyModifiers};

    #[derive(Debug, Clone)]
    struct CounterModel {
        count: i32,
        step: i32,
    }

    impl Model for CounterModel {
        fn init() -> (Self, Option<Cmd>) {
            (Self { count: 0, step: 1 }, None)
        }

        fn update(&mut self, msg: Msg) -> Option<Cmd> {
            if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
                match key_msg.key {
                    KeyCode::Up | KeyCode::Char('+') => {
                        self.count += self.step;
                    }
                    KeyCode::Down | KeyCode::Char('-') => {
                        self.count -= self.step;
                    }
                    KeyCode::Char('r') => {
                        self.count = 0;
                    }
                    KeyCode::Char('s') => {
                        self.step = if self.step == 1 { 10 } else { 1 };
                    }
                    KeyCode::Char('q') => {
                        return Some(Box::pin(async { Some(Box::new(QuitMsg) as Msg) }));
                    }
                    _ => {}
                }
            }
            None
        }

        fn view(&self) -> String {
            format!(
                "Counter: {}\nStep: {}\n\nControls:\n↑/+ : Increment\n↓/- : Decrement\nr : Reset\ns : Toggle step (1/10)\nq : Quit",
                self.count, self.step
            )
        }
    }

    #[derive(Debug, Clone)]
    struct TextInputModel {
        content: String,
        cursor: usize,
        max_length: usize,
    }

    impl Model for TextInputModel {
        fn init() -> (Self, Option<Cmd>) {
            (
                Self {
                    content: String::new(),
                    cursor: 0,
                    max_length: 100,
                },
                None,
            )
        }

        fn update(&mut self, msg: Msg) -> Option<Cmd> {
            if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
                match key_msg.key {
                    KeyCode::Char(c) if self.content.len() < self.max_length => {
                        self.content.insert(self.cursor, c);
                        self.cursor += 1;
                    }
                    KeyCode::Backspace if self.cursor > 0 => {
                        self.cursor -= 1;
                        self.content.remove(self.cursor);
                    }
                    KeyCode::Delete if self.cursor < self.content.len() => {
                        self.content.remove(self.cursor);
                    }
                    KeyCode::Left if self.cursor > 0 => {
                        self.cursor -= 1;
                    }
                    KeyCode::Right if self.cursor < self.content.len() => {
                        self.cursor += 1;
                    }
                    KeyCode::Home => {
                        self.cursor = 0;
                    }
                    KeyCode::End => {
                        self.cursor = self.content.len();
                    }
                    KeyCode::Esc => {
                        return Some(Box::pin(async { Some(Box::new(QuitMsg) as Msg) }));
                    }
                    _ => {}
                }
            }
            None
        }

        fn view(&self) -> String {
            let mut display = self.content.clone();
            display.insert(self.cursor, '|');

            format!(
                "Text Input ({}/{})\n\n{}\n\nControls:\nType to add text\n← → : Move cursor\nBackspace/Delete : Remove text\nHome/End : Jump to start/end\nEsc : Quit",
                self.content.len(),
                self.max_length,
                display
            )
        }
    }

    #[test]
    fn test_counter_model_init() {
        let (model, cmd) = CounterModel::init();
        assert_eq!(model.count, 0);
        assert_eq!(model.step, 1);
        assert!(cmd.is_none());
    }

    #[test]
    fn test_counter_model_update() {
        let (mut model, _) = CounterModel::init();

        let key_msg = KeyMsg {
            key: KeyCode::Up,
            modifiers: KeyModifiers::empty(),
        };
        let cmd = model.update(Box::new(key_msg));
        assert_eq!(model.count, 1);
        assert!(cmd.is_none());

        let key_msg = KeyMsg {
            key: KeyCode::Down,
            modifiers: KeyModifiers::empty(),
        };
        model.update(Box::new(key_msg));
        assert_eq!(model.count, 0);

        model.count = 42;
        let key_msg = KeyMsg {
            key: KeyCode::Char('r'),
            modifiers: KeyModifiers::empty(),
        };
        model.update(Box::new(key_msg));
        assert_eq!(model.count, 0);
    }

    #[test]
    fn test_counter_model_view() {
        let (model, _) = CounterModel::init();
        let view = model.view();
        assert!(view.contains("Counter: 0"));
        assert!(view.contains("Step: 1"));
        assert!(view.contains("Controls:"));
    }

    #[test]
    fn test_text_input_model_init() {
        let (model, cmd) = TextInputModel::init();
        assert!(model.content.is_empty());
        assert_eq!(model.cursor, 0);
        assert_eq!(model.max_length, 100);
        assert!(cmd.is_none());
    }

    #[test]
    fn test_text_input_model_char_input() {
        let (mut model, _) = TextInputModel::init();

        let key_msg = KeyMsg {
            key: KeyCode::Char('H'),
            modifiers: KeyModifiers::empty(),
        };
        model.update(Box::new(key_msg));
        assert_eq!(model.content, "H");
        assert_eq!(model.cursor, 1);

        let key_msg = KeyMsg {
            key: KeyCode::Char('i'),
            modifiers: KeyModifiers::empty(),
        };
        model.update(Box::new(key_msg));
        assert_eq!(model.content, "Hi");
        assert_eq!(model.cursor, 2);
    }

    #[test]
    fn test_text_input_model_backspace() {
        let (mut model, _) = TextInputModel::init();
        model.content = "Hello".to_string();
        model.cursor = 5;

        let key_msg = KeyMsg {
            key: KeyCode::Backspace,
            modifiers: KeyModifiers::empty(),
        };
        model.update(Box::new(key_msg));
        assert_eq!(model.content, "Hell");
        assert_eq!(model.cursor, 4);
    }

    #[test]
    fn test_text_input_model_cursor_movement() {
        let (mut model, _) = TextInputModel::init();
        model.content = "Hello".to_string();
        model.cursor = 2;

        let key_msg = KeyMsg {
            key: KeyCode::Left,
            modifiers: KeyModifiers::empty(),
        };
        model.update(Box::new(key_msg));
        assert_eq!(model.cursor, 1);

        let key_msg = KeyMsg {
            key: KeyCode::Right,
            modifiers: KeyModifiers::empty(),
        };
        model.update(Box::new(key_msg));
        assert_eq!(model.cursor, 2);

        let key_msg = KeyMsg {
            key: KeyCode::Home,
            modifiers: KeyModifiers::empty(),
        };
        model.update(Box::new(key_msg));
        assert_eq!(model.cursor, 0);

        let key_msg = KeyMsg {
            key: KeyCode::End,
            modifiers: KeyModifiers::empty(),
        };
        model.update(Box::new(key_msg));
        assert_eq!(model.cursor, 5);
    }

    #[test]
    fn test_model_trait_bounds() {
        fn assert_send<T: Send>() {}
        fn assert_sized<T: Sized>() {}
        fn assert_static<T: 'static>() {}

        assert_send::<CounterModel>();
        assert_sized::<CounterModel>();
        assert_static::<CounterModel>();

        assert_send::<TextInputModel>();
        assert_sized::<TextInputModel>();
        assert_static::<TextInputModel>();
    }

    #[test]
    fn test_model_send_sync_static() {
        fn assert_send_sync_static<T: Send + Sync + 'static>() {}
        assert_send_sync_static::<CounterModel>();
        assert_send_sync_static::<TextInputModel>();
    }
}
