mod board;
use board::handle::solve_board;
use sudoku_solver::{HARD, print_board};

fn main() {
    // let mut b = board::make_board(Option::None);
    let mut b = board::make_board(Some(HARD.to_vec()));

    print_board(b.clone());
    let (z, x) = solve_board(b);
    println!("is solvable: {}", x);
    b = z;
    print_board(b.clone());
}
