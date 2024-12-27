mod problems;

use problems::arrays::valid_sudoku::SolutionValidSudoku;
use problems::arrays::rotate_image::SolutionRotateImage;


fn main() {
    test_for_invalid_sudoku();
    test_rotate_image_3x3();
}

fn test_for_invalid_sudoku() {
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

    let result = SolutionValidSudoku::is_valid_sudoku(board_vec);
    println!("Is the Sudoku valid? {}", result);
}

fn test_rotate_image_3x3() {
    let mat: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    let mut mat_vec: Vec<Vec<i32>> = mat.iter().map(|row| row.to_vec()).collect();

    SolutionRotateImage::rotate_image(&mut mat_vec);
}
