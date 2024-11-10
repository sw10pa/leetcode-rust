impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];

        let mut prefix = 1;
        for i in 0..n {
            result[i] = prefix;
            prefix *= nums[i];
        }

        let mut suffix = 1;
        for i in 0..n {
            let index = n - 1 - i;
            result[index] *= suffix;
            suffix *= nums[index];
        }

        result
    }
}