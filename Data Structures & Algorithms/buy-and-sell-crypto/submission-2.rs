impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy_at = 0;
        let mut sell_at = 1;
        let mut max_profit = 0;

        while sell_at < prices.len() {
            let profit = prices[sell_at] - prices[buy_at];
            if profit > 0 {
                max_profit = max_profit.max(profit);
                sell_at += 1;
            } else {
                buy_at = sell_at;
                sell_at += 1;
            }
        }

        max_profit
    }
}
