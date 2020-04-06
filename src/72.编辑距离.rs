/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 * [72] 编辑距离
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (n, m) = (word1.len(), word2.len());
        if n > m {
            return Solution::min_distance(word2, word1);
        }
        let s1 = word1.into_bytes();
        let s2 = word2.into_bytes();
        let mut dp = vec![0; n + 1];
        // init dp
        for i in 0..=n {
            dp[i] = i as i32;
        }
        // go thru m
        for j in 1..=m {
            let mut tmp = vec![0; n + 1];
            tmp[0] = j as i32;
            // dp on n
            for i in 1..=n {
                tmp[i] = match s1[i - 1] == s2[j - 1] {
                    // dp[i][j] = dp[i-1][j-1]
                    true => dp[i - 1],
                    // dp[i][j] = min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1]) + 1
                    false => tmp[i - 1].min(dp[i]).min(dp[i - 1]) + 1,
                };
            }
            dp = tmp;
        }
        dp[n]
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s1:expr, $s2:expr, $ans:expr) => {
            assert_eq!(Solution::min_distance($s1.to_owned(), $s2.to_owned()), $ans);
        };
    };
    test!("horse", "ros", 3);
    test!("intention", "execution", 5);
}
