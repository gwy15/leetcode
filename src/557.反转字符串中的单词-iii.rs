/*
 * @lc app=leetcode.cn id=557 lang=rust
 *
 * [557] 反转字符串中的单词 III
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut ans = String::with_capacity(s.as_bytes().len());
        let mut stack = vec![];
        for ch in s.chars() {
            if ch == ' ' {
                while stack.len() > 0 {
                    ans.push(stack.pop().unwrap());
                }
                ans.push(' ');
            } else {
                stack.push(ch);
            }
        }
        while stack.len() > 0 {
            ans.push(stack.pop().unwrap());
        }
        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $ans:expr) => {
            assert_eq!(Solution::reverse_words($s.into()), $ans)
        };
    }
    test!("Let's take LeetCode contest", "s'teL ekat edoCteeL tsetnoc");
    test!("", "");
    test!("abc", "cba");
    test!("  ", "  ");
}
