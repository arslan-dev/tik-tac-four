# 4x4 Tic-Tac-Toe Console Game

A console-based 4x4 tic-tac-toe game written in Rust with two game modes.

## Features

- **4x4 Game Board**: Win by getting 4 X's or O's in a row
- **Two Game Modes**:
  - Player vs Player: Play against a friend
  - Player vs Computer: Play against an AI opponent
- **Smart AI**: Computer can win and block opponent moves
- **Beautiful Console UI**: Unicode box-drawing characters for elegant display
- **Win Detection**: Horizontal, vertical, and diagonal win patterns

## Requirements

- Rust 1.56 or higher (install from [rustup.rs](https://rustup.rs/))

## How to Play

1. **Clone the repository**:
   ```bash
   git clone <your-repo-url>
   cd tik-tac-four
   git checkout claude/tic-tac-toe-four-player-modes-011CUz55c48yp3orue59S91m
   ```

2. **Build and run the game**:
   ```bash
   cargo run --release
   ```

   Or build and run separately:
   ```bash
   cargo build --release
   ./target/release/tik-tac-four
   ```

3. **Game Instructions**:
   - Select your game mode (1 for PvP, 2 for PvC)
   - Cells are numbered 0-15
   - Enter the cell number where you want to place your mark
   - Get 4 in a row (horizontal, vertical, or diagonal) to win
   - Type 'quit' at any time to exit

## Game Board Layout

```
  ╔═════╦═════╦═════╦═════╗
  ║  0  ║  1  ║  2  ║  3  ║
  ╠═════╬═════╬═════╬═════╣
  ║  4  ║  5  ║  6  ║  7  ║
  ╠═════╬═════╬═════╬═════╣
  ║  8  ║  9  ║ 10  ║ 11  ║
  ╠═════╬═════╬═════╬═════╣
  ║ 12  ║ 13  ║ 14  ║ 15  ║
  ╚═════╩═════╩═════╩═════╝
```

## Winning Patterns

- **Horizontal**: 0-1-2-3, 4-5-6-7, 8-9-10-11, 12-13-14-15
- **Vertical**: 0-4-8-12, 1-5-9-13, 2-6-10-14, 3-7-11-15
- **Diagonal**: 0-5-10-15, 3-6-9-12

## Computer AI Strategy

The computer AI follows this strategy:
1. Try to complete 4 in a row to win
2. Block the player from winning
3. Take center positions for strategic advantage
4. Choose random available positions

## Example Gameplay

```
Player X, enter cell number (0-15) or 'quit': 5

  ╔═════╦═════╦═════╦═════╗
  ║  0  ║  1  ║  2  ║  3  ║
  ╠═════╬═════╬═════╬═════╣
  ║  4  ║  X  ║  6  ║  7  ║
  ╠═════╬═════╬═════╬═════╣
  ║  8  ║  9  ║ 10  ║ 11  ║
  ╠═════╬═════╬═════╬═════╣
  ║ 12  ║ 13  ║ 14  ║ 15  ║
  ╚═════╩═════╩═════╩═════╝
```

## License

MIT License
