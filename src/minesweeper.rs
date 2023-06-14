extern crate rand;

use rand::Rng;

#[derive(Copy, Clone)]
pub struct Tile {
    mine: bool,
    num: i32,
    flagged: bool,
    revealed: bool,
    pos: [i32; 2],
}

pub type MinesweeperBoard = Vec<Vec<Tile>>;

impl Tile {
    // no args constructor
    pub fn empty() -> Self {
        Tile {
            mine: false,
            num: 0,
            flagged: false,
            revealed: false,
            pos: [0, 0],
        }
    }

    // full args constructor
    pub fn new(mine: bool, num: i32, flagged: bool, revealed: bool, pos: [i32; 2]) -> Self {
        Tile {
            mine,
            num,
            flagged,
            revealed,
            pos,
        }
    }

    // Setters
    pub fn set_mine(&mut self, mine: bool) {
        self.mine = mine;
    }

    pub fn set_num(&mut self, num: i32) {
        self.num = num;
    }

    pub fn set_flagged(&mut self, flagged: bool) {
        self.flagged = flagged;
    }

    pub fn reveal(&mut self) {
        if !self.revealed {
            self.revealed = true;
        }
    }

    pub fn set_pos(&mut self, pos: [i32; 2]) {
        self.pos = pos;
    }

    // Getters
    pub fn has_mine(&self) -> bool {
        self.mine
    }

    pub fn get_num(&self) -> i32 {
        self.num
    }

    pub fn is_flagged(&self) -> bool {
        self.flagged
    }

    pub fn is_revealed(&self) -> bool {
        self.revealed
    }

    pub fn get_pos(&self) -> [i32; 2] {
        self.pos
    }
}

pub fn build_minesweeper_board(height: i32, width: i32, mut mine_count: i32) -> MinesweeperBoard {
    let mut board: MinesweeperBoard = vec![vec![Tile::empty(); width as usize]; height as usize];
    let mut clean_tiles = height * width;

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            board[i][j].set_pos([i as i32, j as i32])
        }
    }

    while (mine_count > 0) && (clean_tiles > 0) {
        let mut x = rand::thread_rng().gen_range(0..width);
        let mut y = rand::thread_rng().gen_range(0..height);

        while board[y as usize][x as usize].has_mine() {
            x = rand::thread_rng().gen_range(0..width);
            y = rand::thread_rng().gen_range(0..height);
        }

        board[y as usize][x as usize].set_mine(true);
        mine_count -= 1;
        clean_tiles -= 1;
    }

    determine_tile_number(&mut board);

    board
}

pub fn determine_tile_number(board: &mut MinesweeperBoard) {
    let height: i32 = board.len() as i32;

    for j in 0..height {
        let width: i32 = board[j as usize].len() as i32;
        for i in 0..width {
            let mut number = 0;

            if !board[j as usize][i as usize].has_mine() {
                if j + 1 < height {
                    if i + 1 < width && board[(j + 1) as usize][(i + 1) as usize].has_mine() {
                        number += 1;
                    }
                    if i - 1 >= 0 && board[(j + 1) as usize][(i - 1) as usize].has_mine() {
                        number += 1;
                    }
                    if board[(j + 1) as usize][i as usize].has_mine() {
                        number += 1;
                    }
                }
                if j - 1 >= 0 {
                    if i + 1 < width && board[(j - 1) as usize][(i + 1) as usize].has_mine() {
                        number += 1;
                    }
                    if i - 1 >= 0 && board[(j - 1) as usize][(i - 1) as usize].has_mine() {
                        number += 1;
                    }
                    if board[(j - 1) as usize][i as usize].has_mine() {
                        number += 1;
                    }
                }
                if i + 1 < width && board[j as usize][(i + 1) as usize].has_mine() {
                    number += 1;
                }
                if i - 1 >= 0 && board[j as usize][(i - 1) as usize].has_mine() {
                    number += 1;
                }
            }

            board[j as usize][i as usize].set_num(number)
        }
    }
}

pub fn cord_tile(board: &mut MinesweeperBoard, x: i32, y: i32) -> Result<bool, [i32; 2]> {
    if y < 0 || y >= board.len() as i32 || x < 0 || x >= board[y as usize].len() as i32 {
        return Err([-1, -1]);
    }

    if board[y as usize][x as usize].get_num() == 0 {
        return Err([-1, -1]);
    }

    let mut num_flagged = 0;
    if y - 1 >= 0 {
        if x - 1 >= 0 {
            if board[(y - 1) as usize][(x - 1) as usize].is_flagged() {
                num_flagged += 1;
            }
        }
        if x + 1 < board[(y - 1) as usize].len() as i32 {
            if board[(y - 1) as usize][(x + 1) as usize].is_flagged() {
                num_flagged += 1;
            }
        }
        if board[(y - 1) as usize][x as usize].is_flagged() {
            num_flagged += 1;
        }
    }
    if y + 1 < board.len() as i32 {
        if x - 1 >= 0 {
            if board[(y + 1) as usize][(x - 1) as usize].is_flagged() {
                num_flagged += 1;
            }
        }
        if x + 1 < board[(y + 1) as usize].len() as i32 {
            if board[(y + 1) as usize][(x + 1) as usize].is_flagged() {
                num_flagged += 1;
            }
        }
        if board[(y + 1) as usize][x as usize].is_flagged() {
            num_flagged += 1;
        }
    }
    if x - 1 >= 0 {
        if board[y as usize][(x - 1) as usize].is_flagged() {
            num_flagged += 1;
        }
    }
    if x + 1 < board[y as usize].len() as i32 {
        if board[y as usize][(x + 1) as usize].is_flagged() {
            num_flagged += 1;
        }
    }

    let mut result: i32;
    if num_flagged == board[y as usize][x as usize].get_num() {
        if y - 1 >= 0 {
            if x - 1 >= 0 {
                result = reveal_tile(board, x - 1, y - 1, false);
                if result == 1 {
                    return Err([x - 1, y - 1]);
                }
            }
            if x + 1 < board[(y - 1) as usize].len() as i32 {
                result = reveal_tile(board, x + 1, y - 1, false);
                if result == 1 {
                    return Err([x + 1, y - 1]);
                }
            }
            result = reveal_tile(board, x, y - 1, false);
            if result == 1 {
                return Err([x, y - 1]);
            }
        }
        if y + 1 < board.len() as i32 {
            if x - 1 >= 0 {
                result = reveal_tile(board, x - 1, y + 1, false);
                if result == 1 {
                    return Err([x - 1, y + 1]);
                }
            }
            if x + 1 < board[(y + 1) as usize].len() as i32 {
                result = reveal_tile(board, x + 1, y + 1, false);
                if result == 1 {
                    return Err([x + 1, y + 1]);
                }
            }
            result = reveal_tile(board, x, y + 1, false);
            if result == 1 {
                return Err([x, y + 1]);
            }
        }
        if x - 1 >= 0 {
            result = reveal_tile(board, x - 1, y, false);
            if result == 1 {
                return Err([x - 1, y]);
            }
        }
        if x + 1 < board[y as usize].len() as i32 {
            result = reveal_tile(board, x + 1, y, false);
            if result == 1 {
                return Err([x + 1, y]);
            }
        }
    } else {
        return Ok(false);
    }

    return Ok(true);
}

pub fn reveal_tile(board: &mut MinesweeperBoard, x: i32, y: i32, first_click: bool) -> i32 {
    if y < 0 || y >= board.len() as i32
        || x < 0 || x >= board[y as usize].len() as i32
        || board[y as usize][x as usize].is_revealed() || board[y as usize][x as usize].is_flagged()
    {
        return 2;
    }

    if board[y as usize][x as usize].has_mine() && !(first_click) {
        return 1;
    } else if board[y as usize][x as usize].has_mine() && first_click {
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if !(board[i][j].has_mine()) && ([j as i32, i as i32] != [x, y]) {
                    board[i][j] = board[y as usize][x as usize];
                    board[y as usize][x as usize].set_mine(false);
                    determine_tile_number(board);
                    break
                }
            }
        }
    }

    board[y as usize][x as usize].reveal();

    if board[y as usize][x as usize].get_num() == 0 {
        // Top Left
        reveal_tile(board, x - 1, y - 1, first_click);
        // Top
        reveal_tile(board, x, y - 1, first_click);
        // Top Right
        reveal_tile(board, x + 1, y - 1, first_click);

        // Left
        reveal_tile(board, x - 1, y, first_click);
        // Right
        reveal_tile(board, x + 1, y, first_click);

        // Bottom Left
        reveal_tile(board, x - 1, y + 1, first_click);
        // Bottom
        reveal_tile(board, x, y + 1, first_click);
        // Bottom Right
        reveal_tile(board, x + 1, y + 1, first_click);
    }

    return 0;
}

pub fn flag_tile(board: &mut MinesweeperBoard, x: i32, y: i32, mut mine_count: i32) -> i32 {
    let mut selected_tile = board[y as usize][x as usize];

    if !(selected_tile.is_revealed()) {
        if selected_tile.is_flagged() {
            selected_tile.set_flagged(false);
            mine_count += 1
        } else {
            selected_tile.set_flagged(true);
            mine_count -= 1;
        }
    }

    board[y as usize][x as usize] = selected_tile;
    return mine_count;
}
