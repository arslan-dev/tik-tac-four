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
    board: [Cell; 25],
    current_player: Cell,
    game_mode: GameMode,
    game_active: bool,
}

impl Game {
    fn new(mode: GameMode) -> Self {
        Game {
            board: [Cell::Empty; 25],
            current_player: Cell::X,
            game_mode: mode,
            game_active: true,
        }
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn display_board(&self) {
        // ANSI color codes
        const RESET: &str = "\x1b[0m";
        const CYAN: &str = "\x1b[36m";
        const YELLOW: &str = "\x1b[33m";
        const GREEN: &str = "\x1b[32m";
        const BLUE: &str = "\x1b[34m";
        const MAGENTA: &str = "\x1b[35m";
        const BOLD: &str = "\x1b[1m";
        const RED: &str = "\x1b[31m";

        println!();
        println!("  {}+-------+-------+-------+-------+-------+{}", CYAN, RESET);

        for row in 0..5 {
            print!("  {}|{}", CYAN, RESET);
            for col in 0..5 {
                let index = row * 5 + col;
                let cell_str = match self.board[index] {
                    Cell::Empty => format!("{}  {:2}   {}", YELLOW, index, RESET),
                    Cell::X => format!("{}{}   X   {}{}", BOLD, RED, RESET, RESET),
                    Cell::O => format!("{}{}   O   {}{}", BOLD, BLUE, RESET, RESET),
                };
                print!("{}", cell_str);
                print!("{}|{}", CYAN, RESET);
            }
            println!();

            if row < 4 {
                println!("  {}+-------+-------+-------+-------+-------+{}", CYAN, RESET);
            }
        }

        println!("  {}+-------+-------+-------+-------+-------+{}", CYAN, RESET);
        println!();
    }

    fn display_instructions() {
        const CYAN: &str = "\x1b[36m";
        const YELLOW: &str = "\x1b[33m";
        const GREEN: &str = "\x1b[32m";
        const RESET: &str = "\x1b[0m";
        const BOLD: &str = "\x1b[1m";

        println!("\n{}{}+==================================================+{}", BOLD, CYAN, RESET);
        println!("{}{}|         5x5 TIC-TAC-TOE GAME                     |{}", BOLD, CYAN, RESET);
        println!("{}{}+==================================================+{}", BOLD, CYAN, RESET);
        println!("\n{}{} Game Rules:{}", BOLD, GREEN, RESET);
        println!("   {} * The board is 5x5 (25 cells){}", YELLOW, RESET);
        println!("   {} * Players take turns placing X or O{}", YELLOW, RESET);
        println!("   {} * Get 5 in a row to win (horizontal, vertical, or diagonal){}", YELLOW, RESET);
        println!("   {} * Cells are numbered 0-24{}", YELLOW, RESET);
        println!("\n{}{} How to play:{}", BOLD, GREEN, RESET);
        println!("   {} * Enter the cell number (0-24) when prompted{}", YELLOW, RESET);
        println!("   {} * Type 'quit' to exit the game{}", YELLOW, RESET);
        println!("\n{}+==================================================+{}", CYAN, RESET);
    }

    fn get_game_mode() -> GameMode {
        const CYAN: &str = "\x1b[36m";
        const GREEN: &str = "\x1b[32m";
        const RED: &str = "\x1b[31m";
        const YELLOW: &str = "\x1b[33m";
        const RESET: &str = "\x1b[0m";
        const BOLD: &str = "\x1b[1m";

        println!("\n{}{} >> Select Game Mode:{}", BOLD, CYAN, RESET);
        println!("   {}[1]{} Player vs Player", YELLOW, RESET);
        println!("   {}[2]{} Player vs Computer", YELLOW, RESET);

        loop {
            print!("\n{}Enter your choice (1 or 2):{} ", GREEN, RESET);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "1" => {
                    println!("\n{}{} [OK] Player vs Player mode selected!{}", BOLD, GREEN, RESET);
                    return GameMode::PlayerVsPlayer;
                }
                "2" => {
                    println!("\n{}{} [OK] Player vs Computer mode selected!{}", BOLD, GREEN, RESET);
                    println!("{}      You are X, Computer is O{}", CYAN, RESET);
                    return GameMode::PlayerVsComputer;
                }
                _ => println!("{}{} [ERROR] Invalid choice. Please enter 1 or 2.{}", BOLD, RED, RESET),
            }
        }
    }

    fn is_valid_move(&self, position: usize) -> bool {
        position < 25 && self.board[position] == Cell::Empty
    }

    fn make_move(&mut self, position: usize) {
        self.board[position] = self.current_player;
    }

    fn check_winner(&self) -> Option<Vec<usize>> {
        let player = self.current_player;
        let win_patterns: Vec<Vec<usize>> = vec![
            // Horizontal (5 rows)
            vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9], vec![10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19], vec![20, 21, 22, 23, 24],
            // Vertical (5 columns)
            vec![0, 5, 10, 15, 20], vec![1, 6, 11, 16, 21], vec![2, 7, 12, 17, 22],
            vec![3, 8, 13, 18, 23], vec![4, 9, 14, 19, 24],
            // Diagonals (2)
            vec![0, 6, 12, 18, 24], vec![4, 8, 12, 16, 20],
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
        const RED: &str = "\x1b[31m";
        const BLUE: &str = "\x1b[34m";
        const GREEN: &str = "\x1b[32m";
        const YELLOW: &str = "\x1b[33m";
        const RESET: &str = "\x1b[0m";
        const BOLD: &str = "\x1b[1m";

        loop {
            let player_symbol = self.current_player.to_string();
            let player_color = if self.current_player == Cell::X { RED } else { BLUE };
            print!("{}{}Player {}{}, enter cell number (0-24) or 'quit': {}",
                   BOLD, player_color, player_symbol, RESET, RESET);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input == "quit" {
                println!("\n{}{} Thanks for playing! Goodbye!{}", BOLD, GREEN, RESET);
                process::exit(0);
            }

            match input.parse::<usize>() {
                Ok(pos) if self.is_valid_move(pos) => return Some(pos),
                Ok(_) => println!("{}{} [X] Invalid move! Cell must be between 0-24 and empty.{}", BOLD, RED, RESET),
                Err(_) => println!("{}{} [X] Please enter a valid number or 'quit'.{}", BOLD, YELLOW, RESET),
            }
        }
    }

    fn find_best_move(&self, player: Cell) -> Option<usize> {
        let win_patterns: Vec<Vec<usize>> = vec![
            // Horizontal (5 rows)
            vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9], vec![10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19], vec![20, 21, 22, 23, 24],
            // Vertical (5 columns)
            vec![0, 5, 10, 15, 20], vec![1, 6, 11, 16, 21], vec![2, 7, 12, 17, 22],
            vec![3, 8, 13, 18, 23], vec![4, 9, 14, 19, 24],
            // Diagonals (2)
            vec![0, 6, 12, 18, 24], vec![4, 8, 12, 16, 20],
        ];

        for pattern in win_patterns {
            let values: Vec<Cell> = pattern.iter().map(|&i| self.board[i]).collect();
            let player_count = values.iter().filter(|&&c| c == player).count();
            let empty_count = values.iter().filter(|&&c| c == Cell::Empty).count();

            if player_count == 4 && empty_count == 1 {
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
        const MAGENTA: &str = "\x1b[35m";
        const CYAN: &str = "\x1b[36m";
        const RESET: &str = "\x1b[0m";
        const BOLD: &str = "\x1b[1m";

        print!("\n{}{} >> Computer is thinking", BOLD, MAGENTA, );
        io::stdout().flush().unwrap();

        for _ in 0..3 {
            std::thread::sleep(std::time::Duration::from_millis(300));
            print!(".");
            io::stdout().flush().unwrap();
        }
        print!("{}", RESET);
        println!();

        // Try to win
        if let Some(pos) = self.find_best_move(Cell::O) {
            println!("{}    Computer plays at position {}{}", CYAN, pos, RESET);
            return pos;
        }

        // Block opponent from winning
        if let Some(pos) = self.find_best_move(Cell::X) {
            println!("{}    Computer plays at position {}{}", CYAN, pos, RESET);
            return pos;
        }

        // Take center cells if available
        let center_cells = vec![6, 7, 8, 11, 12, 13, 16, 17, 18];
        let available_center: Vec<usize> = center_cells
            .iter()
            .copied()
            .filter(|&i| self.board[i] == Cell::Empty)
            .collect();

        if !available_center.is_empty() {
            let pos = available_center[rand::thread_rng().gen_range(0..available_center.len())];
            println!("{}    Computer plays at position {}{}", CYAN, pos, RESET);
            return pos;
        }

        // Take any available cell
        let available_cells: Vec<usize> = (0..25)
            .filter(|&i| self.board[i] == Cell::Empty)
            .collect();

        let pos = available_cells[rand::thread_rng().gen_range(0..available_cells.len())];
        println!("{}    Computer plays at position {}{}", CYAN, pos, RESET);
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
                const GREEN: &str = "\x1b[32m";
                const MAGENTA: &str = "\x1b[35m";
                const YELLOW: &str = "\x1b[33m";
                const RED: &str = "\x1b[31m";
                const BLUE: &str = "\x1b[34m";
                const RESET: &str = "\x1b[0m";
                const BOLD: &str = "\x1b[1m";

                self.clear_screen();
                self.display_board();
                println!("\n{}{}+==================================================+{}", BOLD, GREEN, RESET);
                if self.game_mode == GameMode::PlayerVsComputer && self.current_player == Cell::O {
                    println!("{}{}|  *** COMPUTER (O) WINS! ***                      |{}", BOLD, MAGENTA, RESET);
                } else {
                    let player_color = if self.current_player == Cell::X { RED } else { BLUE };
                    println!("{}{}|  *** PLAYER {} WINS! ***                          |{}",
                        BOLD, player_color, self.current_player.to_string(), RESET);
                }
                println!("{}{}+==================================================+{}", BOLD, GREEN, RESET);
                println!("\n{} >> Winning positions: {:?}{}", YELLOW, pattern, RESET);
                self.game_active = false;
                break;
            }

            // Check for draw
            if self.is_board_full() {
                const CYAN: &str = "\x1b[36m";
                const YELLOW: &str = "\x1b[33m";
                const RESET: &str = "\x1b[0m";
                const BOLD: &str = "\x1b[1m";

                self.clear_screen();
                self.display_board();
                println!("\n{}{}+==================================================+{}", BOLD, CYAN, RESET);
                println!("{}{}|  *** IT'S A DRAW! ***                            |{}", BOLD, YELLOW, RESET);
                println!("{}{}+==================================================+{}", BOLD, CYAN, RESET);
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
        const CYAN: &str = "\x1b[36m";
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        const BOLD: &str = "\x1b[1m";

        loop {
            print!("\n{}{} >> Play again? (yes/no):{} ", BOLD, CYAN, RESET);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            match input.as_str() {
                "yes" | "y" => return true,
                "no" | "n" => return false,
                _ => println!("{}{} [X] Please enter 'yes' or 'no'{}", BOLD, RED, RESET),
            }
        }
    }
}

fn main() {
    const CYAN: &str = "\x1b[36m";
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";

    println!("\n{}{}+==================================================+{}", BOLD, CYAN, RESET);
    println!("{}{}|  Welcome to 5x5 Tic-Tac-Toe!                    |{}", BOLD, CYAN, RESET);
    println!("{}{}+==================================================+{}", BOLD, CYAN, RESET);

    loop {
        Game::display_instructions();
        let mode = Game::get_game_mode();

        println!("\n{} >> Press Enter to start the game...{}", GREEN, RESET);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut game = Game::new(mode);
        game.play();

        if !Game::play_again() {
            println!("\n{}{}+==================================================+{}", BOLD, CYAN, RESET);
            println!("{}{}|  Thanks for playing! Goodbye!                    |{}", BOLD, GREEN, RESET);
            println!("{}{}+==================================================+{}\n", BOLD, CYAN, RESET);
            break;
        }
    }
}
