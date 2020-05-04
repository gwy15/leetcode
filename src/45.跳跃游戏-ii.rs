/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut steps = 0;
        let mut left = 0;
        let mut right = 0;
        // [left, right]
        while right < n - 1 {
            steps += 1;
            let best_dist = left
                + nums[left..=right]
                    .iter()
                    .enumerate()
                    .map(|(j, n)| j + *n as usize)
                    .max()
                    .unwrap();
            left = right + 1;
            right = best_dist;
        }
        steps
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $a:expr) => {
            assert_eq!(
                Solution::jump(vec!$v),
                $a
            );
        }
    };
    test!([2, 3, 1, 1, 4], 2);
    test!([1], 0);
    test!([1, 1], 1);
    test!([0], 0);
}
