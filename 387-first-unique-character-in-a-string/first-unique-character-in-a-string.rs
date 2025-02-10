use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut letters = vec![0; 26];

        for ch in s.bytes() {
            letters[Self::get_char_index(ch)] += 1;
        }

        for (index, ch) in s.bytes().enumerate() {
            if letters[Self::get_char_index(ch)] == 1 {
                return index as i32;
            }
        }

        -1
    }

    fn get_char_index(ch: u8) -> usize {
        (ch - b'a') as usize
    }
}