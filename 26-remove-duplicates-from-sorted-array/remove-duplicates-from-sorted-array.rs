impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut unique = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[unique] = nums[i];
                unique += 1
            }
        }

        unique as i32
    }
}