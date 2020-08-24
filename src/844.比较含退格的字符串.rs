/*
 * @lc app=leetcode.cn id=844 lang=rust
 *
 * [844] 比较含退格的字符串
 */
struct Solution;
// @lc code=start
impl Solution {
    fn trim(s: String) -> String {
        let mut text = String::new();
        for ch in s.chars() {
            if ch == '#' {
                text.pop();
            } else {
                text.push(ch);
            }
        }
        text
    }
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::trim(s) == Self::trim(t)
    }
}
// @lc code=end
