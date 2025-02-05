use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut unique_nums = HashSet::new();
        for num in &nums {
            unique_nums.insert(num);
        }
        unique_nums.len() != nums.len()
    }
}