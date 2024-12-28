pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rotate_arrays(arr: &mut Vec<i32>, k: usize) {
        let n = arr.len();
        let k = k % n;

        let aux = arr[..n - k].to_vec();
        let aux2 = arr[n - k..].to_vec();

        arr.clear();
        arr.extend(aux2);
        arr.extend(aux);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_arrays() {
        let mut arr: Vec<i32> = vec![1, 2, 3, 4];
        let rotated_arr: Vec<i32> = vec![4, 1, 2, 3];

        Solution::rotate_arrays(&mut arr, 1);
        assert_eq!(arr, rotated_arr);
    }
}
