impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        let initial = x;
        let mut reversed = 0;
    
        while x > 0 {
            reversed = reversed * 10 + x % 10;
            x /= 10;
        }

        initial == reversed
    }
}