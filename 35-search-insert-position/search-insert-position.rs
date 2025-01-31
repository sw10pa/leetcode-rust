impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);

        while left <= right {
            let mut middle = (left + right) / 2;
            println!("{}", middle);

            if target < nums[middle as usize] {
                right = middle - 1;
            } else if target > nums[middle as usize] {
                left = middle + 1;
            } else {
                return middle as i32;
            }
        }

        left as i32
    }
}