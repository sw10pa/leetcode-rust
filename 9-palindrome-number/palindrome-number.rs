impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let (mut reversed_x, mut current) = (0, x);

        while current > 0 {
            reversed_x = reversed_x * 10 + current % 10;
            current /= 10;
        }

        reversed_x == x
    }
}