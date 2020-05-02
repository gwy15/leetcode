/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut counter = [0; 255];
        let mut l = 0;
        let mut r = 0;
        let mut max_length = 0;
        while r < n {
            // increase l
            while counter[s[r] as usize] > 0 {
                counter[s[l] as usize] -= 1;
                l += 1;
            }
            // now increase r
            counter[s[r] as usize] += 1;
            r += 1;
            max_length = max_length.max(r - l);
        }
        max_length as i32
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $a:expr) => {
            assert_eq!(Solution::length_of_longest_substring($s.into()), $a);
        };
    };
    test!("abcabcbb", 3);
    test!("bbbbb", 1);
    test!("pwwkew", 3);
    test!("", 0);
    test!("a", 1);
    test!("asda", 3);
}
