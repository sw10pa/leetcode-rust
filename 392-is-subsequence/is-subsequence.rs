impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (mut s, t) = (s.chars(), t.chars());

        let mut s_char = s.next();
        for t_char in t {
            if Some(t_char) == s_char {
                s_char = s.next()
            }
        }

        s_char.is_none()
    }
}