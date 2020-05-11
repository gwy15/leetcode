/*
 * @lc app=leetcode.cn id=221 lang=rust
 *
 * [221] 最大正方形
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        if m == 0 {
            return 0;
        }
        let n = matrix[0].len();

        let mut max_size = 0;
        let mut last_dp = vec![0; n];
        for i in 0..m {
            let mut dp = vec![0; n];
            for j in 0..n {
                dp[j] = match matrix[i][j] {
                    '0' => 0,
                    '1' => match i * j {
                        0 => 1,
                        _ => 1 + dp[j - 1].min(last_dp[j]).min(last_dp[j - 1]),
                    },
                    _ => unreachable!(),
                };
                max_size = max_size.max(dp[j]);
            }
            last_dp = dp;
        }
        max_size * max_size
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! vec2d {
        [$(
            [$(
                $i:expr
            ),*]
        ),*] => {
            vec![$(
                vec![$($i),*]
            ),*]
        }
    }
    macro_rules! test {
        ($v:tt, $a:expr) => {
            assert_eq!(
                Solution::maximal_square(vec2d!$v),
                $a
            );
        }
    };
    test!(
        [
            ['1', '0', '1', '0', '0'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '0', '0', '1', '0']
        ],
        4
    );
}
