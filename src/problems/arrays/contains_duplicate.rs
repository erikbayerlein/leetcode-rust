use std::collections::HashSet;


pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_duplicate(arr: Vec<i32>) -> bool {
        let mut arr_hash_set: HashSet<i32> = HashSet::new();

        for &element in arr.iter() {
            if !arr_hash_set.insert(element) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let arr: Vec<i32> = vec![1, 2, 3, 4, 1];

        assert!(Solution::contains_duplicate(arr));
    }

    #[test]
    fn test_doesnt_contain_duplicate() {
        let arr: Vec<i32> = vec![1, 2, 3, 4];

        assert_eq!(Solution::contains_duplicate(arr), false);
    }
}
