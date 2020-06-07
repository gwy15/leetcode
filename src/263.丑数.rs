/*
 * @lc app=leetcode.cn id=263 lang=rust
 *
 * [263] 丑数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_ugly(mut num: i32) -> bool {
        for factor in [2, 3, 5].iter() {
            while num % factor == 0 && num != 0 {
                num /= factor;
            }
        }
        num == 1
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $ans:expr) => {
            assert_eq!(Solution::is_ugly($n), $ans);
        };
    };
    test!(6, true);
    test!(8, true);
    test!(14, false);
    test!(1, true);
    test!(0, false);
}
