#!/usr/bin/env python3
"""
4x4 Tic-Tac-Toe Console Game
Players need to get 4 in a row to win
Supports Player vs Player and Player vs Computer modes
"""

import random
import time
import os
import sys


class TicTacToe:
    def __init__(self):
        self.board = [' ' for _ in range(16)]  # 4x4 = 16 cells
        self.current_player = 'X'
        self.game_mode = None
        self.game_active = True

    def clear_screen(self):
        """Clear the console screen"""
        os.system('clear' if os.name == 'posix' else 'cls')

    def display_board(self):
        """Display the 4x4 board with cell numbers"""
        print("\n")
        print("  " + "=" * 41)
        for row in range(4):
            # Display cell numbers for empty cells, or X/O for filled cells
            cells = []
            for col in range(4):
                index = row * 4 + col
                if self.board[index] == ' ':
                    cells.append(f" {index:2d} ")
                else:
                    cells.append(f" {self.board[index]:^2} ")
            print("  |" + "|".join(cells) + "|")
            if row < 3:
                print("  " + "|" + "---|" * 3 + "---|")
        print("  " + "=" * 41)
        print()

    def display_instructions(self):
        """Display game instructions"""
        print("\n" + "=" * 50)
        print("         4x4 TIC-TAC-TOE GAME")
        print("=" * 50)
        print("\nGame Rules:")
        print("  - The board is 4x4 (16 cells)")
        print("  - Players take turns placing X or O")
        print("  - Get 4 in a row (horizontal, vertical, or diagonal) to win")
        print("  - Cells are numbered 0-15")
        print("\nHow to play:")
        print("  - Enter the cell number (0-15) when prompted")
        print("  - Type 'quit' to exit the game")
        print("=" * 50)

    def get_game_mode(self):
        """Get game mode selection from user"""
        print("\nSelect Game Mode:")
        print("  1. Player vs Player")
        print("  2. Player vs Computer")

        while True:
            choice = input("\nEnter your choice (1 or 2): ").strip()
            if choice == '1':
                self.game_mode = 'pvp'
                print("\n✓ Player vs Player mode selected!")
                break
            elif choice == '2':
                self.game_mode = 'pvc'
                print("\n✓ Player vs Computer mode selected!")
                print("  You are X, Computer is O")
                break
            else:
                print("Invalid choice. Please enter 1 or 2.")

    def is_valid_move(self, position):
        """Check if the move is valid"""
        try:
            pos = int(position)
            return 0 <= pos <= 15 and self.board[pos] == ' '
        except (ValueError, TypeError):
            return False

    def make_move(self, position, player):
        """Make a move on the board"""
        self.board[position] = player

    def check_winner(self, player):
        """Check if the given player has won"""
        win_patterns = []

        # Horizontal wins
        for row in range(4):
            win_patterns.append([row * 4, row * 4 + 1, row * 4 + 2, row * 4 + 3])

        # Vertical wins
        for col in range(4):
            win_patterns.append([col, col + 4, col + 8, col + 12])

        # Diagonal wins
        win_patterns.append([0, 5, 10, 15])  # Top-left to bottom-right
        win_patterns.append([3, 6, 9, 12])   # Top-right to bottom-left

        # Check each pattern
        for pattern in win_patterns:
            if all(self.board[i] == player for i in pattern):
                return True, pattern

        return False, None

    def is_board_full(self):
        """Check if the board is full"""
        return ' ' not in self.board

    def get_player_move(self):
        """Get move from human player"""
        while True:
            move = input(f"\nPlayer {self.current_player}, enter cell number (0-15) or 'quit': ").strip().lower()

            if move == 'quit':
                print("\nThanks for playing!")
                sys.exit(0)

            if self.is_valid_move(move):
                return int(move)
            else:
                print("Invalid move! Cell must be between 0-15 and empty.")

    def find_best_move(self, player):
        """Find a winning or blocking move"""
        win_patterns = []

        # Horizontal
        for row in range(4):
            win_patterns.append([row * 4, row * 4 + 1, row * 4 + 2, row * 4 + 3])

        # Vertical
        for col in range(4):
            win_patterns.append([col, col + 4, col + 8, col + 12])

        # Diagonals
        win_patterns.append([0, 5, 10, 15])
        win_patterns.append([3, 6, 9, 12])

        # Check each pattern
        for pattern in win_patterns:
            values = [self.board[i] for i in pattern]
            player_count = values.count(player)
            empty_count = values.count(' ')

            # If player has 3 in this pattern and 1 empty, return the empty cell
            if player_count == 3 and empty_count == 1:
                return pattern[values.index(' ')]

        return -1

    def get_computer_move(self):
        """Get move from computer AI"""
        print("\nComputer is thinking", end="", flush=True)
        for _ in range(3):
            time.sleep(0.3)
            print(".", end="", flush=True)
        print()

        # Try to win
        winning_move = self.find_best_move('O')
        if winning_move != -1:
            print(f"Computer plays at position {winning_move}")
            return winning_move

        # Block opponent from winning
        blocking_move = self.find_best_move('X')
        if blocking_move != -1:
            print(f"Computer plays at position {blocking_move}")
            return blocking_move

        # Take center cells if available
        center_cells = [5, 6, 9, 10]
        available_center = [i for i in center_cells if self.board[i] == ' ']
        if available_center:
            move = random.choice(available_center)
            print(f"Computer plays at position {move}")
            return move

        # Take any available cell
        available_cells = [i for i in range(16) if self.board[i] == ' ']
        if available_cells:
            move = random.choice(available_cells)
            print(f"Computer plays at position {move}")
            return move

        return -1

    def switch_player(self):
        """Switch to the other player"""
        self.current_player = 'O' if self.current_player == 'X' else 'X'

    def play_game(self):
        """Main game loop"""
        self.clear_screen()
        self.display_instructions()
        self.get_game_mode()

        input("\nPress Enter to start the game...")

        while self.game_active:
            self.clear_screen()
            self.display_board()

            # Get move based on game mode and current player
            if self.game_mode == 'pvc' and self.current_player == 'O':
                move = self.get_computer_move()
            else:
                move = self.get_player_move()

            # Make the move
            self.make_move(move, self.current_player)

            # Check for winner
            winner, pattern = self.check_winner(self.current_player)
            if winner:
                self.clear_screen()
                self.display_board()
                print("=" * 50)
                if self.game_mode == 'pvc' and self.current_player == 'O':
                    print(f"  🤖 Computer (O) wins!")
                else:
                    print(f"  🎉 Player {self.current_player} wins!")
                print("=" * 50)
                print(f"\nWinning positions: {pattern}")
                self.game_active = False
                break

            # Check for draw
            if self.is_board_full():
                self.clear_screen()
                self.display_board()
                print("=" * 50)
                print("  🤝 It's a draw!")
                print("=" * 50)
                self.game_active = False
                break

            # Switch player
            self.switch_player()

            # Small delay for computer move
            if self.game_mode == 'pvc' and self.current_player == 'X':
                time.sleep(1)

    def play_again(self):
        """Ask if player wants to play again"""
        while True:
            choice = input("\nPlay again? (yes/no): ").strip().lower()
            if choice in ['yes', 'y']:
                return True
            elif choice in ['no', 'n']:
                return False
            else:
                print("Please enter 'yes' or 'no'")


def main():
    """Main function to run the game"""
    print("\n" + "=" * 50)
    print("  Welcome to 4x4 Tic-Tac-Toe!")
    print("=" * 50)

    while True:
        game = TicTacToe()
        game.play_game()

        if not game.play_again():
            print("\n" + "=" * 50)
            print("  Thanks for playing! Goodbye!")
            print("=" * 50 + "\n")
            break


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n\nGame interrupted. Thanks for playing!")
        sys.exit(0)
