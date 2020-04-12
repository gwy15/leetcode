/*
 * @lc app=leetcode.cn id=409 lang=rust
 *
 * [409] 最长回文串
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        const a: usize = 'a' as usize;
        const A: usize = 'A' as usize;
        let mut counter = vec![0; 26 * 2];
        for ch in s.chars() {
            match ch {
                'a'..='z' => {
                    counter[ch as usize - a] += 1;
                }
                'A'..='Z' => {
                    counter[26 + ch as usize - A] += 1;
                }
                _ => unreachable!(),
            }
        }
        let mut has_odd = 0;
        let mut length = 0;
        for count in counter.iter() {
            length += count & (!1);
            has_odd += count & 1;
        }
        length + has_odd.min(1)
    }
}
// @lc code=end
#[test]
fn test_solution() {
    assert_eq!(Solution::longest_palindrome("abccccdd".to_owned()), 7);
    assert_eq!(Solution::longest_palindrome("abcd".to_owned()), 1);
    assert_eq!(Solution::longest_palindrome("AAAAAA".to_owned()), 6);
}
