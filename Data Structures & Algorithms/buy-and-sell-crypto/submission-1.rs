impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 0..prices.len() {
            for j in i..prices.len() {
                profit = profit.max(prices[j] - prices[i]);
            }
        }
        profit
    }
}
