# 5x5 Tic-Tac-Toe Console Game

A console-based 5x5 tic-tac-toe game written in Rust with two game modes.

## Features

- **5x5 Game Board**: Win by getting 5 X's or O's in a row
- **Two Game Modes**:
  - Player vs Player: Play against a friend
  - Player vs Computer: Play against an AI opponent
- **Smart AI**: Computer can win and block opponent moves
- **Beautiful Console UI**: ASCII art with colorful ANSI escape codes
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
   - Cells are numbered 0-24
   - Enter the cell number where you want to place your mark
   - Get 5 in a row (horizontal, vertical, or diagonal) to win
   - Type 'quit' at any time to exit

## Game Board Layout

```
  +-------+-------+-------+-------+-------+
  |   0   |   1   |   2   |   3   |   4   |
  +-------+-------+-------+-------+-------+
  |   5   |   6   |   7   |   8   |   9   |
  +-------+-------+-------+-------+-------+
  |  10   |  11   |  12   |  13   |  14   |
  +-------+-------+-------+-------+-------+
  |  15   |  16   |  17   |  18   |  19   |
  +-------+-------+-------+-------+-------+
  |  20   |  21   |  22   |  23   |  24   |
  +-------+-------+-------+-------+-------+
```

The game uses ANSI colors:
- Grid borders: Cyan
- Cell numbers: Yellow
- Player X: Red (bold)
- Player O: Blue (bold)
- Computer moves: Magenta

## Winning Patterns

- **Horizontal**: 0-1-2-3-4, 5-6-7-8-9, 10-11-12-13-14, 15-16-17-18-19, 20-21-22-23-24
- **Vertical**: 0-5-10-15-20, 1-6-11-16-21, 2-7-12-17-22, 3-8-13-18-23, 4-9-14-19-24
- **Diagonal**: 0-6-12-18-24, 4-8-12-16-20

## Computer AI Strategy

The computer AI follows this strategy:
1. Try to complete 5 in a row to win
2. Block the player from winning
3. Take center positions (6, 7, 8, 11, 12, 13, 16, 17, 18) for strategic advantage
4. Choose random available positions

## Example Gameplay

```
Player X, enter cell number (0-24) or 'quit': 12

  +-------+-------+-------+-------+-------+
  |   0   |   1   |   2   |   3   |   4   |
  +-------+-------+-------+-------+-------+
  |   5   |   6   |   7   |   8   |   9   |
  +-------+-------+-------+-------+-------+
  |  10   |  11   |   X   |  13   |  14   |
  +-------+-------+-------+-------+-------+
  |  15   |  16   |  17   |  18   |  19   |
  +-------+-------+-------+-------+-------+
  |  20   |  21   |  22   |  23   |  24   |
  +-------+-------+-------+-------+-------+
```

Note: In the actual game, the grid and text are displayed in vibrant colors!

## License

MIT License
