impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut last_non_val_index = 0;
        for curr_index in 0..nums.len() {
            if nums[curr_index] != val {
                nums[last_non_val_index] = nums[curr_index];
                last_non_val_index += 1;
            }
        }
        last_non_val_index as i32
    }
}