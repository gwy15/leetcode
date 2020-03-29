/*
 * @lc app=leetcode.cn id=903 lang=rust
 *
 * [903] DI 序列的有效排列
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn num_perms_di_sequence(s: String) -> i32 {
        const MOD: i32 = 1_000_000_000 + 7;
        let n = s.len();

        let s_chars: Vec<char> = s.chars().collect();
        let mut dp = vec![0; 1];
        // i=0, dp(0, 0) = 1
        dp[0] = 1;
        for i in 1..n + 1 {
            // j in [0, i]
            let mut new_dp = vec![0; i + 1];
            match s_chars[i - 1] {
                'D' => {
                    // skip j=i (default as 0)
                    for j in (0..i).rev() {
                        new_dp[j] = (new_dp[j + 1] + dp[j]) % MOD;
                    }
                }
                'I' => {
                    // skip j=0 (default as 0)
                    for j in 1..i + 1 {
                        new_dp[j] = (new_dp[j - 1] + dp[j - 1]) % MOD;
                    }
                }
                _ => unreachable!(),
            }
            dp = new_dp;
        }

        dp.into_iter().fold(0, |a, b| (a + b) % MOD)
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $ans:expr) => {
            assert_eq!(Solution::num_perms_di_sequence($n.to_owned()), $ans);
        };
    }
    test!("D", 1);
    test!("DD", 1);
    test!("DID", 5);
    test!("DDDDDDDDD", 1);
    test!("DIDDDDIID", 3629);
    test!("IDDDIIDIIIIIIIIDIDID", 853197538);
}
