impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut fib1, mut fib2) = (1, 2);
        for _ in 1..n {
            let temp = fib1 + fib2;
            fib1 = fib2;
            fib2 = temp;
        }
        fib1
    }
}