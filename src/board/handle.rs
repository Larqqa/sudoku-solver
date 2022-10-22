use crate::board::validate::validate_board;
use rand::seq::SliceRandom;
use sudoku_solver::Cell;
use sudoku_solver::SIZE;

pub fn update(board: Vec<Cell>) -> Vec<Cell> {
    let mut updated_board = board.clone();

    for (i, cell) in board.iter().enumerate() {
        if cell.possibilities.len() == 0 {
            continue;
        }

        let x = i % SIZE;
        let y = i / SIZE;

        let mut horizontal = Vec::new();
        let mut vertical = Vec::new();
        let mut group = Vec::new();

        for xz in 0..SIZE {
            let ind = SIZE * y + xz;
            if i != ind && board[ind].current != 0 {
                horizontal.push(board[ind].current);
            }
        }

        for yz in 0..SIZE {
            let ind = SIZE * yz + x;
            if i != ind && board[ind].current != 0 {
                vertical.push(board[ind].current);
            }
        }

        #[rustfmt::skip]
        let oy = if y < 3 {0} else if y < 6  {3} else {6};
        #[rustfmt::skip]
        let ox = if x < 3 {0} else if x < 6  {3} else {6};

        for yz in 0..3 {
            for xz in 0..3 {
                let ind = SIZE * (yz + oy) + (xz + ox);
                if i != ind && board[ind].current != 0 {
                    group.push(board[ind].current);
                }
            }
        }

        let mut filled = [horizontal, vertical, group].concat();
        filled.sort_unstable();
        filled.dedup();

        for f in filled {
            let index = updated_board[i].possibilities.iter().position(|x| x == &f);

            if index.is_some() {
                updated_board[i].possibilities.remove(index.unwrap());
            }
        }
    }

    return updated_board;
}

pub fn collapse(board: Vec<Cell>) -> Vec<Cell> {
    let mut collapsed_board = board.clone();

    for (i, el) in board.iter().enumerate() {
        if el.possibilities.len() == 1 {
            let collapsed = el.possibilities[0];
            collapsed_board[i].current = collapsed;
            collapsed_board[i].possibilities.remove(0);
        }
    }

    return collapsed_board;
}

pub fn has_collapsible(board: &Vec<Cell>) -> bool {
    return board.iter().any(|x| x.possibilities.len() == 1);
}

pub fn find_indexes_of_lowest_entropy(board: &Vec<Cell>) -> Vec<usize> {
    let mut indexes = Vec::new();
    for (i, el) in board.iter().enumerate() {
        let length = el.possibilities.len();
        if length == 0 {
            continue;
        }

        if indexes.len() == 0 {
            indexes.push((i, length));
        } else if indexes.iter().any(|(_, y)| y > &length) {
            indexes = Vec::from([(i, length)]);
        } else if indexes.iter().any(|(_, y)| y == &length) {
            indexes.push((i, length));
        }
    }

    return indexes.iter().map(|(x, _)| *x).collect();
}

pub fn all_collapsed(board: &Vec<Cell>) -> bool {
    return board.iter().all(|x| x.possibilities.len() == 0);
}

pub fn not_solvable(board: &Vec<Cell>) -> bool {
    if board
        .iter()
        .any(|x| x.possibilities.len() == 0 && x.current == 0)
    {
        return true;
    }

    let b = board.iter().map(|x| x.current).collect();
    return !validate_board(&b);
}

pub fn solve_board(mut board: Vec<Cell>) -> (Vec<Cell>, bool) {
    if not_solvable(&board) {
        return (board, false);
    } else if all_collapsed(&board) {
        return (board, true);
    }

    if has_collapsible(&board) {
        board = collapse(board);
        board = update(board);
        return solve_board(board);
    } else {
        let indexes = find_indexes_of_lowest_entropy(&board);
        let random_index = *indexes.choose(&mut rand::thread_rng()).unwrap();
        let mut copy_of_board = board.clone();

        let mut counter = copy_of_board[random_index].possibilities.len() - 1;
        while copy_of_board[random_index].possibilities.len() != 0 {
            let mut new_board = board.clone();

            let current = copy_of_board[random_index].possibilities[counter];
            new_board[random_index].possibilities.remove(counter);
            new_board[random_index].current = current;

            let (b, s) = solve_board(new_board);
            if s {
                return (b, s);
            } else {
                copy_of_board[random_index].possibilities.remove(counter);
            }

            if counter > 0 {
                counter = counter - 1;
            }
        }
    }

    (board, false)
}
