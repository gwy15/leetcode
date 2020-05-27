/*
 * @lc app=leetcode.cn id=453 lang=rust
 *
 * [453] 最小移动次数使数组元素相等
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        let mut ans = 0;
        for n in nums.iter() {
            ans += n - min;
        }

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:expr) => {
            assert_eq!(
                Solution::min_moves(vec!$n),
                $ans
            );
        }
    };
    test!([1, 2, 3], 3);
}
