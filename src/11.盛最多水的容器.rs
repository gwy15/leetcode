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
        let mut ans = 0;
        while l < r {
            let h_l = height[l];
            let h_r = height[r];
            ans = ans.max(h_l.min(h_r) * (r - l) as i32);
            // 移动小的那个
            if h_l < h_r {
                while l < r && height[l] <= h_l {
                    l += 1;
                }
            } else {
                while l < r && height[r] <= h_r {
                    r -= 1;
                }
            }
        }
        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(
                Solution::max_area(vec!$args),
                $ans
            )
        };
    }
    test!([1, 8, 6, 2, 5, 4, 8, 3, 7], 49);
    test!([1, 1], 1);
    test!([4, 3, 2, 1, 4], 16);
    test!([1, 2, 1], 2);
}
