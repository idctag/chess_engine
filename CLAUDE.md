# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Context

This is a **learning project** — a Rust chess engine (edition 2024, no external dependencies). Per `AGENTS.md`, the collaboration style is teaching-oriented:

- Don't hand out complete solutions; guide the user through the reasoning.
- Ask what they've tried before diving in.
- Prefer incremental steps the user implements themselves.
- Suggest experiments via `cargo run` or tests rather than just explaining.

## Commands

```bash
cargo build                  # Build
cargo run                    # Run the binary (prints bitboards from the initial position)
cargo test                   # Run tests
cargo test <name>            # Run a specific test
cargo test -- --nocapture    # Show println! output during tests
cargo fmt                    # Format
cargo clippy -- -D warnings  # Lint, warnings as errors
```

## Architecture

The board uses a **bitboard** representation. Key invariants to keep in mind across files:

- `Board.pieces: [[u64; 6]; 2]` — indexed as `pieces[color][piece_type]`.
  - Color index: `0 = White`, `1 = Black` (see `fen.rs` where uppercase → 0).
  - Piece index: `0=pawn, 1=knight, 2=bishop, 3=rook, 4=queen, 5=king` (see the match in `fen.rs::populate_board`).
  - Note: this numeric ordering **does not match** the declaration order in `piece.rs` (`King, Queen, Rook, Bishop, Knight, Pawn`). If you convert between the `Piece` enum and array indices, don't assume `as usize` works.
- Square indexing: 0–63. FEN is parsed **right-to-left, top-to-bottom**: `populate_board` starts at `square = 63` (pre-decrements to 63 on first iter) and walks down, so bit `1 << square` corresponds to that square. Digits `'1'..'8'` skip empty squares by further decrementing.
- `Board::new()` loads the standard initial position from `INITIAL_BOARD_FEN`; `Board::empty_board()` gives an all-zero bitboard set.

## Module Layout

- `src/lib.rs` — module exports. `move` is declared but commented out; `src/move.rs` exists but is empty (move generation is not yet implemented).
- `src/board.rs` — `Board` struct + FEN entry points.
- `src/fen.rs` — FEN parsing and `FenError`. Currently only the piece-placement field is parsed; side-to-move / castling / en passant / clocks are ignored, and `Board.turn` is left as `None`.
- `src/piece.rs` — `Color` and `Piece` enums.
- `src/main.rs` — demo binary that prints white/black pawn bitboards.
