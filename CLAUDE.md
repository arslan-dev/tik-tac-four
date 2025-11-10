# CLAUDE.md - Project Guide

This guide helps AI assistants and developers understand the project structure and workflow.

## Project Overview

5x5 Tic-Tac-Toe console game written in Rust with two game modes (Player vs Player and Player vs Computer).

## Cargo Commands

- `cargo build`: Build the project in debug mode
- `cargo build --release`: Build optimized release version (recommended for gameplay)
- `cargo run`: Compile and run the game in debug mode
- `cargo run --release`: Compile and run the optimized version
- `cargo check`: Quick check for compilation errors without building
- `cargo test`: Run all tests (if any are added)
- `cargo clean`: Remove the target directory

## Code Style

- Use idiomatic Rust patterns (enums, pattern matching, Result/Option types)
- Follow Rust naming conventions:
  - `snake_case` for functions and variables
  - `CamelCase` for types and enums
  - `SCREAMING_SNAKE_CASE` for constants
- Use `const` for ANSI color codes and other compile-time constants
- Prefer immutable variables (`let`) over mutable (`let mut`) when possible
- Use explicit types for public functions, inference for internal logic
- Keep functions focused and single-purpose

## Game Architecture

- **Cell enum**: Represents board cell state (Empty, X, O)
- **GameMode enum**: Player vs Player or Player vs Computer
- **Game struct**: Main game state and logic
  - `board`: 25-element array representing 5x5 grid
  - `current_player`: Current turn (X or O)
  - `game_mode`: Selected game mode
  - `game_active`: Game state flag

## Display System

- ASCII art grid using `+`, `-`, and `|` characters
- ANSI color codes for visual appeal:
  - Cyan: Grid borders and frames
  - Yellow: Cell numbers
  - Red (bold): Player X
  - Blue (bold): Player O
  - Magenta: Computer messages
  - Green: Prompts and success messages

## Win Detection

- Checks 12 patterns total:
  - 5 horizontal rows
  - 5 vertical columns
  - 2 diagonals (main and anti-diagonal)

## AI Strategy

Computer player (O) follows this priority:
1. Try to complete 5 in a row to win
2. Block opponent from winning (4 in a row)
3. Take center positions (6, 7, 8, 11, 12, 13, 16, 17, 18)
4. Random available position

## Workflow

- Always build with `--release` flag before testing gameplay (better performance)
- Run `cargo check` for quick syntax validation during development
- Use `cargo build` to catch all compilation errors and warnings
- The game uses ANSI colors, so test in a terminal that supports them
- Clear warnings about unused constants (they're defined for consistency but may not all be used)

## File Structure

```
.
├── Cargo.toml          # Rust project manifest
├── src/
│   └── main.rs         # Complete game implementation
├── README.md           # User-facing documentation
├── CLAUDE.md           # This file - AI/developer guide
└── .gitignore          # Git ignore rules (target/, Cargo.lock, etc.)
```

## Testing the Game

To quickly test:
```bash
cargo run --release
```

Then:
1. Select game mode (1 or 2)
2. Press Enter to start
3. Enter cell numbers (0-24) to play
4. Type 'quit' to exit anytime

## Adding Features

When adding new features:
1. Maintain the existing ANSI color scheme
2. Keep ASCII grid alignment (7 characters per cell)
3. Update this CLAUDE.md if adding new commands or patterns
4. Test both PvP and PvC modes
5. Ensure game logic maintains the "5 in a row" win condition
