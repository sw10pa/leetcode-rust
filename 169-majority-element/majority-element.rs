impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut elem, mut count) = (nums[0], 0);
        for num in nums {
            if num == elem {
                count += 1;
            } else {
                count -= 1;
                if count == -1 {
                    elem = num;
                    count = 1;
                }
            }
        }
        elem
    }
}