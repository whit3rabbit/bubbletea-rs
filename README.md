# bubbletea-rs

[![CI](https://github.com/whit3rabbit/bubbletea-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/whit3rabbit/bubbletea-rs/actions/workflows/ci.yml)

A Rust reimagining of the delightful Bubble Tea TUI framework — inspired by, and paying homage to, the original Go project from Charmbracelet.

> Status: WIP. Interfaces and APIs may evolve.

## About

Bubble Tea (Go) popularized a functional, message-passing architecture for building terminal applications. This project explores those ideas in Rust: an ergonomic, async-friendly take on the Model–Update–View pattern, with a focus on correctness, performance, and great developer experience.

- Framework style: Elm-inspired MVU
- Async runtime: Tokio
- Key features: timers, batched/sequence commands, terminal controls, gradient helpers, input handling, memory counters

If you're familiar with the Go version, check our Go → Rust notes in [docs/API.md](docs/API.md).

## Getting Started

### Examples

Each example is a standalone crate. To run one (e.g., `simple`):

```bash
cd examples/simple
cargo run
```

### Tests

```bash
cargo test
```

### Docs and API

See the high-level overview and signatures in [docs/API.md](docs/API.md). Many items include Rustdoc with examples and doctests.

## Inspiration & Credits

- Bubble Tea (Go): https://github.com/charmbracelet/bubbletea
- Charm: https://charm.sh

This work draws heavily from Charmbracelet's design and spirit. If you’re building in Go, you should absolutely use the original Bubble Tea.

## License

See [LICENSE](LICENSE) for details.

---

Part of the broader Bubble Tea ecosystem — with gratitude to the Charm team and community.
