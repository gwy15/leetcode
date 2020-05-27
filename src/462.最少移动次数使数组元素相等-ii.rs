/*
 * @lc app=leetcode.cn id=462 lang=rust
 *
 * [462] 最少移动次数使数组元素相等 II
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mid = nums[nums.len() / 2]; // 奇偶统一处理
        nums.iter().fold(0, |sum, &n| sum + (n - mid).abs())
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:expr) => {
            assert_eq!(
                Solution::min_moves2(vec!$n),
                $ans
            );
        }
    };
    test!([1, 2, 3], 2);
    test!([1, 2, 2], 1);
    test!([1, 2], 1);
}
