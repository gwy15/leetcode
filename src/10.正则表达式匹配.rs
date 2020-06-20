/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // eprintln!("matching s={}, p={}", s, p);
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let (m, n) = (s.len(), p.len());
        let mut dp = vec![vec![false; n + 1]; m + 1];

        macro_rules! matches {
            ($i:expr, $j:expr) => {
                // j != 0, i == 0
                if $i == 0 {
                    false
                } else if p[$j - 1] == '.' {
                    true
                } else {
                    s[$i - 1] == p[$j - 1]
                }
            };
        }

        dp[0][0] = true;
        for i in 0..=m {
            for j in 1..=n {
                dp[i][j] = match p[j - 1] {
                    'a'..='z' | '.' => {
                        if matches!(i, j) {
                            dp[i - 1][j - 1]
                        } else {
                            false
                        }
                    }
                    '*' => {
                        let zero_match = dp[i][j - 2];
                        if matches!(i, j - 1) {
                            zero_match || dp[i - 1][j]
                        } else {
                            zero_match
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }
        dp[m][n]
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $p:expr, $ans:expr) => {
            assert_eq!(Solution::is_match($s.into(), $p.into()), $ans);
        };
    };
    test!("", "a*", true);
    test!("aa", "a", false);
    test!("aa", "a*", true);
    test!("ab", ".*", true);
    test!("aab", "c*a*b", true);
    test!("mississippi", "mis*is*p*.", false);
}
