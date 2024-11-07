impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let total_length = word1.len() + word2.len();
        let mut result = String::with_capacity(total_length);

        let max_length = word1.len().max(word2.len());

        let mut word1 = word1.chars();
        let mut word2 = word2.chars();

        for _ in 0..max_length {
            if let Some(ch) = word1.next() {
                result.push(ch);
            }
            if let Some(ch) = word2.next() {
                result.push(ch);
            }
        }

        result
    }
}