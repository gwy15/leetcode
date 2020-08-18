/*
 * @lc app=leetcode.cn id=78 lang=rust
 *
 * [78] 子集
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut ret = vec![];
        for i in 0..(1 << n) {
            let mut sub = vec![];
            for j in 0..n {
                if (i >> j) & 1 == 1 {
                    sub.push(nums[j]);
                }
            }
            ret.push(sub);
        }
        ret
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:tt) => {
            assert_eq!(
                Solution::subsets(vec!$args),
                vec2d!$ans
            )
        };
    }
    test!(
        [1, 2, 3],
        [[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]]
    );
}
