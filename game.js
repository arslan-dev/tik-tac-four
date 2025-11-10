// Game state
let board = Array(16).fill(null); // 4x4 grid = 16 cells
let currentPlayer = 'X';
let gameActive = true;
let gameMode = 'pvp'; // 'pvp' or 'pvc'

// DOM elements
const gameBoard = document.getElementById('game-board');
const currentPlayerDisplay = document.getElementById('current-player');
const gameStatusDisplay = document.getElementById('game-status');
const resetBtn = document.getElementById('reset-btn');
const pvpBtn = document.getElementById('pvp-btn');
const pvcBtn = document.getElementById('pvc-btn');

// Initialize the game board
function initBoard() {
    gameBoard.innerHTML = '';
    for (let i = 0; i < 16; i++) {
        const cell = document.createElement('div');
        cell.classList.add('cell');
        cell.dataset.index = i;
        cell.addEventListener('click', handleCellClick);
        gameBoard.appendChild(cell);
    }
}

// Handle cell click
function handleCellClick(e) {
    const index = parseInt(e.target.dataset.index);

    if (!gameActive || board[index] !== null) {
        return;
    }

    makeMove(index, currentPlayer);

    if (gameActive && gameMode === 'pvc' && currentPlayer === 'O') {
        // Computer's turn
        setTimeout(computerMove, 500);
    }
}

// Make a move
function makeMove(index, player) {
    board[index] = player;
    const cell = document.querySelector(`[data-index="${index}"]`);
    cell.textContent = player;
    cell.classList.add('taken', player.toLowerCase());

    if (checkWinner(player)) {
        gameActive = false;
        gameStatusDisplay.textContent = `Player ${player} wins!`;
        highlightWinningCells();
        return;
    }

    if (board.every(cell => cell !== null)) {
        gameActive = false;
        gameStatusDisplay.textContent = "It's a draw!";
        return;
    }

    currentPlayer = currentPlayer === 'X' ? 'O' : 'X';
    updateDisplay();
}

// Check for winner
function checkWinner(player) {
    const winPatterns = [];

    // Horizontal wins
    for (let row = 0; row < 4; row++) {
        winPatterns.push([row * 4, row * 4 + 1, row * 4 + 2, row * 4 + 3]);
    }

    // Vertical wins
    for (let col = 0; col < 4; col++) {
        winPatterns.push([col, col + 4, col + 8, col + 12]);
    }

    // Diagonal wins (top-left to bottom-right)
    winPatterns.push([0, 5, 10, 15]);

    // Diagonal wins (top-right to bottom-left)
    winPatterns.push([3, 6, 9, 12]);

    // Check each pattern
    for (let pattern of winPatterns) {
        if (pattern.every(index => board[index] === player)) {
            window.winningPattern = pattern; // Store for highlighting
            return true;
        }
    }

    return false;
}

// Highlight winning cells
function highlightWinningCells() {
    if (window.winningPattern) {
        window.winningPattern.forEach(index => {
            const cell = document.querySelector(`[data-index="${index}"]`);
            cell.classList.add('winner');
        });
    }
}

// Computer move (AI)
function computerMove() {
    if (!gameActive) return;

    // Try to win
    const winningMove = findBestMove('O');
    if (winningMove !== -1) {
        makeMove(winningMove, 'O');
        return;
    }

    // Block opponent from winning
    const blockingMove = findBestMove('X');
    if (blockingMove !== -1) {
        makeMove(blockingMove, 'O');
        return;
    }

    // Take center cells if available
    const centerCells = [5, 6, 9, 10];
    const availableCenter = centerCells.filter(i => board[i] === null);
    if (availableCenter.length > 0) {
        const move = availableCenter[Math.floor(Math.random() * availableCenter.length)];
        makeMove(move, 'O');
        return;
    }

    // Take any available cell
    const availableCells = board.map((cell, index) => cell === null ? index : null).filter(val => val !== null);
    if (availableCells.length > 0) {
        const move = availableCells[Math.floor(Math.random() * availableCells.length)];
        makeMove(move, 'O');
    }
}

// Find the best move for a player (either to win or block)
function findBestMove(player) {
    const winPatterns = [];

    // Horizontal
    for (let row = 0; row < 4; row++) {
        winPatterns.push([row * 4, row * 4 + 1, row * 4 + 2, row * 4 + 3]);
    }

    // Vertical
    for (let col = 0; col < 4; col++) {
        winPatterns.push([col, col + 4, col + 8, col + 12]);
    }

    // Diagonals
    winPatterns.push([0, 5, 10, 15]);
    winPatterns.push([3, 6, 9, 12]);

    // Check each pattern
    for (let pattern of winPatterns) {
        const values = pattern.map(i => board[i]);
        const playerCount = values.filter(v => v === player).length;
        const emptyCount = values.filter(v => v === null).length;

        // If player has 3 in this pattern and 1 empty, return the empty cell
        if (playerCount === 3 && emptyCount === 1) {
            return pattern[values.indexOf(null)];
        }
    }

    return -1;
}

// Update display
function updateDisplay() {
    if (gameActive) {
        if (gameMode === 'pvc' && currentPlayer === 'O') {
            currentPlayerDisplay.textContent = "Computer's turn...";
        } else {
            currentPlayerDisplay.textContent = `Current Player: ${currentPlayer}`;
        }
    }
}

// Reset game
function resetGame() {
    board = Array(16).fill(null);
    currentPlayer = 'X';
    gameActive = true;
    window.winningPattern = null;
    gameStatusDisplay.textContent = '';
    initBoard();
    updateDisplay();
}

// Set game mode
function setGameMode(mode) {
    gameMode = mode;
    resetGame();

    if (mode === 'pvp') {
        pvpBtn.classList.add('active');
        pvcBtn.classList.remove('active');
    } else {
        pvcBtn.classList.add('active');
        pvpBtn.classList.remove('active');
    }
}

// Event listeners
resetBtn.addEventListener('click', resetGame);
pvpBtn.addEventListener('click', () => setGameMode('pvp'));
pvcBtn.addEventListener('click', () => setGameMode('pvc'));

// Initialize the game
initBoard();
updateDisplay();
