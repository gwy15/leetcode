/*
 * @lc app=leetcode.cn id=136 lang=rust
 *
 * [136] 只出现一次的数字
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |a, b| a ^ b)
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $a:expr) => {
            assert_eq!(
                Solution::single_number(vec!$v),
                $a
            );
        }
    };
    test!([2, 2, 1], 1);
    test!([4, 1, 2, 1, 2], 4);
    test!([1], 1);
}
