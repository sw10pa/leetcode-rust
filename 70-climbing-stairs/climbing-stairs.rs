impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut fib1, mut fib2, mut fib3) = (1, 1, 2);
        for _ in 0..n {
            fib1 = fib2;
            fib2 = fib3;
            fib3 = fib1 + fib2;
        }
        fib1
    }
}