/*
 * @lc app=leetcode.cn id=953 lang=rust
 *
 * [953] 验证外星语词典
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        const OFFSET: usize = b'a' as usize;
        // map from char to its order
        let mut map = [-1; 26];
        for (i, &ch) in order.as_bytes().iter().enumerate() {
            map[ch as usize - OFFSET] = i as i32;
        }
        // Return if s1 < s2
        let cmp = |s1: &[u8], s2: &[u8]| {
            let (l1, l2) = (s1.len(), s2.len());
            let mut i = 0;
            while i < l1 && i < l2 {
                let (v1, v2) = (map[s1[i] as usize - OFFSET], map[s2[i] as usize - OFFSET]);
                if v1 < v2 {
                    return true;
                }
                if v1 > v2 {
                    return false;
                }
                i += 1;
            }
            // if s1 is shorter, s1 < s2
            i == l1
        };

        for (s1, s2) in words.iter().zip(words.iter().skip(1)) {
            if !cmp(s1.as_bytes(), s2.as_bytes()) {
                return false;
            }
        }
        true
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        (words=$words:tt, order=$order:expr, $ans:expr) => {
            assert_eq!(
                Solution::is_alien_sorted(vec_string!$words, $order.into()),
                $ans
            )
        };
    }
    test!(
        words = ["hello", "leetcode"],
        order = "hlabcdefgijkmnopqrstuvwxyz",
        true
    );
    test!(
        words = ["word", "world", "row"],
        order = "worldabcefghijkmnpqstuvxyz",
        false
    );
    test!(
        words = ["", "row", "world"],
        order = "worldabcefghijkmnpqstuvxyz",
        false
    );
    test!(
        words = ["apple", "app"],
        order = "abcdefghijklmnopqrstuvwxyz",
        false
    );
}
