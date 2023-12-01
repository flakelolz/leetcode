#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = 1;
        let mut max_profit = 0;

        while right < prices.len() {
            if prices[left] < prices[right] {
                let profit = prices[right] - prices[left];
                max_profit = max_profit.max(profit);
            } else {
                left = right;
            }

            right += 1;
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_profit_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(prices), 5);
    }

    #[test]
    fn max_profit_2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices), 0);
    }
}

