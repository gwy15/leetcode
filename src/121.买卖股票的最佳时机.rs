/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }
        let mut max_profit = 0;
        let mut min_buy_price = *prices.iter().max().unwrap();
        for price in prices.into_iter() {
            max_profit = max_profit.max(price - min_buy_price);
            min_buy_price = min_buy_price.min(price);
        }
        max_profit
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(
                Solution::max_profit(vec!$args),
                $ans
            )
        };
    }
    test!([7, 1, 5, 3, 6, 4], 5);
    test!([7, 6, 4, 3, 1], 0);
    test!([], 0);
}
