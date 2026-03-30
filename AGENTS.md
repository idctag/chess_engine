# Agent Guidelines for Chess Engine

## Project Overview

This is a Rust-based chess engine. The codebase uses Rust 2024 edition.

## Build Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run the project
cargo run

# Run with specific arguments
cargo run -- arg1 arg2

# Clean build artifacts
cargo clean
```

## Testing Commands

```bash
# Run all tests
cargo test

# Run a specific test
cargo test <test_name>

# Run tests with output
cargo test -- --nocapture

# Run tests including doc tests
cargo test --doc

# Run tests with thread count
cargo test -- --test-threads=1
```

## Linting and Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy (linter)
cargo clippy

# Run clippy with all warnings
cargo clippy -- -D warnings

# Check for security issues
cargo audit
```

## Code Style Guidelines

### Naming Conventions

- **Types/Enums**: PascalCase (e.g., `Piece`, `Color`, `Board`)
- **Functions/Methods**: snake_case (e.g., `from_fen`, `parse_piece_placement`)
- **Variables/Fields**: snake_case (e.g., `initial_board`, `piece`)
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `INITIAL_BOARD`)
- **Modules**: snake_case

### Types and Structures

```rust
// Enums for types with fixed variants
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

enum Color {
    White,
    Black,
}

// Structs for composite types
struct Square {
    piece: Option<(Piece, Color)>,
}

// Error handling with Result types
enum FenError {
    InvalidPiece,
    InvalidStructure,
}
```

### Error Handling

- Use `Result<T, E>` for fallible operations
- Use custom error enums for domain-specific errors
- Return early on error conditions
- Use `?` operator for error propagation
- Avoid `unwrap()` and `expect()` in production code

### Imports

- Group imports: standard library first, then third-party, then local
- Use `use` statements for frequently used items
- Avoid wildcard imports unless necessary

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::board::{Board, Square};
```

### Formatting

- 4 spaces for indentation (Rust default)
- Maximum line length: 100 characters (default)
- No trailing whitespace
- Single blank line between function definitions
- No unnecessary blank lines within functions

### Documentation

- Document public APIs with doc comments (`///`)
- Include examples in documentation where helpful
- Document all exported items

```rust
/// Parses a FEN string and creates a Board.
///
/// # Arguments
/// * `raw` - A FEN-formatted string
///
/// # Example
/// ```
/// let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0");
/// ```
pub fn from_fen(raw: &str) -> Result<Board, FenError> {
    // ...
}
```

### General Rust Idioms

- Prefer `impl Trait` over generic parameters when concrete return type doesn't matter
- Use pattern matching exhaustively
- Use `match` with `Some`/`None` instead of `if let`
- Use iterators and iterator adapters over manual loops where idiomatic
- Use `Arc<Mutex<T>>` or `RwLock` for shared mutable state
- Clone only when necessary; prefer borrowing

### Chess-Specific Patterns

- Board representation: Use 0-63 indices for squares (a1=0, h8=63)
- Bitboards for efficient piece position representation
- FEN string parsing for position initialization
- Algebraic notation for move representation

## File Organization

```
src/
├── main.rs           # Entry point
├── board.rs          # Board and square structures
├── piece.rs          # Piece types and utilities
├── move.rs           # Move generation and representation
├── fen.rs            # FEN parsing and serialization
└── lib.rs            # Library entry point (if applicable)
```

## Common Patterns

### Board Representation

```rust
// Using Option for empty squares
struct Square {
    piece: Option<(Piece, Color)>,
}

// Index-based board access
fn square_index(file: u8, rank: u8) -> usize {
    (rank * 8 + file) as usize
}
```

### FEN Parsing

```rust
fn parse_fen(fen: &str) -> Result<Board, FenError> {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.len() != 6 {
        return Err(FenError::InvalidStructure);
    }
    // ... parse each field
}
```
