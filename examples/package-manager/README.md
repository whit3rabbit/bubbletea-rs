# Package Manager

<img width="1200" src="./package-manager.gif" />

A package installer simulation demonstrating advanced bubbletea-rs patterns and techniques. This example serves as a comprehensive guide for developers learning bubbletea-rs.

## Key Learning Patterns Demonstrated

### 🎨 **Custom UI Components**
- **Custom Spinner**: Hand-built component with state management and lipgloss styling
- **Animated Progress Bar**: Smooth percentage animations using `gradient_filled_segment()`
- **Component Lifecycle**: Init, update, and render patterns for reusable components

### 📊 **State Management**
- **Model-Driven UI**: All state lives in the model, including completed packages list
- **Alternative to printf()**: Shows how to handle "printed above UI" pattern without printf
- **Responsive Layout**: Dynamic width calculations and text truncation

### ⚡ **Command & Animation Patterns**
- **Multi-Command Coordination**: Using `batch()` for concurrent operations
- **Animation Loops**: Self-sustaining tick loops for spinners
- **Conditional Commands**: Progress bars that only animate when needed
- **Async Simulation**: Using `tick()` for realistic timing delays

### 🎭 **Advanced Styling**
- **Visual vs String Length**: Handling ANSI escape codes in layout calculations  
- **Consistent Colors**: Using bubbletea-rs color palette (matching Charm defaults)
- **Gap Calculations**: Proper spacing for complex multi-component layouts

## Usage

```bash
cargo run --bin package-manager
```

## Controls

- **q** or **Esc**: Quit the program
- **Ctrl+C**: Force quit

The program will automatically quit after all packages are installed.

## Code Structure for Learning

The source code is extensively commented with `## bubbletea-rs Pattern:` sections that explain:

- Why each pattern is used
- How it differs from the Go version  
- Common pitfalls and solutions
- Performance considerations

This makes it an ideal reference for developers building their own bubbletea-rs applications.

## What You'll See

```
✓ vegeutils-2.4.7
✓ libgardening-8.1.3  
✓ currykit-5.9.2
⠦ Installing fullenglish-4.3.2    ████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  4/29
```

- **List Building**: Completed packages accumulate above with green checkmarks
- **Live Progress**: Spinner, progress bar, and package counter update in real-time
- **Responsive Layout**: Text truncates gracefully on narrow terminals
- **Smooth Animations**: 60fps progress bar and 10fps spinner animations