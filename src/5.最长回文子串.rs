/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */
struct Solution;
// @lc code=start
impl Solution {
    /// abc => #a#b#c#
    fn fill_string(s: &String) -> Vec<char> {
        let mut v = Vec::new();
        v.reserve(s.len() * 2 + 1);
        for c in s.chars() {
            v.push('#');
            v.push(c);
        }
        v.push('#');
        v
    }
    #[allow(unused)]
    pub fn longest_palindrome(raw_s: String) -> String {
        let s = Self::fill_string(&raw_s);
        let n = s.len() as i32;
        // cache radius
        let mut radius = vec![0; n as usize];
        let (mut l, mut r): (i32, i32) = (0, -1);
        for center in 0..n {
            // init radius of i
            let mut radius_i = if (center > r) {
                1
            } else {
                (r - center).min(radius[(l + r - center) as usize])
            };
            // expand
            while (center - radius_i >= 0
                && center + radius_i < n
                && s[(center - radius_i) as usize] == s[(center + radius_i) as usize])
            {
                radius_i += 1;
            }
            radius_i -= 1;
            radius[center as usize] = radius_i;
            if (center + radius_i > r) {
                l = center - radius_i;
                r = center + radius_i;
            }
        }
        // find longest result
        let (center, &max_radius) = radius.iter().enumerate().max_by_key(|(_, &r)| r).unwrap();
        let c = center / 2;
        let r = (max_radius / 2) as usize;
        if center % 2 == 0 {
            // #
            raw_s[c - r..c + r].into()
        } else {
            // char
            raw_s[c - r..c + r + 1].into()
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $ans:expr) => {
            assert_eq!(Solution::longest_palindrome($s.into()), String::from($ans));
        };
    };
    test!("ababc", "bab");
    test!("", "");
    test!("a", "a");
    test!("aa", "aa");
    test!("aabbaa", "aabbaa");
}
