use itertools::Itertools;
use sudoku_solver::{GRID_SIZE, SIZE};

pub fn validate_board(preset: &Vec<i32>) -> bool {
    if preset.iter().any(|x| *x > SIZE as i32) {
        return false;
    }

    for i in 0..SIZE {
        let row_start = i * SIZE;
        let row_end = row_start + SIZE;
        let row = &preset[row_start..row_end];

        let mut nums: Vec<&i32> = row.iter().filter(|&x| *x > 0).collect();
        nums.sort();
        let counts: Vec<(usize, &i32)> = nums.into_iter().dedup_with_count().collect();
        // println!("row   : {:?}", row);
        // println!("counts: {:?}", counts);

        if counts.iter().any(|(amount, _)| amount > &1) {
            return false;
        }

        let col: Vec<i32> = preset.iter().enumerate().filter(|(j, _)| j % SIZE == i).map(|(_, &a)| a).collect();
        let mut c_nums: Vec<&i32> = col.iter().filter(|&x| *x > 0).collect();
        c_nums.sort();
        let c_counts: Vec<(usize, &i32)> = c_nums.into_iter().dedup_with_count().collect();
        // println!("col: {:?}", col);
        // println!("counts: {:?}", c_counts);

        if c_counts.iter().any(|(amount, _)| amount > &1) {
            return false;
        }

        let xoffset = (i % GRID_SIZE) * GRID_SIZE;
        let yoffset = (i / GRID_SIZE) * GRID_SIZE;

        let mut group = Vec::new();
        for y in yoffset..(yoffset + GRID_SIZE) {
            for x in xoffset..(xoffset + GRID_SIZE) {
                let ind = SIZE * y + x;
                group.push(preset[ind]);
            }
        }
        let mut g_nums: Vec<&i32> = group.iter().filter(|&x| *x > 0).collect();
        g_nums.sort();
        let g_counts: Vec<(usize, &i32)> = g_nums.into_iter().dedup_with_count().collect();
        // println!("col: {:?}", group);
        // println!("counts: {:?}", g_counts);

        if g_counts.iter().any(|(amount, _)| amount > &1) {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod board_validation_tests {
    use super::validate_board;
    use sudoku_solver::BASIC;

    #[test]
    fn validate_valid() {
        assert_eq!(true, validate_board(&BASIC.to_vec()));
    }

    #[test]
    fn validate_horizontal() {
        #[rustfmt::skip]
      let invalid1 = [
          1,0,0,0,0,0,0,0,1,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0
      ];
        assert_eq!(false, validate_board(&invalid1.to_vec()));
    }

    #[test]
    fn validate_vertical() {
        #[rustfmt::skip]
      let invalid2 = [
          1,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          1,0,0,0,0,0,0,0,0
      ];
        assert_eq!(false, validate_board(&invalid2.to_vec()));
    }
    #[test]
    fn validate_grouping() {
        #[rustfmt::skip]
      let invalid3 = [
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,1,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,1,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0
      ];
        assert_eq!(false, validate_board(&invalid3.to_vec()));
    }

    #[test]
    fn validate_number() {
        #[rustfmt::skip]
      let invalid3 = [
          11,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0
      ];
        assert_eq!(false, validate_board(&invalid3.to_vec()));
    }
}
