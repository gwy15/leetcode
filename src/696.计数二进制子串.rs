/*
 * @lc app=leetcode.cn id=696 lang=rust
 *
 * [696] 计数二进制子串
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn count_binary_substrings(mut s: String) -> i32 {
        s.push('$');
        let (mut last_count, mut cur_count, mut cur_char) = (0, 0, '^');
        let mut ans = 0;
        for ch in s.chars() {
            if cur_char == ch {
                cur_count += 1;
            } else {
                ans += cur_count.min(last_count);
                last_count = cur_count;
                cur_count = 1;
                cur_char = ch;
            }
        }
        ans
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($s:tt, $ans:expr) => {
            assert_eq!(Solution::count_binary_substrings($s.into()), $ans)
        };
    }
    test!("00110011", 6);
    test!("10101", 4);
    test!("0", 0);
    test!("10", 1);
    test!("", 0);
}
