use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut result = 0;

        let mut letters = HashMap::new();

        let mut left = 0;
        for (right, ch) in s.chars().enumerate() {
            if let Some(&prev_index) = letters.get(&ch) {
                left = left.max(prev_index + 1);
            }
            letters.insert(ch, right);

            let curr_length = (right - left + 1) as i32;
            result = result.max(curr_length);
        }

        result
    }
}