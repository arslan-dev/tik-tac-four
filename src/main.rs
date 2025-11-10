use rand::Rng;
use std::io::{self, Write};
use std::process;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

impl Cell {
    fn to_string(&self) -> &str {
        match self {
            Cell::Empty => " ",
            Cell::X => "X",
            Cell::O => "O",
        }
    }
}

#[derive(PartialEq)]
enum GameMode {
    PlayerVsPlayer,
    PlayerVsComputer,
}

struct Game {
    board: [Cell; 16],
    current_player: Cell,
    game_mode: GameMode,
    game_active: bool,
}

impl Game {
    fn new(mode: GameMode) -> Self {
        Game {
            board: [Cell::Empty; 16],
            current_player: Cell::X,
            game_mode: mode,
            game_active: true,
        }
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn display_board(&self) {
        println!();
        println!("  ╔═════╦═════╦═════╦═════╗");

        for row in 0..4 {
            print!("  ║");
            for col in 0..4 {
                let index = row * 4 + col;
                let cell_str = match self.board[index] {
                    Cell::Empty => format!(" {:2} ", index),
                    Cell::X => "  X ".to_string(),
                    Cell::O => "  O ".to_string(),
                };
                print!("{}", cell_str);
                print!("║");
            }
            println!();

            if row < 3 {
                println!("  ╠═════╬═════╬═════╬═════╣");
            }
        }

        println!("  ╚═════╩═════╩═════╩═════╝");
        println!();
    }

    fn display_instructions() {
        println!("\n╔══════════════════════════════════════════════════╗");
        println!("║         4x4 TIC-TAC-TOE GAME                     ║");
        println!("╚══════════════════════════════════════════════════╝");
        println!("\n📋 Game Rules:");
        println!("   • The board is 4x4 (16 cells)");
        println!("   • Players take turns placing X or O");
        println!("   • Get 4 in a row to win (horizontal, vertical, or diagonal)");
        println!("   • Cells are numbered 0-15");
        println!("\n🎮 How to play:");
        println!("   • Enter the cell number (0-15) when prompted");
        println!("   • Type 'quit' to exit the game");
        println!("\n══════════════════════════════════════════════════");
    }

    fn get_game_mode() -> GameMode {
        println!("\n🎯 Select Game Mode:");
        println!("   1. Player vs Player");
        println!("   2. Player vs Computer");

        loop {
            print!("\nEnter your choice (1 or 2): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "1" => {
                    println!("\n✓ Player vs Player mode selected!");
                    return GameMode::PlayerVsPlayer;
                }
                "2" => {
                    println!("\n✓ Player vs Computer mode selected!");
                    println!("  You are X, Computer is O");
                    return GameMode::PlayerVsComputer;
                }
                _ => println!("❌ Invalid choice. Please enter 1 or 2."),
            }
        }
    }

    fn is_valid_move(&self, position: usize) -> bool {
        position < 16 && self.board[position] == Cell::Empty
    }

    fn make_move(&mut self, position: usize) {
        self.board[position] = self.current_player;
    }

    fn check_winner(&self) -> Option<Vec<usize>> {
        let player = self.current_player;
        let win_patterns: Vec<Vec<usize>> = vec![
            // Horizontal
            vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11], vec![12, 13, 14, 15],
            // Vertical
            vec![0, 4, 8, 12], vec![1, 5, 9, 13], vec![2, 6, 10, 14], vec![3, 7, 11, 15],
            // Diagonal
            vec![0, 5, 10, 15], vec![3, 6, 9, 12],
        ];

        for pattern in win_patterns {
            if pattern.iter().all(|&i| self.board[i] == player) {
                return Some(pattern);
            }
        }
        None
    }

    fn is_board_full(&self) -> bool {
        self.board.iter().all(|&cell| cell != Cell::Empty)
    }

    fn get_player_move(&self) -> Option<usize> {
        loop {
            let player_symbol = self.current_player.to_string();
            print!("Player {}, enter cell number (0-15) or 'quit': ", player_symbol);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input == "quit" {
                println!("\n👋 Thanks for playing!");
                process::exit(0);
            }

            match input.parse::<usize>() {
                Ok(pos) if self.is_valid_move(pos) => return Some(pos),
                Ok(_) => println!("❌ Invalid move! Cell must be between 0-15 and empty."),
                Err(_) => println!("❌ Please enter a valid number or 'quit'."),
            }
        }
    }

    fn find_best_move(&self, player: Cell) -> Option<usize> {
        let win_patterns: Vec<Vec<usize>> = vec![
            // Horizontal
            vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11], vec![12, 13, 14, 15],
            // Vertical
            vec![0, 4, 8, 12], vec![1, 5, 9, 13], vec![2, 6, 10, 14], vec![3, 7, 11, 15],
            // Diagonal
            vec![0, 5, 10, 15], vec![3, 6, 9, 12],
        ];

        for pattern in win_patterns {
            let values: Vec<Cell> = pattern.iter().map(|&i| self.board[i]).collect();
            let player_count = values.iter().filter(|&&c| c == player).count();
            let empty_count = values.iter().filter(|&&c| c == Cell::Empty).count();

            if player_count == 3 && empty_count == 1 {
                for &pos in &pattern {
                    if self.board[pos] == Cell::Empty {
                        return Some(pos);
                    }
                }
            }
        }
        None
    }

    fn get_computer_move(&self) -> usize {
        print!("\n🤖 Computer is thinking");
        io::stdout().flush().unwrap();

        for _ in 0..3 {
            std::thread::sleep(std::time::Duration::from_millis(300));
            print!(".");
            io::stdout().flush().unwrap();
        }
        println!();

        // Try to win
        if let Some(pos) = self.find_best_move(Cell::O) {
            println!("   Computer plays at position {}", pos);
            return pos;
        }

        // Block opponent from winning
        if let Some(pos) = self.find_best_move(Cell::X) {
            println!("   Computer plays at position {}", pos);
            return pos;
        }

        // Take center cells if available
        let center_cells = vec![5, 6, 9, 10];
        let available_center: Vec<usize> = center_cells
            .iter()
            .copied()
            .filter(|&i| self.board[i] == Cell::Empty)
            .collect();

        if !available_center.is_empty() {
            let pos = available_center[rand::thread_rng().gen_range(0..available_center.len())];
            println!("   Computer plays at position {}", pos);
            return pos;
        }

        // Take any available cell
        let available_cells: Vec<usize> = (0..16)
            .filter(|&i| self.board[i] == Cell::Empty)
            .collect();

        let pos = available_cells[rand::thread_rng().gen_range(0..available_cells.len())];
        println!("   Computer plays at position {}", pos);
        pos
    }

    fn switch_player(&mut self) {
        self.current_player = if self.current_player == Cell::X {
            Cell::O
        } else {
            Cell::X
        };
    }

    fn play(&mut self) {
        while self.game_active {
            self.clear_screen();
            self.display_board();

            // Get move based on game mode and current player
            let position = if self.game_mode == GameMode::PlayerVsComputer
                && self.current_player == Cell::O
            {
                self.get_computer_move()
            } else {
                self.get_player_move().unwrap()
            };

            // Make the move
            self.make_move(position);

            // Check for winner
            if let Some(pattern) = self.check_winner() {
                self.clear_screen();
                self.display_board();
                println!("╔══════════════════════════════════════════════════╗");
                if self.game_mode == GameMode::PlayerVsComputer && self.current_player == Cell::O {
                    println!("║  🤖 Computer (O) wins!                           ║");
                } else {
                    println!("║  🎉 Player {} wins!                               ║",
                        self.current_player.to_string());
                }
                println!("╚══════════════════════════════════════════════════╝");
                println!("\n🏆 Winning positions: {:?}", pattern);
                self.game_active = false;
                break;
            }

            // Check for draw
            if self.is_board_full() {
                self.clear_screen();
                self.display_board();
                println!("╔══════════════════════════════════════════════════╗");
                println!("║  🤝 It's a draw!                                 ║");
                println!("╚══════════════════════════════════════════════════╝");
                self.game_active = false;
                break;
            }

            // Switch player
            self.switch_player();

            // Small delay after computer move
            if self.game_mode == GameMode::PlayerVsComputer && self.current_player == Cell::X {
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }

    fn play_again() -> bool {
        loop {
            print!("\n🎮 Play again? (yes/no): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            match input.as_str() {
                "yes" | "y" => return true,
                "no" | "n" => return false,
                _ => println!("❌ Please enter 'yes' or 'no'"),
            }
        }
    }
}

fn main() {
    println!("\n╔══════════════════════════════════════════════════╗");
    println!("║  Welcome to 4x4 Tic-Tac-Toe!                    ║");
    println!("╚══════════════════════════════════════════════════╝");

    loop {
        Game::display_instructions();
        let mode = Game::get_game_mode();

        println!("\n⏎ Press Enter to start the game...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut game = Game::new(mode);
        game.play();

        if !Game::play_again() {
            println!("\n╔══════════════════════════════════════════════════╗");
            println!("║  Thanks for playing! Goodbye! 👋                 ║");
            println!("╚══════════════════════════════════════════════════╝\n");
            break;
        }
    }
}
