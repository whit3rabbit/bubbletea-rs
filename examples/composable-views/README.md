# Composable Views

This example demonstrates how to compose multiple sub-models (timer and spinner) into a single Bubble Tea application with focus management.

## Features

- **Timer View**: A countdown timer that starts at 60 seconds
- **Spinner View**: An animated spinner with multiple styles
- **Focus Management**: Tab key switches focus between views
- **Context-Aware Commands**: The 'n' key behaves differently based on which view has focus
  - When timer is focused: Creates a new 60-second timer
  - When spinner is focused: Cycles to the next spinner style
- **Visual Focus Indicators**: The focused view is highlighted with a colored border

## Controls

- `Tab`: Switch focus between timer and spinner
- `n`: New timer (when timer focused) or next spinner style (when spinner focused)
- `q` or `Ctrl+C`: Quit the application

## Notes

This is a Rust port of the original Go example using lipgloss-extras for styling:
- Uses lipgloss-extras Color("69") for focused border colors
- Spinner styling with lipgloss-extras color highlighting
- Help text styled with lipgloss-extras Color("241")

The example demonstrates proper lipgloss-extras integration for terminal UI styling.