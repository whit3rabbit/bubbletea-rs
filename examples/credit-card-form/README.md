# Credit Card Form

A credit card form example demonstrating input validation and field navigation using bubbletea-widgets.

## Features

- **Three input fields**: Credit card number, expiry date, and CVV
- **Auto-formatting**: Credit card number auto-formats with spaces (4505 1234 5678 9012), expiry date auto-inserts slash (MM/YY)
- **Input validation**: Real-time validation for each field
- **Navigation**: Tab/Shift+Tab to move between fields
- **Styled interface**: Hot pink labels and dark gray styling

## Running

```bash
cargo run --bin credit-card-form
```

## Controls

- `Tab` / `Ctrl+N` - Next field
- `Shift+Tab` / `Ctrl+P` - Previous field
- `Enter` - Next field (or submit if on last field)
- `Esc` / `Ctrl+C` - Quit

## Auto-formatting & Validation Rules

- **Credit Card Number**: Auto-formats as you type to add spaces every 4 digits (e.g., typing "4505123456789012" becomes "4505 1234 5678 9012")
- **Expiry Date**: Auto-inserts slash after 2 digits (e.g., typing "1225" becomes "12/25")
- **CVV**: 3 digits only (e.g., "123")

<img width="800" src="./credit-card-form.gif" />