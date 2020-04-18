/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        let (mut l, mut r) = (0, n - 1);
        let mut max = 0;
        while l < r {
            max = max.max((r - l) as i32 * i32::min(height[l], height[r]));
            if height[l] < height[r] {
                // move l
                l += 1;
            } else {
                r -= 1;
            }
        }
        max
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($t:tt, $a:expr) => {
            assert_eq!(
                Solution::max_area(vec!$t),
                $a
            );
        }
    };
    test!([1, 8, 6, 2, 5, 4, 8, 3, 7], 49);
    test!([1, 2], 1);
}
