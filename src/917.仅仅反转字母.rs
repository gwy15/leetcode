/*
 * @lc app=leetcode.cn id=917 lang=rust
 *
 * [917] 仅仅反转字母
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn reverse_only_letters(s: String) -> String {
        let n = s.len() as i32;
        if n <= 1 {
            return s;
        }
        let mut s = s.into_bytes();

        let (mut left, mut right): (i32, i32) = (0, n - 1);
        while left < right {
            while left < n && !s[left as usize].is_ascii_alphabetic() {
                left += 1;
            }
            while right >= 0 && !s[right as usize].is_ascii_alphabetic() {
                right -= 1;
            }
            if left < n && right >= 0 && left < right {
                s.swap(left as usize, right as usize);
                left += 1;
                right -= 1;
            }
        }

        unsafe { String::from_utf8_unchecked(s) }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $ans:expr) => {
            assert_eq!(Solution::reverse_only_letters($s.into()), $ans);
        };
    };
    test!("ab-cd", "dc-ba");
    test!("a-bC-dEf-ghIj", "j-Ih-gfE-dCba");
    test!("Test1ng-Leet=code-Q!", "Qedo1ct-eeLg=ntse-T!");
    test!("", "");
    test!("a", "a");
    test!("7_28]", "7_28]");
    test!("?6C40E", "?6E40C");
}
