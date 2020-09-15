struct Solution;

impl Solution {
    pub fn minimum_operations(leaves: String) -> i32 {
        const INF: i32 = i32::max_value() / 2;
        let mut dp = [INF, INF, INF];
        for (i, ch) in leaves.chars().enumerate() {
            if ch == 'r' {
                // 0
                dp = [dp[0], dp[0].min(dp[1]) + 1, dp[1].min(dp[2])];
                if i == 0 {
                    dp[0] = 0;
                }
            } else {
                // 1
                dp = [dp[0] + 1, dp[0].min(dp[1]), dp[1].min(dp[2]) + 1];
                if i == 0 {
                    dp[0] = 1;
                }
            }
        }

        dp[2]
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(Solution::minimum_operations($args.into()), $ans)
        };
    }
    test!("rrryyyrryyyrr", 2);
    test!("ryr", 0);
    test!("rrrrrrrrrrrrrrr", 1);
    test!("yyy", 2);
    test!("yyyyyyyy", 2);
    test!("ryyyy", 1);
    test!("yyyyr", 1);
    test!("rrr", 1);
}
