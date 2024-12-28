use std::collections::HashSet;


pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: Vec<i32>) -> i32 {
        let mut nums_hash_set: HashSet<i32> = HashSet::new();
        let mut nums2: Vec<i32> = Vec::new();

        for (_index, &element) in nums.iter().enumerate() {
            if nums_hash_set.insert(element) {
                nums2.push(element);
            }
        }

        nums2.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let nums: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];

        assert_eq!(Solution::remove_duplicates(nums), 5);
    }
}
