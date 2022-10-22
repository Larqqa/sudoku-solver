use generate::gen_board;
use sudoku_solver::{Cell, EMPTY};
use validate::validate_board;

pub mod generate;
pub mod handle;
pub mod validate;

pub fn make_board(preset: Option<Vec<i32>>) -> Vec<Cell> {
    let board_preset = preset.unwrap_or(EMPTY.to_vec());
    if !validate_board(&board_preset) {
        panic!("preset invalid");
    }
    let board = gen_board(board_preset);
    return board;
}
