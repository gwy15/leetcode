/*
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 * [1143] 最长公共子序列
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let n1 = text1.len();
        let n2 = text2.len();
        // preferably, n1 <= n2
        if n1 > n2 {
            return Solution::longest_common_subsequence(text2, text1);
        }
        // thus we can save some memory :)
        let mut dp = vec![0; n1];
        let ch1 = text1.as_bytes();
        let ch2 = text2.as_bytes();
        for j in 0..n2 {
            // update dp
            let mut tmp = vec![0; n1];
            tmp[0] = match ch1[0] == ch2[j] {
                true => 1,
                false => dp[0],
            };
            for i in 1..n1 {
                tmp[i] = match ch1[i] == ch2[j] {
                    true => 1 + dp[i - 1],
                    false => i32::max(tmp[i - 1], dp[i]),
                };
            }
            dp = tmp;
        }
        dp[n1 - 1]
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($s1:expr, $s2:expr, $ans:expr) => {
            assert_eq!(
                Solution::longest_common_subsequence($s1.to_owned(), $s2.to_owned()),
                $ans
            );
        };
    };
    test!("abcde", "ace", 3);
    test!("abc", "abc", 3);
    test!("abc", "def", 0);
}
