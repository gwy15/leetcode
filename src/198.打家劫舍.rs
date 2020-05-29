/*
 * @lc app=leetcode.cn id=198 lang=rust
 *
 * [198] 打家劫舍
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut last, mut last_1) = (0, 0);
        for n in nums.iter() {
            let this = last.max(last_1 + n);
            last_1 = last;
            last = this;
        }
        last
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:expr) => {
            assert_eq!(
                Solution::rob(vec!$n),
                $ans
            );
        }
    };
    test!([1, 2, 3, 1], 4);
    test!([2, 7, 9, 3, 1], 12);
}
