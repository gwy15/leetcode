/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] 最小覆盖子串
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        // init counter
        let mut counter = [0i32; 256];
        for ch in t.chars() {
            counter[ch as usize] += 1;
        }
        let mut entries_count = counter.iter().filter(|&&v| v > 0).count();
        // slide window
        let n = s.len();
        let ss: Vec<char> = s.chars().collect();
        let (mut res_start, mut res_len) = (0, n + 1);
        let (mut l, mut r) = (0, 0); // [l, r)
        while r < n {
            // move r
            let ch = ss[r];
            counter[ch as usize] -= 1;
            if counter[ch as usize] == 0 {
                entries_count -= 1;
            }
            r += 1;
            // move l
            while entries_count == 0 {
                if r - l < res_len {
                    res_len = r - l;
                    res_start = l;
                }

                // shrink
                let ch = ss[l];
                counter[ch as usize] += 1;
                if counter[ch as usize] == 1 {
                    entries_count += 1;
                }
                l += 1;
            }
        }
        if res_len == n + 1 {
            "".into()
        } else {
            s[res_start..res_start + res_len].into()
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $t:expr, $ans:expr) => {
            assert_eq!(Solution::min_window($s.into(), $t.into()), $ans.to_string());
        };
    };
    test!("ADOBECODEBANC", "ABC", "BANC");
    test!("AAAAAAAAAAAAA", "AA", "AA");
    test!("a", "aa", "");
}
