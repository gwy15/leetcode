/*
 * @lc app=leetcode.cn id=345 lang=rust
 *
 * [345] 反转字符串中的元音字母
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        if n == 0 {
            return s;
        }
        let (mut left, mut right) = (0, n - 1);

        #[inline(always)]
        fn is_vowel(c: char) -> bool {
            "aeiouAEIOU".contains(c)
        }

        while left < right {
            while left < right && !is_vowel(chars[left]) {
                left += 1;
            }
            while left < right && !is_vowel(chars[right]) {
                right -= 1;
            }
            if left < right {
                chars.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        chars.into_iter().collect()
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:expr, $ans:expr) => {
            assert_eq!(Solution::reverse_vowels(($args).into()), $ans)
        };
    }
    test!("hello", "holle");
    test!("leetcode", "leotcede");
    test!("", "");
    test!("a", "a");
    test!("aE", "Ea");
}
