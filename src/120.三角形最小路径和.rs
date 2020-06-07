/*
 * @lc app=leetcode.cn id=120 lang=rust
 *
 * [120] 三角形最小路径和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();

        let mut last_row = vec![triangle[0][0]];
        for i in 1..n {
            // current row has i+1 elements
            let mut dp = vec![i32::max_value(); i + 1];
            for j in 0..=i {
                if j < i {
                    dp[j] = dp[j].min(last_row[j] + triangle[i][j]);
                }
                if j > 0 {
                    dp[j] = dp[j].min(last_row[j - 1] + triangle[i][j]);
                }
            }
            // println!("dp = {:?}", dp);
            last_row = dp;
        }

        last_row.into_iter().min().unwrap()
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($t:tt, $ans:expr) => {
            assert_eq!(
                Solution::minimum_total(vec2d!$t),
                $ans
            );
        }
    };
    test!([[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]], 11);
    test!([[1]], 1);
}
