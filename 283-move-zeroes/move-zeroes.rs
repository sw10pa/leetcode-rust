impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut non_zeroes = 0;
        for index in 0..nums.len() {
            if nums[index] != 0 {
                nums.swap(non_zeroes, index);
                non_zeroes += 1;
            }
        }
    }
}