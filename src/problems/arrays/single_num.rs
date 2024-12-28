pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn single_num(nums: Vec<i32>) -> i32 {
        let mut answer = 0;

        for element in nums {
            answer ^= element;
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_num() {
        let arr = vec![2, 2, 1];

        assert_eq!(Solution::single_num(arr), 1)
    }
}
