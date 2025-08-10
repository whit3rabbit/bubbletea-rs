# bubbletea-rs

[![CI](https://github.com/whit3rabbit/bubbletea-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/whit3rabbit/bubbletea-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/bubbletea-rs.svg)](https://crates.io/crates/bubbletea-rs)

A Rust reimagining of the delightful Bubble Tea TUI framework — inspired by, and paying homage to, the original Go project from Charmbracelet.

Build delightful terminal user interfaces with the Model-View-Update pattern, async commands, and rich styling capabilities.

> Status: Active development. Core APIs are stabilizing, but some interfaces may still evolve.

## The Ecosystem

The Rust Bubble Tea ecosystem consists of three complementary crates:

| Crate | Repository | Purpose |
|-------|------------|---------|
| **bubbletea-rs** | [bubbletea-rs](https://github.com/whit3rabbit/bubbletea-rs) | Core MVU framework with async runtime |
| **bubbletea-widgets** | [bubbles-rs](https://github.com/whit3rabbit/bubbles-rs) | Pre-built UI components (spinners, inputs, tables, etc.) |
| **lipgloss-extras** | [lipgloss-rs](https://github.com/whit3rabbit/lipgloss-rs) | Styling framework with colors, layouts, and rich text |

### Quick Start

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
bubbletea-rs = "0.0.6"
bubbletea-widgets = "0.1.6" 
lipgloss-extras = { version = "0.0.8", features = ["full"] }
```

Then create your first TUI app:

```rust
use bubbletea_rs::{model::Model, program::ProgramBuilder, Msg};
use lipgloss_extras::lipgloss::{Style, Color};

struct MyModel {
    counter: i32,
}

impl Model for MyModel {
    fn update(&mut self, msg: Msg) -> bubbletea_rs::Result<()> {
        // Handle key presses, timer ticks, etc.
        Ok(())
    }

    fn view(&self) -> String {
        Style::new()
            .foreground(Color::from("#FF7CCB"))
            .render(&format!("Counter: {}", self.counter))
    }
}

#[tokio::main]
async fn main() -> bubbletea_rs::Result<()> {
    let model = MyModel { counter: 0 };
    ProgramBuilder::new(model).build()?.run().await
}
```

## About

Bubble Tea (Go) popularized a functional, message-passing architecture for building terminal applications. This project explores those ideas in Rust: an ergonomic, async-friendly take on the Model–Update–View pattern, with a focus on correctness, performance, and great developer experience.

### Core Features

- **Model-View-Update Architecture**: Clean separation of state, rendering, and updates
- **Async-First Design**: Built on Tokio with async commands and non-blocking operations  
- **Rich Styling**: Full color support, gradients, borders, and layouts via lipgloss-extras
- **Pre-built Components**: 13+ widgets including spinners, inputs, tables, progress bars
- **Command System**: Timers, HTTP requests, batch operations, and custom async commands
- **Terminal Controls**: Mouse support, alternate screen, window sizing, focus management
- **Type Safety**: Leverages Rust's type system for reliable, memory-safe TUIs

### Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Model         │    │    Commands      │    │     View        │
│   (State)       │    │   (Async Ops)    │    │  (Rendering)    │
├─────────────────┤    ├──────────────────┤    ├─────────────────┤
│ • App state     │    │ • Timers         │    │ • lipgloss      │
│ • Business      │    │ • HTTP requests  │    │ • Styled text   │
│   logic         │    │ • File I/O       │    │ • Layouts       │
│ • Updates       │    │ • Custom async   │    │ • Components    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                       │                       │
         │                       ▼                       │
         │              ┌─────────────────┐              │
         └──────────────►│   bubbletea-rs  │◄─────────────┘
                        │   Event Loop    │
                        └─────────────────┘
```

If you're familiar with the Go version, check our Go → Rust notes in the [API documentation](docs/API-BUBBLETEA-RS.md).

## Getting Started

### Examples

Each example is a standalone crate showcasing different features:

| Example | Description | Features |
|---------|-------------|----------|
| **simple** | Basic counter app | Model-View-Update basics |
| **spinner** | Loading indicators | bubbletea-widgets, styling |
| **textinput** | Text input forms | User input, validation |
| **progress** | Progress bars | Gradients, animations |
| **chat** | Chat interface | Complex layouts, scrolling |
| **table** | Data tables | Sorting, selection, pagination |
| **help** | Help systems | Key bindings, auto-generation |

To run any example:

```bash
cd examples/simple  # or any example directory
cargo run
```

Or run directly from the workspace root:

```bash
cargo run --example simple
```

### Development

**Run tests:**
```bash
cargo test
```

**Format and lint:**
```bash
cargo fmt
cargo clippy
```

**Generate documentation:**
```bash
cargo doc --open
```

### Documentation

- **[API Reference](docs/API-BUBBLETEA-RS.md)** - Core bubbletea-rs APIs and patterns  
- **[Widgets Guide](docs/API-BUBBLES-RS.md)** - Available components and usage
- **[Styling Guide](docs/API-LIPGLOSS.md)** - Colors, layouts, and theming
- **[CLAUDE.md](CLAUDE.md)** - Development guidelines and patterns

## Contributing

Contributions are welcome! This project aims to:

- Maintain 1:1 API compatibility with Bubble Tea (Go) where possible
- Provide idiomatic Rust patterns and safety guarantees  
- Support the full ecosystem of widgets and styling capabilities
- Keep performance characteristics suitable for real applications

Please see [CLAUDE.md](CLAUDE.md) for development guidelines and architectural notes.

## Ecosystem Status

| Component | Status | Version | Notes |
|-----------|--------|---------|-------|
| bubbletea-rs | ✅ Active | v0.0.6 | Core framework stable |
| bubbletea-widgets | ✅ Active | v0.0.6 | 13+ widgets available |
| lipgloss-extras | ✅ Active | v0.0.7 | Full styling support |

## Inspiration & Credits

- **[Bubble Tea (Go)](https://github.com/charmbracelet/bubbletea)** - The original and inspiration
- **[Charm](https://charm.sh)** - Beautiful CLI tools and design philosophy
- **[Elm Architecture](https://guide.elm-lang.org/architecture/)** - Model-View-Update pattern

This work draws heavily from Charmbracelet's design and spirit. If you're building in Go, you should absolutely use the original Bubble Tea. This Rust implementation aims to bring the same joy and productivity to the Rust ecosystem.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

🫧 Built with bubbles, styled with charm, powered by Rust.
