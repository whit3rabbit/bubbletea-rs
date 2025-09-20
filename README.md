# bubbletea-rs

[![CI](https://github.com/whit3rabbit/bubbletea-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/whit3rabbit/bubbletea-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/bubbletea-rs.svg)](https://crates.io/crates/bubbletea-rs)

A Rust reimagining of the delightful Bubble Tea TUI framework — inspired by, and paying homage to, the original Go project from Charmbracelet.

Build delightful terminal user interfaces with the Model-View-Update pattern, async commands, and rich styling capabilities.

> Status: Active development. Core APIs are stabilizing, but some interfaces may still evolve.

## Examples

I tried to match all the examples from the bubbletea repository in rust. You can view the examples here:

https://github.com/whit3rabbit/bubbletea-rs/blob/main/examples/README.md

## The Ecosystem

The Rust Bubble Tea ecosystem consists of three complementary crates:

| Crate | Repository | Purpose |
|-------|------------|---------|
| **bubbletea-rs** | [bubbletea-rs](https://github.com/whit3rabbit/bubbletea-rs) | Core MVU framework with async runtime |
| **bubbletea-widgets** | [bubbles-rs](https://github.com/whit3rabbit/bubbles-rs) | Pre-built UI components (spinners, inputs, tables, etc.) |
| **lipgloss-extras** | [lipgloss-rs](https://github.com/whit3rabbit/lipgloss-rs) | Styling framework with colors, layouts, and rich text |

### Crates.io:

All crates are published to crates.io:

* https://crates.io/crates/bubbletea-rs
* https://crates.io/crates/bubbletea-widgets
* https://crates.io/crates/lipgloss
* > https://crates.io/crates/lipgloss-extras
* > https://crates.io/crates/lipgloss-list
* > https://crates.io/crates/lipgloss-table

### Quick Start

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
bubbletea-rs = "0.0.8"
bubbletea-widgets = "0.1.11" 
lipgloss-extras = { version = "0.1.0", features = ["full"] }
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
         └─────────────►│   bubbletea-rs  │◄─────────────┘
                        │   Event Loop    │
                        └─────────────────┘
```

## Getting Started

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

## Ecosystem Status

| Component | Status | Version | Notes |
|-----------|--------|---------|-------|
| bubbletea-rs | ✅ Active | v0.0.8 | Core framework stable |
| bubbletea-widgets | ✅ Active | v0.1.11 | 13+ widgets available |
| lipgloss-extras | ✅ Active | v0.1.0 | Full styling support |

## Inspiration & Credits

- **[Bubble Tea (Go)](https://github.com/charmbracelet/bubbletea)** - The original and inspiration
- **[Charm](https://charm.sh)** - Beautiful CLI tools and design philosophy
- **[Elm Architecture](https://guide.elm-lang.org/architecture/)** - Model-View-Update pattern

This work draws heavily from Charmbracelet's design and spirit. If you're building in Go, you should absolutely use the original Bubble Tea. This Rust implementation aims to bring the same joy and productivity to the Rust ecosystem.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

🫧 Built with bubbles, styled with charm, powered by Rust.
