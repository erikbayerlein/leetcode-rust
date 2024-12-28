use std::collections::HashMap;


pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn intersection(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut freq_hash_map: HashMap<i32, i32> = HashMap::new();

        for element in arr1 {
            *freq_hash_map.entry(element).or_insert(0) += 1;
        }

        let mut result = Vec::new();

        for element in arr2 {
            if let Some(count) = freq_hash_map.get_mut(&element) {
                if *count > 0 {
                    result.push(element);
                    *count -= 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let arr1: Vec<i32> = vec![1, 2, 2, 1];
        let arr2: Vec<i32> = vec![2, 2];

        assert_eq!(Solution::intersection(arr1, arr2), [2, 2]);
    }
}
