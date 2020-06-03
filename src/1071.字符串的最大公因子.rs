/*
 * @lc app=leetcode.cn id=1071 lang=rust
 *
 * [1071] 字符串的最大公因子
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut s1 = str1.into_bytes();
        let mut s2 = str2.into_bytes();
        while s2.len() > 0 {
            // gcd(s1, s2) = gcd(s2, s1 % s2)
            // find s1 % s2
            while s1.len() >= s2.len() {
                // try right-strip s2 from s1
                let (m, n) = (s1.len(), s2.len());
                if &s1[m - n..m] != &s2[..] {
                    return "".into();
                }
                s1.truncate(m - n);
            }
            // now swap s1 s2
            std::mem::swap(&mut s1, &mut s2);
        }

        // String::from_utf8(s1).unwrap()
        unsafe { String::from_utf8_unchecked(s1) }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s1:expr, $s2:expr, $ans:expr) => {
            assert_eq!(Solution::gcd_of_strings($s1.into(), $s2.into()), $ans);
        };
    };
    test!("ABCABC", "ABC", "ABC");
    test!("ABABAB", "ABAB", "AB");
    test!("LEET", "CODE", "");
}
