/*
 * @lc app=leetcode.cn id=287 lang=rust
 *
 * [287] 寻找重复数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut slow, mut fast) = (0, 0);
        while slow == 0 || slow != fast {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
        }
        slow = 0;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }
        slow as i32
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $ans:expr) => {
            assert_eq!(
                Solution::find_duplicate(vec!$v),
                $ans
            );
        }
    };
    test!([1, 2, 3, 4, 4], 4);
    test!([1, 3, 4, 2, 2], 2);
    test!([1, 1], 1);
    test!([3, 1, 3, 4, 2], 3);
}
