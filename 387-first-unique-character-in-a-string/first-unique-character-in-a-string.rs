use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut letters: HashMap<char, u32> = HashMap::new();

        for ch in s.chars() {
            *letters.entry(ch).or_insert(0) += 1;
        }

        for (index, ch) in s.chars().enumerate() {
            if letters[&ch] == 1 {
                return index as i32;
            }
        }

        -1
    }
}