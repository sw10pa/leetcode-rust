impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut decreases = 0;

        for index in (0..nums.len()) {
            if nums[index] > nums[(index + 1) % nums.len()] {
                decreases += 1;
            }
        }

        (decreases <= 1)
    }
}