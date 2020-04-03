/*
 * @lc app=leetcode.cn id=322 lang=rust
 *
 * [322] 零钱兑换
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        const FAIL: i32 = -1;

        let mut dp = vec![0; amount + 1];
        for target in 1..=amount {
            dp[target] = coins
                .iter()
                .filter(|&&change| change as usize <= target)
                .map(|&change| match dp[target - change as usize] {
                    FAIL => FAIL,
                    count => count + 1,
                })
                .filter(|&count| count > 0)
                .min()
                .unwrap_or(FAIL);
        }
        dp[amount]
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($v:expr, $amount:expr, $ans:expr) => {
            assert_eq!(Solution::coin_change($v, $amount), $ans);
        };
    };
    test!(vec![1, 2, 5], 11, 3);
    test!(vec![2], 3, -1);
}
