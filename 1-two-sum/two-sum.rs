use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut visited = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            let needed = target - num;
            if let Some(&needed_index) = visited.get(&needed) {
                return vec![needed_index as i32, index as i32];
            }

            visited.insert(num, index);
        }

        vec![]
    }
}