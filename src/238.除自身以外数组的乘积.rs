/*
 * @lc app=leetcode.cn id=238 lang=rust
 *
 * [238] 除自身以外数组的乘积
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![1; n];

        let mut pre = 1;
        for i in 0..n {
            ans[i] *= pre;
            pre *= nums[i];
        }
        let mut post = 1;
        for i in (0..n).rev() {
            ans[i] *= post;
            post *= nums[i];
        }

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:tt) => {
            assert_eq!(
                Solution::product_except_self(vec!$n),
                vec!$ans
            );
        }
    };
    test!([1, 2, 3, 4], [24, 12, 8, 6]);
}
