/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            while 1 <= nums[i] && nums[i] < i as i32 + 1 && nums[i] != nums[nums[i] as usize - 1] {
                let num = nums[i];
                nums.swap(i, num as usize - 1);
            }
        }
        for i in 0..n {
            if nums[i] as usize != i + 1 {
                return i as i32 + 1;
            }
        }
        n as i32 + 1
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $a:expr) => {
            assert_eq!(
                Solution::first_missing_positive(vec!$v),
                $a
            );
        }
    };
    test!([1, 2, 0], 3);
    test!([], 1);
    test!([3, 4, -1, 1], 2);
    test!([2, 1], 3);
    test!(
        [11, 1, 6, 11, 5, 5, -6, 9, -3, 9, 5, 4, 2, -8, 16, -6, -4, 2, 3],
        7
    );
}
