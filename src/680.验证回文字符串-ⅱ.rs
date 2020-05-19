/*
 * @lc app=leetcode.cn id=680 lang=rust
 *
 * [680] 验证回文字符串 Ⅱ
 */
struct Solution;
// @lc code=start
impl Solution {
    fn is_palindrome(s: &[u8]) -> bool {
        let n = s.len();
        if n <= 1 {
            return true;
        }
        let (mut i, mut j) = (0, n - 1);
        while i < j {
            if s[i] == s[j] {
                i += 1;
                j -= 1;
            } else {
                return false;
            }
        }
        true
    }
    #[allow(unused)]
    pub fn valid_palindrome(s: String) -> bool {
        let n = s.len();
        if n <= 1 {
            return true;
        }
        let s = s.into_bytes();
        // begin iterate
        let (mut i, mut j) = (0, n - 1);
        while i < j {
            if s[i] == s[j] {
                i += 1;
                j -= 1;
            } else {
                return Self::is_palindrome(&s[i..j]) || Self::is_palindrome(&s[i + 1..j + 1]);
            }
        }
        true
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $ans:expr) => {
            assert_eq!(Solution::valid_palindrome($s.into()), $ans);
        };
    };
    test!("aba", true);
    test!("", true);
    test!("a", true);
    test!("ab", true);
    test!("abcde", false);
}
