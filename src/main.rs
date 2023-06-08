use crate::user_interface::GUI;

pub mod minesweeper;
mod user_interface;

fn main() {
    let board = minesweeper::build_minesweeper_board(16, 30, 99);

    let mut gui: GUI = GUI::new(board, 99);
    gui.run();
}
