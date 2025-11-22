# Multiple Text Inputs

<img width="1200" src="./textinputs.gif" />

This example demonstrates how to create a complete form with multiple text input fields using the `bubbletea-widgets` library. Perfect for login forms, registration pages, or any multi-field input interface.

If you're new to TUI (Terminal User Interface) development or the Bubbletea framework, this guide will walk you through building a professional-looking form from scratch.

## What You'll Learn

- How to manage multiple text input fields in one application
- How to implement focus navigation between fields (Tab, Shift+Tab, arrows)
- How to handle form submission and validation
- How to style and customize text inputs
- How to troubleshoot common rendering issues

## Prerequisites

- Basic Rust knowledge (structs, traits, ownership)
- Rust and Cargo installed on your system
- No prior TUI experience needed!

## Running the Example

```bash
# From the example directory
cargo run

# Or from the project root
cargo run --example textinputs
```

**Controls:**
- **Tab / Down Arrow**: Move to next field
- **Shift+Tab / Up Arrow**: Move to previous field  
- **Enter**: Advance to next field, or submit when button is focused
- **Esc / Ctrl+C**: Quit the application
- **Ctrl+R**: Toggle cursor style (blink → static → hidden)

## What the Example Shows

This form demonstrates:
- **3 text input fields**: Nickname, Email, Password
- **Focus management**: Clear visual indication of which field is active
- **Navigation flow**: Seamless movement between fields
- **Submit button**: Focusable button that completes the form
- **Keyboard shortcuts**: Standard form navigation patterns

## Setting Up Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
bubbletea-rs = "0.0.9"           # The main TUI framework
bubbletea-widgets = "0.1.12"      # Pre-built UI components including textinput
lipgloss-extras = { version = "0.1.1", features = ["full"] }  # Styling
crossterm = "0.29"               # Terminal interaction
tokio = { version = "1.0", features = ["full"] }  # Async runtime
```

## Understanding the Code Structure

### 1. Application State

```rust
struct ModelTextInputs {
    focus_index: isize,           // Which field is currently focused (0, 1, 2, or 3 for submit)
    inputs: Vec<textinput::Model>, // The three input fields
    cursor_mode: cursor::Mode,     // Current cursor appearance
    submit_focused: bool,          // Whether submit button has focus
    keymap: AppKeyMap,            // Keyboard shortcut definitions
}
```

### 2. Input Configuration

Each text input is configured with:
- **Placeholder text**: Hint shown when empty ("Nickname", "Email", "Password")
- **Character limits**: Maximum allowed input length
- **Styling**: Colors and appearance for focused/unfocused states

### 3. Focus Management System

The app tracks focus using:
- `focus_index`: Current field (0=Nickname, 1=Email, 2=Password)
- `submit_focused`: Whether the submit button is selected

Focus changes trigger:
- Visual styling updates (pink prompt for active field)
- Cursor activation/deactivation
- Proper command scheduling for blinking

## Key Implementation Details

### Message Handling Pattern

```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    // 1. Handle global shortcuts (quit, cursor toggle)
    if self.keymap.quit.matches(key_msg) {
        return Some(quit());
    }
    
    // 2. Handle navigation (Tab, arrows, Enter)
    if is_navigation_key {
        // Update focus_index, call focus()/blur() on inputs
        return Some(batch(focus_commands));
    }
    
    // 3. Delegate typing to the focused input only
    if !self.submit_focused {
        let idx = self.focus_index.max(0) as usize;
        return self.inputs[idx].update(msg);
    }
}
```

### Rendering Strategy

```rust
fn view(&self) -> String {
    // Render each input field
    for input in &self.inputs {
        output.push_str(&input.view());
    }
    
    // Render submit button with focus styling
    let button = if self.submit_focused { 
        focused_style.render("[ Submit ]")
    } else { 
        "[ Submit ]" 
    };
}
```

## Common Issues and Solutions

### Problem: "Letters are duplicating when I type"

**Symptoms:** First character appears twice (e.g., "NNickname" instead of "Nickname")

**Root Cause:** This was a rendering bug in the textinput widget's placeholder handling.

**Solution:** Update to `bubbletea-widgets` version 0.0.8 or later. This bug has been fixed.

```toml
bubbletea-widgets = "0.1.12"  # Fixed version
```

### Problem: "Cursor isn't blinking"

**Symptoms:** Cursor appears as a static block instead of blinking

**Cause:** Cursor mode not properly initialized or blink messages not being processed

**Solution:**
```rust
// Ensure cursor is set to blink mode
let _ = input.cursor.set_mode(cursor::Mode::Blink);

// Make sure you're calling focus() which starts blinking
let focus_cmd = input.focus();
```

### Problem: "Focus indication isn't working"

**Symptoms:** Can't tell which field is active

**Solution:** Set distinct styles for focused vs unfocused states:
```rust
if field_is_focused {
    input.prompt_style = Style::new().foreground(Color::from("205")); // Pink
} else {
    input.prompt_style = Style::new(); // Default
}
```

### Problem: "Navigation feels broken"

**Symptoms:** Tab/Enter doesn't move between fields properly

**Cause:** Focus management logic has bugs in index calculation

**Solution:** Carefully handle the focus_index wraparound:
```rust
// Forward navigation
self.focus_index += 1;
if self.focus_index >= self.inputs.len() as isize {
    self.submit_focused = true;
    self.focus_index = self.inputs.len() as isize;
}

// Backward navigation  
self.focus_index -= 1;
if self.focus_index < 0 {
    self.submit_focused = true;
    self.focus_index = self.inputs.len() as isize;
}
```

### Problem: "Runtime panic about 'period must be non-zero'"

**Cause:** Creating timer commands with zero duration

**Solution:** Don't create your own blink timers. Let the textinput widget handle its own cursor blinking.

## Building Your Own Multi-Input Form

### Step 1: Set Up Your Model

```rust
struct MyFormModel {
    inputs: Vec<textinput::Model>,
    focus_index: isize,
    // Add your own fields here
}

impl MyFormModel {
    fn new() -> Self {
        let mut inputs = Vec::new();
        
        // Create your input fields
        for (i, placeholder) in ["Username", "Email", "Password"].iter().enumerate() {
            let mut input = textinput::new();
            input.set_placeholder(placeholder);
            input.set_char_limit(50);
            inputs.push(input);
        }
        
        Self {
            inputs,
            focus_index: 0,
        }
    }
}
```

### Step 2: Implement Navigation

```rust
fn update(&mut self, msg: Msg) -> Option<Cmd> {
    if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
        match key_msg.key {
            KeyCode::Tab => {
                // Move to next field
                self.move_focus_forward();
                return Some(self.update_focus());
            }
            KeyCode::BackTab => {
                // Move to previous field  
                self.move_focus_backward();
                return Some(self.update_focus());
            }
            KeyCode::Enter => {
                // Handle submit or advance
                if self.at_last_field() {
                    return self.handle_submit();
                } else {
                    self.move_focus_forward();
                    return Some(self.update_focus());
                }
            }
            _ => {}
        }
    }
    
    // Pass to focused input
    self.inputs[self.focus_index as usize].update(msg)
}
```

### Step 3: Add Validation

```rust
fn validate_form(&self) -> Vec<String> {
    let mut errors = Vec::new();
    
    if self.inputs[0].value().is_empty() {
        errors.push("Username is required".to_string());
    }
    
    if !self.inputs[1].value().contains('@') {
        errors.push("Valid email is required".to_string());
    }
    
    if self.inputs[2].value().len() < 8 {
        errors.push("Password must be at least 8 characters".to_string());
    }
    
    errors
}
```

## Styling and Customization

### Colors and Themes

```rust
// Define your color palette
let primary_color = Color::from("205");    // Pink
let secondary_color = Color::from("240");  // Gray
let success_color = Color::from("46");     // Green
let error_color = Color::from("196");      // Red

// Apply to focused inputs
input.prompt_style = Style::new().foreground(primary_color);
input.text_style = Style::new().foreground(Color::from("255")); // White text
```

### Custom Cursor Styles

```rust
// Different cursor modes for different contexts
input.cursor.set_mode(cursor::Mode::Blink);   // Normal editing
input.cursor.set_mode(cursor::Mode::Static);  // Read-only fields
input.cursor.set_mode(cursor::Mode::Hide);    // Hidden fields
```

### Field-Specific Styling

```rust
match field_type {
    FieldType::Email => {
        input.set_placeholder("user@example.com");
        input.set_char_limit(100);
    }
    FieldType::Password => {
        input.set_placeholder("Enter secure password");
        input.set_echo_mode(EchoMode::EchoPassword); // Show dots instead of characters
        input.set_char_limit(50);
    }
    FieldType::Phone => {
        input.set_placeholder("(555) 123-4567");
        input.set_char_limit(15);
    }
}
```

## Advanced Features

### Form Validation with Live Feedback

```rust
fn view(&self) -> String {
    let mut output = String::new();
    
    for (i, input) in self.inputs.iter().enumerate() {
        // Show the input
        output.push_str(&input.view());
        
        // Show validation errors
        if let Some(error) = self.get_field_error(i) {
            let error_style = Style::new().foreground(Color::from("196"));
            output.push_str(&format!("\n  {}", error_style.render(&error)));
        }
        
        output.push('\n');
    }
    
    output
}
```

### Auto-completion and Suggestions

```rust
// Set up suggestions for email field
input.set_suggestions(vec![
    "@gmail.com".to_string(),
    "@yahoo.com".to_string(), 
    "@outlook.com".to_string(),
]);
```

### Save and Load Form State

```rust
fn save_form_state(&self) -> String {
    let values: Vec<String> = self.inputs.iter()
        .map(|input| input.value().to_string())
        .collect();
    serde_json::to_string(&values).unwrap()
}

fn load_form_state(&mut self, json: &str) {
    if let Ok(values) = serde_json::from_str::<Vec<String>>(json) {
        for (input, value) in self.inputs.iter_mut().zip(values) {
            input.set_value(&value);
        }
    }
}
```

## Testing Your Form

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_navigation() {
        let mut model = MyFormModel::new();
        assert_eq!(model.focus_index, 0);
        
        model.move_focus_forward();
        assert_eq!(model.focus_index, 1);
    }
    
    #[test]
    fn test_validation() {
        let model = MyFormModel::new();
        let errors = model.validate_form();
        assert!(!errors.is_empty()); // Empty form should have errors
    }
}
```

### Integration Testing

```rust
#[test]
fn test_full_form_flow() {
    let (mut model, _) = MyFormModel::init();
    
    // Simulate typing in first field
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('t'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let _ = model.update(key_msg);
    assert_eq!(model.inputs[0].value(), "t");
}
```

## Performance Tips

### Efficient Rendering

- Only update styling when focus changes, not on every keystroke
- Use `batch()` to combine multiple commands
- Cache styled strings for static elements

### Memory Management

- Limit input field character counts
- Clear suggestions when not needed
- Use `String::with_capacity()` for large text buffers

## Related Examples and Resources

- **Single Text Input**: `/examples/textinput/` - Simpler single-field example
- **Text Area**: Use `bubbletea-widgets::textarea` for multi-line input
- **Lists**: Use `bubbletea-widgets::list` for selection forms
- **Tables**: Use `bubbletea-widgets::table` for data entry grids

## API Documentation

- [Text Input Documentation](https://docs.rs/bubbletea-widgets/latest/bubbletea_widgets/textinput/index.html)
- [Cursor Documentation](https://docs.rs/bubbletea-widgets/latest/bubbletea_widgets/cursor/index.html)
- [Key Bindings Documentation](https://docs.rs/bubbletea-widgets/latest/bubbletea_widgets/key/index.html)
- [Bubbletea-rs Framework](https://docs.rs/bubbletea-rs/latest/bubbletea_rs/)

## Summary

You've learned how to:
- Create and manage multiple text input fields
- Implement proper focus navigation and keyboard shortcuts
- Style inputs with colors and themes
- Handle form validation and submission
- Troubleshoot common issues
- Extend the basic form with advanced features

This example provides a solid foundation for building any multi-field input interface in your TUI applications. The techniques shown here scale from simple login forms to complex data entry applications.

Happy coding! 🎉

