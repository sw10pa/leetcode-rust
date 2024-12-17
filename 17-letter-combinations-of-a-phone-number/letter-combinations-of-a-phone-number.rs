impl Solution {
    const KEYBOARD: [&'static str; 10] = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result = Vec::new();
        Self::helper(&mut String::new(), &digits, 0, &mut result);
        result
    }

    fn helper(current: &mut String, digits: &String, index: usize, result: &mut Vec<String>) -> () {
        if index == digits.len() {
            if !current.is_empty() {
                result.push(current.clone());
            }
            return;
        }

        let digit = digits.chars().nth(index).unwrap().to_digit(10).unwrap();
        let letters = Self::KEYBOARD[digit as usize].chars();

        for ch in letters {
            current.push(ch);
            Self::helper(current, digits, index + 1, result);
            current.pop();
        }
    }
}