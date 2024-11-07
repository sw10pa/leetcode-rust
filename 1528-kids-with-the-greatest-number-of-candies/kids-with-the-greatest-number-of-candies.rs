impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut max_candies = 0;
        for cur_candies in candies.iter() {
            max_candies = max_candies.max(*cur_candies);
        }

        let mut result = Vec::new();
        for cur_candies in candies.iter() {
            let is_greatest = (cur_candies + extra_candies >= max_candies);
            result.push(is_greatest);
        }
        result
    }
}