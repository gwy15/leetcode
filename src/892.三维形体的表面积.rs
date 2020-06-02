/* 和为s的连续正数序列
 * @lc app=leetcode.cn id=892 lang=rust
 *
 * [892] 三维形体的表面积
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                // 顶部
                if grid[i][j] != 0 {
                    ans += 1;
                }
                // 左右
                ans += grid[i][j];
                // 减去重叠部分
                if j + 1 < n {
                    ans -= grid[i][j].min(grid[i][j + 1]);
                }
            }
        }
        for j in 0..n {
            for i in 0..m {
                ans += grid[i][j];
                if i + 1 < m {
                    ans -= grid[i][j].min(grid[i + 1][j]);
                }
            }
        }

        2 * ans
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($g:tt, $ans:expr) => {
            assert_eq!(
                Solution::surface_area(vec2d!$g),
                $ans
            );
        }
    };
    test!([[2]], 10);
    test!([[1, 2], [3, 4]], 34);
    test!([[1, 0], [0, 2]], 16);
    test!([[1, 1, 1], [1, 0, 1], [1, 1, 1]], 32);
    test!([[2, 2, 2], [2, 1, 2], [2, 2, 2]], 46);
}
