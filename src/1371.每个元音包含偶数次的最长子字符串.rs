/*
 * @lc app=leetcode.cn id=1371 lang=rust
 *
 * [1371] 每个元音包含偶数次的最长子字符串
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut best_length = 0;
        let mut first_seen: Vec<i32> = vec![-1; 1 << 5];
        let mut state = 0;
        for (i, c) in s.chars().enumerate() {
            // save first seen state position
            if first_seen[state] < 0 {
                first_seen[state] = i as i32;
            }
            // change state
            match c {
                'a' => state ^= 1 << 0,
                'i' => state ^= 1 << 1,
                'u' => state ^= 1 << 2,
                'e' => state ^= 1 << 3,
                'o' => state ^= 1 << 4,
                _ => {}
            }
            // find length
            if first_seen[state] >= 0 {
                let length = i as i32 - first_seen[state] + 1;
                best_length = best_length.max(length);
            }
        }
        best_length
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $ans:expr) => {
            assert_eq!(Solution::find_the_longest_substring($s.into()), $ans);
        };
    };
    test!("eleetminicoworoep", 13);
    test!("leetcodeisgreat", 5);
    test!("bcbcbc", 6);
    test!("", 0);
    test!("a", 0);
    test!("aa", 2);
}
