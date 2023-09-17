
const GridPosition = {
    EMPTY: 0,
    YELLOW: 1,
    RED: 2,
}

class Grid {
    #rows;
    #columns;
    #grid;

    constructor(rows, columns) {
        this.#rows = rows;
        this.#columns = columns;
        this.initGrid();
    }

    initGrid() {
        this.#grid =
            Array(this.#rows).fill(0).map(
                () => Array(this.#columns).fill(GridPosition.EMPTY));
    }

    getGrid() {
        return this.#grid;
    }

    getColumnCount() {
        return this.#columns;
    }

    placePiece(column, piece) {
        if (column < 0 || column >= this.#columns) {
            throw new Error('Invalid column');
        }
        if (piece === GridPosition.EMPTY) {
            throw new Error('Invalid piece');
        }
        // Place piece in the lowest empty row
        for (let row = this.#rows - 1; row >= 0; row--) {
            if (this.#grid[row][column] === GridPosition.EMPTY) {
                this.#grid[row][column] = piece;
                return row;
            }
        }
    }

    checkWin(connectN, row, col, piece) {
        // Check horizontal
        let count = 0;
        for (let c = 0; c < this.#columns; c++) {
            if (this.#grid[row][c] === piece) {
                count++;
            } else {
                count = 0;
            }
            if (count === connectN) {
                return true;
            }
        }

        // Check vertical
        count = 0;
        for (let r = 0; r < this.#rows; r++) {
            if (this.#grid[r][col] === piece) {
                count++;
            } else {
                count = 0;
            }
            if (count === connectN) {
                return true;
            }
        }

        // Check diagonal
        count = 0;
        for (let r = 0; r < this.#rows; r++) {
            let c = row + col - r; // row + col = r + c, for a diagonal
            if (c >= 0 && c < this.#columns && this.#grid[r][c] === piece) {
                count++;
            } else {
                count = 0;
            }
            if (count === connectN) {
                return true;
            }
        }

        // Check anti-diagonal
        count = 0;
        for (let r = 0; r < this.#rows; r++) {
            let c = col - row + r; // row - col = r - c, for an anti-diagonal
            if (c >= 0 && c < this.#columns && this.#grid[r][c] === piece) {
                count++;
            } else {
                count = 0;
            }
            if (count === connectN) {
                return true;
            }
        }
        return false;
    }
}

class Player {
    #name
    #pieceColor;

    constructor(name, pieceColor) {
        this.#name = name;
        this.#pieceColor = pieceColor;
    }

    getName() {
        return this.#name;
    }

    getPieceColor() {
        return this.#pieceColor;
    }
}


const prompt = require('prompt-sync')({ sigint: true });

class Game {
    #grid;
    #connectN;
    #players;
    #score;
    #targetScore;

    constructor(grid, connectN, targetScore) {
        this.#grid = grid;
        this.#connectN = connectN;
        this.#targetScore = targetScore;

        this.#players = [
            new Player('Player 1', GridPosition.YELLOW),
            new Player('Player 2', GridPosition.RED)
        ];

        this.#score = {};
        for (const player of this.#players) {
            this.#score[player.getName()] = 0;
        }
    }

    #printBoard() {
        console.log('Board:\n');
        const grid = this.#grid.getGrid();
        for (let i = 0; i < grid.length; i++) {
            let row = '';
            for (let piece of grid[i]) {
                if (piece === GridPosition.EMPTY) {
                    row += '0 ';
                } else if (piece === GridPosition.YELLOW) {
                    row += 'Y ';
                } else if (piece === GridPosition.RED) {
                    row += 'R ';
                }
            }
            console.log(row);
        }
        console.log('');
    }

    #playMove(player) {
        this.#printBoard();
        console.log(`${player.getName()}'s turn`);
        const colCnt = this.#grid.getColumnCount();
        const moveColumn =
            Number(prompt(`Enter column between ${0} and ${colCnt - 1} to add piece: `));
        let moveRow = this.#grid.placePiece(moveColumn, player.getPieceColor());
        return [moveRow, moveColumn];
    }

    #playRound() {
        while (true) {
            for (const player of this.#players) {
                let [row, col] = this.#playMove(player);
                const pieceColor = player.getPieceColor();
                if (this.#grid.checkWin(this.#connectN, row, col, pieceColor)) {
                    this.#score[player.getName()]++;
                    return player;
                }
            }
        }
    }

    play() {
        let maxScore = 0;
        let winner = null;
        while (maxScore < this.#targetScore) {
            winner = this.#playRound();
            console.log(`${winner.getName()} won the round`);
            maxScore = Math.max(this.#score[winner.getName()], maxScore);

            this.#grid.initGrid(); // reset grid
        }
        console.log(`${winner.getName()} won the game`);
    }
}

const grid = new Grid(6, 7);
const game = new Game(grid, /* connectN= */ 4, /* targetScore= */ 2);
game.play();