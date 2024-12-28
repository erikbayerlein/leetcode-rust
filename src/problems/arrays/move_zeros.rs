pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut index = 0;

        let len = nums.len();
        for i in 0..len {
            if nums[i] != 0 {
                nums[index] = nums[i];
                index += 1;
            }
        }

        while index < len {
            nums[index] = 0;
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut inp: Vec<i32> = vec![1, 0, 2, 0, 3];
        let out: Vec<i32> = vec![1, 2, 3, 0, 0];

        Solution::move_zeroes(&mut inp);
        assert_eq!(inp, out);
    }

    #[test]
    fn test_move_only_one_zero() {
        let mut inp: Vec<i32> = vec![0];
        let out: Vec<i32> = vec![0];

        Solution::move_zeroes(&mut inp);
        assert_eq!(inp, out);
    }
}
