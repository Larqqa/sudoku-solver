use crate::board::handle::update;
use sudoku_solver::{Cell, SQUARE};

pub fn gen_board(preset: Vec<i32>) -> Vec<Cell> {
    let total_size = SQUARE;
    let mut board: Vec<Cell> = vec![Cell::make_cell(); total_size];

    for (i, el) in preset.iter().enumerate() {
        if el == &0 {
            continue;
        }

        board[i].possibilities.clear();
        board[i].current = *el;
    }

    board = update(board);
    return board;
}

#[cfg(test)]
mod board_tests {
    use super::gen_board;
    use sudoku_solver::BASIC;

    #[test]
    fn validate() {
        let board = gen_board(BASIC.to_vec());
        let current: Vec<i32> = board.iter().map(|x| x.current).collect();
        assert_eq!(current, BASIC);
    }

    #[test]
    fn possibilities_match() {
        let board = gen_board(BASIC.to_vec());
        assert_eq!(board[3].possibilities, vec![2, 6]);
        assert_eq!(board[21].possibilities, vec![2, 3]);
        assert_eq!(board[56].possibilities, vec![1, 3, 4, 5, 7, 9]);
        assert_eq!(board[72].possibilities, vec![1, 2, 3]);
        assert_eq!(board[0].possibilities, vec![]);
    }
}
