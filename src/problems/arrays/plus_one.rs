pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();

        for i in (0..n).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }

        let mut result = vec![1];
        result.extend(digits);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        let inp: Vec<i32> = vec![1, 2, 3];
        let out: Vec<i32> = vec![1, 2, 4];

        assert_eq!(Solution::plus_one(inp), out);
    }

    #[test]
    fn test_plus_one_with_9() {
        let inp: Vec<i32> = vec![9];
        let out: Vec<i32> = vec![1, 0];

        assert_eq!(Solution::plus_one(inp), out);
    }
}
