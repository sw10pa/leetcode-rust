impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut length = 0;

        for ch in s.chars().rev() {
            if ch != ' ' {
                length += 1;
            } else if length > 0 {
                break;
            }
        }

        length
    }
}