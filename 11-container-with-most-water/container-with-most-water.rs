use std::cmp::max;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;

        let (mut left, mut right) = (0, height.len() - 1);
        while left < right {
            let width = (right - left) as i32;

            if height[left] < height[right] {
                let cur_area = width * height[left];
                max_area = max(max_area, cur_area);
                left += 1;
            } else {
                let cur_area = width * height[right];
                max_area = max(max_area, cur_area);
                right -= 1;
            }
        }

        max_area
    }
}