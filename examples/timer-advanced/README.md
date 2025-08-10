# Advanced Timer Example

<img width="1200" src="./timer-advanced.gif" />

A comprehensive timer application showcasing the **bubbletea-widgets** ecosystem. This example demonstrates how to build sophisticated TUI applications using reusable widget components with proper key binding management, visual styling, and responsive layouts.

## 🚀 Features

### Multiple Timer Presets
- **Quick Timer**: 5-second demonstration timer for testing
- **Pomodoro Work**: 25-minute focused work session
- **Short Break**: 5-minute relaxation break

### Advanced Widget Integration
- ✅ **bubbletea-widgets::timer** - Precise countdown timers with automatic tick management (v0.0.8+ improved timing accuracy)
- ✅ **bubbletea-widgets::key** - Organized key binding system with help text generation
- ✅ **lipgloss-extras** - Rich visual styling with colors and formatting
- ✅ **KeyMap trait** - Expandable help system (short and full views)

### Interactive Controls
- **Space/S**: Start/stop current timer
- **R**: Reset current timer to full duration
- **→/L/N**: Switch to next timer preset
- **←/H/P**: Switch to previous timer preset
- **?**: Toggle between short and full help views
- **Q/Esc/Ctrl+C**: Quit application

### Visual Enhancements
- 🎨 Color-coded timer states (Running, Paused, Expired)
- 📊 Real-time progress bars with gradient colors
- 🔄 Interactive timer switcher with highlights
- 📚 Context-aware help system
- 📱 Responsive layout that adapts to terminal size

## 🏃‍♂️ Quick Start

From the repository root:

```bash
cargo run --example timer-advanced
```

Or using the example package:

```bash
cargo run -p timer-advanced
```

## 🎯 What This Demonstrates

### Widget System Usage
This example shows how to properly integrate multiple `bubbletea-widgets` components:

1. **Timer Management**: Using `timer::Model` for precise countdown functionality
2. **Key Bindings**: Implementing `key::Binding` for organized input handling
3. **Help System**: Using the `KeyMap` trait for automatic help generation
4. **State Management**: Coordinating multiple widget instances
5. **Visual Styling**: Applying `lipgloss-extras` for rich terminal output

### Architecture Patterns
- **Component Separation**: Clean separation of timer logic, key bindings, and UI rendering
- **Message Routing**: Proper forwarding of timer messages with ID-based filtering
- **State Coordination**: Managing multiple timer instances with shared controls
- **Responsive Design**: Adapting UI elements to terminal dimensions

### Modern Rust Practices
- **Type Safety**: Using enums and strong typing for timer presets
- **Builder Patterns**: Leveraging widget builder APIs for configuration
- **Error Handling**: Proper async error handling in the main function
- **Documentation**: Comprehensive inline documentation and examples

## 🔧 Code Architecture

### Core Components

#### `TimerType` Enum
Defines different timer presets with associated durations and metadata:

```rust
enum TimerType {
    Quick,      // 5 seconds - for demos and quick tasks
    Pomodoro,   // 25 minutes - work session  
    Break,      // 5 minutes - short break
}
```

#### `TimerApp` Struct
Main application model containing:
- Multiple timer instances (`Vec<(TimerType, TimerModel)>`)
- UI state (help expansion, terminal dimensions)
- Key binding definitions (`TimerKeyBindings`)

#### `TimerKeyBindings` Struct
Organized key binding definitions implementing the `KeyMap` trait:
- Timer controls (start/stop, reset, navigation)
- Application controls (help, quit)
- Automatic help text generation

### Message Flow

1. **Timer Messages**: `TickMsg`, `TimeoutMsg`, `StartStopMsg` are forwarded to all timers
2. **Key Messages**: Processed through binding system for semantic actions
3. **UI Messages**: Window size changes update responsive layout
4. **Init Messages**: Trigger initial rendering and timer startup

### Visual Rendering

The application renders in distinct sections:
- **Timer Info**: Name, description, and current status
- **Time Display**: Formatted countdown with visual status indicators
- **Progress Bar**: Animated progress visualization
- **Timer Selector**: Interactive preset switcher
- **Help System**: Context-sensitive help text

## 📚 Usage Examples

### Adding New Timer Presets

```rust
// Add to TimerType enum
enum TimerType {
    Quick,
    Pomodoro, 
    Break,
    Custom(Duration), // New custom duration support
}

// Update duration method
fn duration(self) -> Duration {
    match self {
        // ... existing cases
        TimerType::Custom(d) => d,
    }
}
```

### Extending Key Bindings

```rust
// Add new binding to TimerKeyBindings
pub struct TimerKeyBindings {
    // ... existing bindings
    pub custom_action: Binding,
}

// Implement in constructor
impl TimerKeyBindings {
    pub fn new() -> Self {
        Self {
            // ... existing bindings
            custom_action: new_binding(vec![
                with_keys_str(&["c"]),
                with_help("c", "custom action"),
            ]),
        }
    }
}
```

### Customizing Visual Styles

```rust
// Define custom color scheme
let primary_color = Color::from("#FF6B6B");
let secondary_color = Color::from("#4ECDC4");
let accent_color = Color::from("#45B7D1");

// Apply to status rendering
let status_style = Style::new()
    .foreground(primary_color)
    .bold();
```

## 🎨 Styling System

The example demonstrates `lipgloss-extras` usage patterns:

### Color Schemes
- **Primary**: `#FF75B7` (Pink) - Timer names and highlights
- **Success**: `#00FF00` (Green) - Running status
- **Warning**: `#FFAA00` (Orange) - Paused status  
- **Error**: `#FF0000` (Red) - Expired status
- **Info**: `#00AAFF` (Blue) - Progress bars and selections

### Typography
- **Bold**: Timer names and important status messages
- **Dimmed**: Descriptions and secondary text
- **Background Highlights**: Active timer selection

### Layout Elements
- **Progress Bars**: Unicode block characters with gradient colors
- **Separators**: Horizontal lines for visual section breaks
- **Padding**: Consistent spacing for readable layouts

## 🔄 Comparison with Basic Timer

### Basic Timer Example
- ❌ Single 5-second timer only
- ❌ Basic text-only display
- ❌ Limited help system
- ❌ No visual progress indicators
- ❌ No timer switching capability

### Advanced Timer Example
- ✅ Multiple timer types with easy switching
- ✅ Rich visual styling with colors and progress bars
- ✅ Expandable help system with organized display
- ✅ Visual progress indicators and status styling
- ✅ Responsive layout and interactive UI elements

## 🛠️ Development Tips

### Widget Integration
1. **Always forward messages**: Let widgets handle their own message filtering
2. **Use builder patterns**: Leverage widget configuration builders for clean setup
3. **Implement KeyMap**: Use the trait system for consistent help generation
4. **Coordinate state**: Maintain shared state while respecting widget boundaries

### Performance Considerations
- Timer tick messages are filtered by ID, preventing cross-timer interference
- Progress bar calculations are cached based on terminal width
- Help text is only generated when help state changes
- Color styles are created once and reused across renders

### Testing and Debugging
- Use the Quick Timer (5 seconds) for rapid testing cycles
- Help system provides real-time key binding documentation
- Visual status indicators make state changes immediately apparent
- Alt-screen mode provides clean testing environment

## 📖 Further Reading

- [bubbletea-widgets Documentation](https://docs.rs/bubbletea-widgets/)
- [lipgloss-extras Styling Guide](https://docs.rs/lipgloss-extras/)  
- [Bubble Tea Architecture](https://github.com/charmbracelet/bubbletea)
- [Go Bubble Tea Examples](https://github.com/charmbracelet/bubbletea/tree/master/examples)

## 🤝 Contributing

This example serves as a reference for:
- Widget integration patterns
- Key binding system usage
- Visual styling techniques
- State management in complex TUI applications

Feel free to extend this example or use it as a foundation for your own timer applications!

## Files

- `main.rs` — Advanced timer implementation with multiple presets
- `Cargo.toml` — Example dependencies with styling libraries
- `README.md` — This comprehensive documentation