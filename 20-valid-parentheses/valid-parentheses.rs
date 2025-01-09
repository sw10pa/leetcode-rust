impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for ch in s.chars() {
            match ch {
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                },
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                },
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                },
                _ => {
                    stack.push(ch);
                }
            }
        }

        stack.is_empty()
    }
}