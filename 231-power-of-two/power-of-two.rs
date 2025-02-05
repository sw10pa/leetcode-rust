impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n < 1 {
            return false;
        }

        if n == 1 {
            return true;
        }

        n % 2 == 0 && Self::is_power_of_two(n / 2)
    }
}