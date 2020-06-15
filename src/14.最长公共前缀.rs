/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".into();
        }
        let mut prefix: Vec<char> = strs[0].chars().collect();
        for s in &strs[1..strs.len()] {
            prefix.truncate(s.len());
            for (j, ch) in s.chars().enumerate().take(prefix.len()) {
                if ch != prefix[j] {
                    prefix.truncate(j);
                    break;
                }
            }
        }

        prefix.into_iter().collect()
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:tt, $ans:expr) => {
            assert_eq!(
                Solution::longest_common_prefix(
                    vec!$s.into_iter().map(|s: &str| s.to_string()).collect()
                ),
                $ans
            );
        }
    };
    test!(["flower", "flow", "flight"], "fl");
    test!(["dog", "racecar", "car"], "");
    test!(["abc", "abcd", "ab"], "ab");
    test!([""], "");
    test!([], "");
}
