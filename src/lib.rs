pub const SIZE: usize = 9;
pub const SQUARE: usize = SIZE * SIZE;
pub const GRID_SIZE: usize = 3;
pub const EMPTY_MARKER: i32 = 0;
pub const DEFAULT: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

#[rustfmt::skip]
pub const EMPTY: [i32; SQUARE] = [
    0,0,0, 0,0,0, 0,0,0,
    0,0,0, 0,0,0, 0,0,0,
    0,0,0, 0,0,0, 0,0,0,

    0,0,0, 0,0,0, 0,0,0,
    0,0,0, 0,0,0, 0,0,0,
    0,0,0, 0,0,0, 0,0,0,

    0,0,0, 0,0,0, 0,0,0,
    0,0,0, 0,0,0, 0,0,0,
    0,0,0, 0,0,0, 0,0,0
];

#[rustfmt::skip]
pub const BASIC: [i32; SQUARE] = [
    5,3,0, 0,7,0, 0,0,0,
    6,0,0, 1,9,5, 0,0,0,
    0,9,8, 0,0,0, 0,6,0,

    8,0,0, 0,6,0, 0,0,3,
    4,0,0, 8,0,3, 0,0,1,
    7,0,0, 0,2,0, 0,0,6,

    0,6,0, 0,0,0, 2,8,0,
    0,0,0, 4,1,9, 0,0,5,
    0,0,0, 0,8,0, 0,7,9
];

// Solution: https://fi.wikipedia.org/wiki/Sudoku
#[rustfmt::skip]
pub const BASIC_SOLUTION: [i32; SQUARE] = [
    5,3,4, 6,7,8, 9,1,2,
    6,7,2, 1,9,5, 3,4,8,
    1,9,8, 3,4,2, 5,6,7,

    8,5,9, 7,6,1, 4,2,3,
    4,2,6, 8,5,3, 7,9,1,
    7,1,3, 9,2,4, 8,5,6,

    9,6,1, 5,3,7, 2,8,4,
    2,8,7, 4,1,9, 6,3,5,
    3,4,5, 2,8,6, 1,7,9
];

#[rustfmt::skip]
pub const HARD: [i32; SQUARE] = [
    2,0,0, 5,0,7, 4,0,6,
    0,0,0, 0,3,1, 0,0,0,
    0,0,0, 0,0,0, 2,3,0,

    0,0,0, 0,2,0, 0,0,0,
    8,6,0, 3,1,0, 0,0,0,
    0,4,5, 0,0,0, 0,0,0,

    0,0,9, 0,0,0, 7,0,0,
    0,0,6, 9,5,0, 0,0,2,
    0,0,1, 0,0,6, 0,0,8
];

// Solution: https://valeur.org/sudoku/hard/a/free-printable-hard-sudoku-with-the-answer-14739.html
#[rustfmt::skip]
pub const HARD_SOLUTION: [i32; SQUARE] = [
    2,3,8, 5,9,7, 4,1,6,
    6,9,4, 2,3,1, 8,5,7,
    5,1,7, 8,6,4, 2,3,9,

    1,7,3, 4,2,9, 6,8,5,
    8,6,2, 3,1,5, 9,7,4,
    9,4,5, 6,7,8, 3,2,1,

    4,5,9, 1,8,2, 7,6,3,
    7,8,6, 9,5,3, 1,4,2,
    3,2,1, 7,4,6, 5,9,8
];

#[derive(Debug, Clone)]
pub struct Cell {
    pub possibilities: Vec<i32>,
    pub current: i32,
}

impl Cell {
    pub fn make_cell() -> Cell {
        return Cell {
            possibilities: Vec::from(DEFAULT),
            current: 0,
        };
    }
}

pub fn print_board(board: Vec<Cell>) {
    let cur: Vec<i32> = board.iter().map(|x| x.current).collect();
    for (i, val) in cur.iter().enumerate() {
        let y = i / 9;
        print!("{}", val);

        if i == 80 {
            print!("\n");
            break;
        }

        if (i + 1) % 3 == 0 {
            print!(" ");
        }

        if (i + 1) % 9 == 0 {
            print!("\n");

            if (y + 1) % 3 == 0 {
                print!("\n");
            }
        }
    }
    println!("-----------");
}
