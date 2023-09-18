// 设计一个四字棋游戏
// 说明:
// 四字棋游戏是一个二维游戏，在一个二维数组里面进行的游戏
// 游戏有两玩家，分别为YELLOW，RED，
// 游戏初始化是，每个格子都为EMPTY，当有玩家下棋的时候，
// 被选中的格子就变为玩家的颜色
// 取胜规则，连着四个棋子为同一个颜色的即为得一分，可以是横向，纵向，斜向

use std::cmp::max;
use std::collections::HashMap;
use std::io::stdin;

use anyhow::anyhow;

// =============================================================================
// 棋盘

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GirdPosition {
    EMPTY = 0,
    YELLOW = 1,
    RED = 2,
}

#[derive(Debug, Clone)]
pub struct Grid {
    rows: usize,
    columns: usize,
    grid: Vec<Vec<GirdPosition>>,
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut grid = Vec::new();
        for _ in 0..rows {
            let mut column = Vec::new();
            for _ in 0..columns {
                column.push(GirdPosition::EMPTY)
            }
            grid.push(column);
        }
        Self {
            rows,
            columns,
            grid,
        }
    }

    pub fn get_gird(&self) -> Vec<Vec<GirdPosition>> {
        self.grid.clone()
    }

    pub fn get_columns_count(&self) -> usize {
        self.columns
    }

    pub fn place_piece(&mut self, column: usize, piece: &GirdPosition) -> anyhow::Result<usize> {
        if column >= self.columns {
            return Err(anyhow!("Invalid column"));
        }
        if piece == &GirdPosition::EMPTY {
            return Err(anyhow!("Invalid piece"));
        }

        // (0..self.rows).rev() 等价于 [0, self.rows-1).rev() 等价于 [self.rows-1, 0]
        for row in (0..self.rows).rev() {
            if self.grid[row][column] == GirdPosition::EMPTY {
                self.grid[row][column] = piece.clone();
                return Ok(row);
            }
        }

        Ok(0)
    }

    pub fn check_win(
        &self,
        connect_n: usize,
        row: usize,
        column: usize,
        piece: &GirdPosition,
    ) -> bool {
        let mut count = 0;
        // Check horizontal
        for c in 0..self.columns {
            if self.grid[row][c] == *piece {
                count += 1;
            } else {
                count = 0;
            }
            if count == connect_n {
                return true;
            }
        }

        // Check vertical
        count = 0;
        for r in 0..self.rows {
            if self.grid[r][column] == *piece {
                count += 1;
            } else {
                count = 0;
            }
            if count == connect_n {
                return true;
            }
        }

        // Check diagonal
        count = 0;
        for r in 0..self.rows {
            let c = ((row as i32 + column as i32) - r as i32) as usize;
            if c < self.columns && self.grid[r][c] == *piece {
                count += 1;
            } else {
                count = 0;
            }
            if count == connect_n {
                return true;
            }
        }

        // Check anti-diagonal
        count = 0;
        for r in 0..self.rows {
            let c = ((row as i32 - column as i32) + r as i32) as usize;
            if c < self.columns && self.grid[r][c] == *piece {
                count += 1;
            } else {
                count = 0;
            }
            if count == connect_n {
                return true;
            }
        }

        false
    }
}

// =============================================================================
// 玩家

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub piece_color: GirdPosition,
}

impl Player {
    pub fn new(name: String, piece_color: GirdPosition) -> Self {
        Self { name, piece_color }
    }
}

// =============================================================================
// 游戏
pub struct Game {
    gird: Grid,
    connect_n: usize,
    players: Vec<Player>,

    score: HashMap<String, usize>,
    target_score: usize,
}

impl Game {
    pub fn new(gird: Grid, connect_n: usize, target_score: usize) -> Self {
        let mut game = Self {
            gird,
            connect_n,
            players: vec![
                Player::new("Player 1".to_string(), GirdPosition::RED),
                Player::new("Player 2".to_string(), GirdPosition::YELLOW),
            ],
            score: HashMap::new(),
            target_score,
        };
        game.init_grid();
        game
    }

    pub fn play(&mut self) -> anyhow::Result<()> {
        let mut max_score = 0;
        let mut winner: Option<Player> = None;
        while max_score < self.target_score {
            let player = self.play_round()?;
            println!("{} won the round", player.name.clone());
            let score = *self.score.get(&player.name).unwrap_or(&0);
            max_score = max(max_score, score);
            winner = Some(player);
            self.init_grid();
        }

        println!("{} won the game", winner.unwrap().name);
        Ok(())
    }

    fn init_grid(&mut self) {
        self.score.clear();
        for player in &self.players {
            self.score.insert(player.name.clone(), 0);
        }
    }

    fn play_move(&mut self, player: &Player) -> anyhow::Result<(usize, usize)> {
        self.print_board();
        println!("{}'s turn", player.name);
        let column_count = self.gird.get_columns_count();
        println!(
            "Enter column between 0 and {} to add piece: ",
            column_count - 1
        );
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let move_column = input.trim().parse::<usize>()?;
        let move_row = self.gird.place_piece(move_column, &player.piece_color)?;
        Ok((move_row, move_column))
    }

    fn play_round(&mut self) -> anyhow::Result<Player> {
        let players = self.players.clone();
        loop {
            for player in &players {
                let player_clone = player.clone();
                let (row, column) = self.play_move(player)?;
                if self
                    .gird
                    .check_win(self.connect_n, row, column, &player_clone.piece_color)
                {
                    if let Some(score) = self.score.get_mut(&player_clone.name) {
                        *score += 1;
                    }
                    return Ok(player_clone);
                }
            }
        }
    }

    fn print_board(&self) {
        println!("Board:");
        let gird = self.gird.get_gird();
        for i in 0..gird.len() {
            let mut row = String::from("");
            for piece in &gird[i] {
                match piece {
                    GirdPosition::EMPTY => row += "0 ",
                    GirdPosition::RED => row += "R ",
                    GirdPosition::YELLOW => row += "Y ",
                }
            }
            println!("{row}");
        }
        println!("");
    }
}

fn main() {
    let grid = Grid::new(6, 7);
    let mut game = Game::new(grid, 4, 2);
    game.play().unwrap();
}
