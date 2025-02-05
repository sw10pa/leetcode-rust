use std::cmp::{max, min};

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;

        let mut min_price = prices[0];
        for price in prices {
            let cur_profit = price - min_price;
            max_profit = max(max_profit, cur_profit);

            min_price = min(min_price, price);
        }

        max_profit
    }
}