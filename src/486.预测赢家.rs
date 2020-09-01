/*
 * @lc app=leetcode.cn id=486 lang=rust
 *
 * [486] 预测赢家
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        // dp(i,j) = max(
        //    nums[i] - dp(i+1,j),
        //    nums[j] - dp(i,j+1))
        let n = nums.len();
        let mut last_dp = vec![];
        let mut dp;
        for j in 0..n {
            dp = vec![0; j + 1];
            for i in (0..=j).rev() {
                if i == j {
                    dp[i] = nums[i];
                } else {
                    dp[i] = nums[i] - dp[i + 1];
                    dp[i] = dp[i].max(nums[j] - last_dp[i]);
                }
            }
            last_dp = dp;
        }
        // dp(0, n-1)
        last_dp[0] >= 0
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(
                Solution::predict_the_winner(vec!$args),
                $ans
            )
        };
    }
    test!([1, 5, 2], false);
    test!([1, 5, 233, 7], true);
    test!([0], true);
    test!([0, 0, 7, 6, 5, 6, 1], false);
}
