pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rotate_image(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();
        for i in 0..matrix.len() {
            for j in i + 1..matrix[0].len() {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_image_3x3() {
        let mat: [[i32; 3]; 3] = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ];
        let mut mat_vec: Vec<Vec<i32>> = mat.iter().map(|row| row.to_vec()).collect();

        let mat_answer: [[i32; 3]; 3] = [
            [7, 4, 1],
            [8, 5, 2],
            [9, 6, 3]
        ];
        let mat_answer_vec: Vec<Vec<i32>> = mat_answer.iter().map(|row| row.to_vec()).collect();

        Solution::rotate_image(&mut mat_vec);

        assert_eq!(mat_vec, mat_answer_vec);
    }

    #[test]
    fn test_rotate_image_4x4() {
        let mat: [[i32; 4]; 4] = [
            [5, 1, 9, 11],
            [2, 4, 8, 10],
            [13, 3, 6, 7],
            [15, 14, 12, 16],
        ];
        let mut mat_vec: Vec<Vec<i32>> = mat.iter().map(|row| row.to_vec()).collect();

        let mat_answer: [[i32; 4]; 4] = [
            [15, 13, 2, 5],
            [14, 3, 4, 1],
            [12, 6, 8, 9],
            [16, 7, 10, 11],
        ];
        let mat_answer_vec: Vec<Vec<i32>> = mat_answer.iter().map(|row| row.to_vec()).collect();

        Solution::rotate_image(&mut mat_vec);

        assert_eq!(mat_vec, mat_answer_vec);
    }
}
