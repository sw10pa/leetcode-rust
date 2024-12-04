impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut length = 0;

        let first_str = &strs[0];
        'outer: for (i, c) in first_str.chars().enumerate() {
            for s in &strs {
                if i >= s.len() || s.chars().nth(i) != Some(c) {
                    break 'outer;
                }
            }
            length += 1;
        }

        first_str.chars().take(length).collect()
    }
}