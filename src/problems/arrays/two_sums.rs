use std::collections::HashMap;


pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (index, &element) in nums.iter().enumerate() {
            let complement = target - element;

            if let Some(&idx) = map.get(&complement) {
                return vec![idx as i32, index as i32];
            }

            map.insert(element, index);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sums() {
        let inp: Vec<i32> = vec![2,7,11,15];
        let out: Vec<i32> = vec![0, 1];

        assert_eq!(Solution::two_sum(inp, 9), out);
    }

    #[test]
    fn test_two_sums_2() {
        let inp: Vec<i32> = vec![3, 2, 4];
        let out: Vec<i32> = vec![1, 2];

        assert_eq!(Solution::two_sum(inp, 6), out);
    }

    #[test]
    fn test_two_sums_3() {
        let inp: Vec<i32> = vec![3, 3];
        let out: Vec<i32> = vec![0, 1];

        assert_eq!(Solution::two_sum(inp, 6), out);
    }
}
