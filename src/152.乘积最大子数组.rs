/*
 * @lc app=leetcode.cn id=152 lang=rust
 *
 * [152] 乘积最大子数组
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut min = nums[0];
        let mut ans = max;
        for &n in &nums[1..] {
            let max_n = max * n;
            let min_n = min * n;
            max = n.max(max_n).max(min_n);
            min = n.min(max_n).min(min_n);
            ans = ans.max(max);
        }

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $ans:expr) => {
            assert_eq!(
                Solution::max_product(vec!$v),
                $ans
            );
        }
    };
    test!([2, 3, -2, 4], 6);
    test!([-2, 0, -1], 0);
    test!([1], 1);
    test!([-4, -3, -2], 12);
}
