# Text Input Example - Bubbletea-rs Tutorial

This example demonstrates how to create an interactive text input field using the `bubbletea-rs` framework and its `bubbletea-widgets` library. If you're new to TUI (Terminal User Interface) development or the Bubbletea framework, this guide will walk you through everything you need to know.

## What You'll Learn

- How to build a TUI application using the MVU (Model-View-Update) pattern
- How to use the pre-built `textinput` widget from `bubbletea-widgets`
- How to handle keyboard input and user interactions
- How to manage application state and control flow

## Prerequisites

You don't need prior experience with Bubbletea, but you should have:
- Basic Rust knowledge (structs, traits, ownership)
- Rust and Cargo installed on your system

## Core Concepts

Before diving into the code, let's understand the key concepts:

### MVU Pattern (Model-View-Update)

Bubbletea uses the **Model-View-Update** pattern, which consists of three parts:

1. **Model**: Your application's state (data)
2. **View**: How your state is displayed on screen
3. **Update**: How your state changes in response to events (messages)

Think of it like this:
- Your app receives a message (like a keypress)
- The `update` function processes this message and updates your model
- The `view` function renders the new state to the terminal
- The cycle repeats

### Messages and Commands

- **Messages (Msg)**: Events that occur in your app (keypresses, mouse clicks, timers)
- **Commands (Cmd)**: Asynchronous operations that produce messages (like fetching data or setting timers)

## Step-by-Step Implementation

Let's build a text input that asks for your favorite Pokémon!

### Step 1: Set Up Dependencies

First, add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
bubbletea-rs = "0.0.6"           # The main framework
bubbletea-widgets = "0.1.6"      # Pre-built UI components
tokio = { version = "1.0", features = ["full"] }  # Async runtime
crossterm = "0.29"                # Terminal manipulation
```

### Step 2: Import Required Components

```rust
use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::textinput;
use crossterm::event::{KeyCode, KeyModifiers};
```

- `Model`: The trait your app must implement
- `KeyMsg`: Message type for keyboard events
- `quit`: Command to exit the program
- `textinput`: The pre-built text input widget

### Step 3: Define Your Model

The model holds your application's state:

```rust
pub struct TextInputModel {
    text_input: textinput::Model,  // The text input widget
    quitting: bool,                 // Flag to track if we're exiting
}
```

### Step 4: Initialize Your Model

```rust
impl TextInputModel {
    fn new() -> Self {
        let mut ti = textinput::new();
        ti.set_placeholder("Pikachu");  // Hint text when empty
        ti.set_char_limit(156);         // Maximum characters allowed
        ti.set_width(20);                // Visual width of input field
        
        Self { 
            text_input: ti, 
            quitting: false 
        }
    }
}
```

### Step 5: Implement the Model Trait

This is where the MVU pattern comes together:

```rust
impl Model for TextInputModel {
    // Called once when the app starts
    fn init() -> (Self, Option<Cmd>) {
        let mut model = Self::new();
        let cmd = model.text_input.focus();  // Focus the input field
        (model, Some(cmd))                   // Return model and initial command
    }

    // Called whenever a message is received
    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Check if the message is a keyboard event
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                // Enter or Escape key quits the app
                KeyCode::Enter | KeyCode::Esc => {
                    self.quitting = true;
                    return Some(quit());  // Return quit command
                }
                // Ctrl+C also quits
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quitting = true;
                    return Some(quit());
                }
                _ => {}  // Other keys are handled by the text input
            }
        }

        // Pass the message to the text input widget
        // It handles typing, backspace, cursor movement, etc.
        self.text_input.update(msg)
    }

    // Called to render the current state
    fn view(&self) -> String {
        if self.quitting {
            return String::new();  // Clear screen when quitting
        }

        // Format the display
        format!(
            "What's your favorite Pokémon?\n\n{}\n\n(esc to quit)\n",
            self.text_input.view()  // The widget renders itself
        )
    }
}
```

### Step 6: Create the Main Function

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the program with configuration
    let program = Program::<TextInputModel>::builder()
        .alt_screen(true)      // Use alternate screen buffer (cleans up on exit)
        .signal_handler(true)  // Handle Ctrl+C properly
        .build()?;

    // Run the program
    let _ = program.run().await?;
    Ok(())
}
```

## How It All Works Together

1. **Startup**: The program calls `init()` to create your model and get any initial commands
2. **Event Loop**: The program continuously:
   - Waits for events (keypresses, etc.)
   - Converts events to messages
   - Calls `update()` with each message
   - Calls `view()` to render the new state
3. **User Types**: When the user presses a key:
   - A `KeyMsg` is created
   - Your `update()` function checks if it's a quit key
   - If not, the message is passed to the text input widget
   - The widget updates its internal state (adds the character, moves cursor, etc.)
4. **Display Updates**: After each update, `view()` is called:
   - It formats the prompt and calls the widget's `view()` method
   - The widget returns its visual representation
   - The terminal is updated with the new content

## Running the Example

```bash
# From the example directory
cargo run

# Or from the project root
cargo run --example textinput
```

## Customizing Your Text Input

The `textinput` widget offers many customization options:

```rust
let mut ti = textinput::new();

// Visual customization
ti.set_width(30);                    // Width in characters
ti.set_placeholder("Enter text...");  // Hint text
ti.set_char_limit(100);              // Maximum input length

// Behavior customization
ti.focus();                          // Give keyboard focus
ti.blur();                           // Remove focus
ti.set_value("Initial text");       // Pre-fill with text
ti.reset();                          // Clear the input

// Get the current value
let user_input = ti.value();        // Returns &str
```

## Common Patterns and Extensions

### Adding Validation

```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    // ... handle keys ...
    
    // Validate after each update
    let value = self.text_input.value();
    self.is_valid = value.len() >= 3 && value.len() <= 20;
    
    self.text_input.update(msg)
}
```

### Multiple Input Fields

```rust
struct FormModel {
    inputs: Vec<textinput::Model>,
    active_input: usize,
}

// Switch between inputs with Tab key
if key_msg.key == KeyCode::Tab {
    self.inputs[self.active_input].blur();
    self.active_input = (self.active_input + 1) % self.inputs.len();
    return self.inputs[self.active_input].focus();
}
```

### Styling with Colors

```rust
use lipgloss_extras::lipgloss::{Style, Color};

fn view(&self) -> String {
    let input_style = Style::new()
        .foreground(Color::from("205"))  // Pink color
        .bold(true);
    
    format!("{}\n{}", 
        "Enter your name:",
        input_style.render(&self.text_input.view())
    )
}
```

## Key Features Demonstrated

- **Pre-built Widget**: Using `bubbletea-widgets::textinput` instead of building from scratch
- **Event Handling**: Processing keyboard input through the message system
- **State Management**: Tracking application state (input content, quitting flag)
- **Clean Exit**: Properly handling Escape, Enter, and Ctrl+C
- **Focus Management**: Ensuring the input field receives keyboard events

## Troubleshooting

### Input not responding to keypresses
- Make sure you call `focus()` on the text input in `init()`
- Verify you're passing messages to the widget with `self.text_input.update(msg)`

### Characters not appearing
- Check that you're not accidentally consuming the message before passing it to the widget
- Ensure the widget's `view()` method is being called in your model's `view()`

### Terminal cleanup issues
- Always use `.alt_screen(true)` in the program builder
- This ensures the terminal is restored when your app exits

## Next Steps

Now that you understand the basics, try:

1. **Add validation**: Show an error message if input is invalid
2. **Multiple fields**: Create a form with name, email, and password fields
3. **Custom styling**: Use `lipgloss-extras` to add colors and borders
4. **Save input**: Store the value when Enter is pressed instead of quitting
5. **Add suggestions**: Show autocomplete options below the input

## Further Reading

- [Bubbletea-rs Documentation](https://docs.rs/bubbletea-rs)
- [Bubbletea-widgets Documentation](https://docs.rs/bubbletea-widgets)
- [Original Go Bubble Tea](https://github.com/charmbracelet/bubbletea) - for additional examples and patterns

## Summary

You've learned how to:
- Structure a TUI application using the MVU pattern
- Use the `textinput` widget for user input
- Handle keyboard events and control flow
- Build and run a Bubbletea application

The beauty of Bubbletea is that once you understand these concepts, you can build complex TUI applications by composing simple, reusable components. The `textinput` widget handles all the complex logic of cursor movement, text insertion, and selection - you just need to wire it into your application's flow!