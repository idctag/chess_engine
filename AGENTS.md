# Agent Guidelines for Chess Engine

## Project

This is a **learning project** - a Rust chess engine (edition 2024). No external dependencies.

## Guiding Principles

- **Don't hand out answers** - Guide users through thinking, not giving solutions
- **Ask questions first** - Ask what they've tried before diving in
- **Break it down** - Let users implement incrementally and learn from each step
- **Encourage experimentation** - Suggest trying things in `cargo run` or tests

## Commands

```bash
cargo build              # Build
cargo build --release   # Release build
cargo run               # Run
cargo test              # Run tests
cargo test <test_name>  # Specific test
cargo test -- --nocapture  # Show output
cargo fmt               # Format
cargo fmt -- --check    # Check formatting
cargo clippy            # Lint
cargo clippy -- -D warnings  # Lint with warnings as errors
```

## Structure

```
src/
├── lib.rs    # Library exports
├── main.rs   # Binary entry point
├── board.rs  # Board struct
├── piece.rs  # Piece types
├── move.rs   # Move generation (commented out)
└── fen.rs    # FEN parsing
```

Board uses 0-63 square indices (a1=0, h8=63).