pub mod minesweeper;
mod user_interface;

fn main() {
    let board = minesweeper::build_minesweeper_board(16, 30, 99);

    user_interface::run(board, 99)
}
