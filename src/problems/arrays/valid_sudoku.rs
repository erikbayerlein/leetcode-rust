use std::collections::HashSet;


pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut boxes: Vec<Vec<HashSet<char>>> = vec![vec![HashSet::new(); 3]; 3];

        for i in 0..9 {
            for j in 0..9 {
                let num = board[i][j];
                if num != '.' {
                    if rows[i].contains(&num) {
                        return false;
                    }
                    rows[i].insert(num);

                    if cols[j].contains(&num) {
                        return false;
                    }
                    cols[j].insert(num);

                    let box_row = i / 3;
                    let box_col = j / 3;
                    if boxes[box_row][box_col].contains(&num) {
                        return false;
                    }
                    boxes[box_row][box_col].insert(num);
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku() {
        const BOARD: [[char; 9]; 9] = [
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let board_vec: Vec<Vec<char>> = BOARD.iter().map(|row| row.to_vec()).collect();

        assert!(Solution::is_valid_sudoku(board_vec));
    }

        #[test]
    fn test_invalid_sudoku() {
        const BOARD: [[char; 9]; 9] = [
            ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let board_vec: Vec<Vec<char>> = BOARD.iter().map(|row| row.to_vec()).collect();

        assert!(!Solution::is_valid_sudoku(board_vec));
    }
}
