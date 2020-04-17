/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] 跳跃游戏
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut R = 0;
        let mut i = 0;
        while i < n && i <= R {
            R = R.max(i + (nums[i] as usize));
            i += 1;
        }
        R >= n - 1
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $a:expr) => {
            assert_eq!(
                Solution::can_jump(vec!$n),
                $a
            );
        }
    };
    test!([2, 3, 1, 1, 4], true);
    test!([3, 2, 1, 0, 4], false);
}
