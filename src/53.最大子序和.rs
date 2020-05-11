/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子序和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = nums[0];
        let mut max = nums[0];
        for n in nums.iter().skip(1) {
            if sum + n < *n {
                sum = *n;
            } else {
                sum += n;
            }
            max = max.max(sum);
        }
        max
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $a:expr) => {
            assert_eq!(
                Solution::max_sub_array(vec!$v),
                $a
            );
        }
    };
    test!([-2, 1, -3, 4, -1, 2, 1, -5, 4], 6);
    test!([1], 1);
    test!([-1], -1);
    test!([-2, 1], 1);
}
