/*
 * @lc app=leetcode.cn id=807 lang=rust
 *
 * [807] 保持城市天际线
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut x_max = vec![0; n];
        let mut y_max = vec![0; n];

        grid.iter().enumerate().for_each(|(x, row)| {
            x_max[x] = row.iter().map(|i| *i).max().unwrap();
            for i in 0..n {
                y_max[i] = y_max[i].max(row[i]);
            }
        });

        let mut count = 0;
        for i in 0..n {
            for j in 0..n {
                count += x_max[i].min(y_max[j]) - grid[i][j];
            }
        }

        count
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(
                Solution::max_increase_keeping_skyline(vec2d!$args),
                $ans
            )
        };
    }
    test!([[3, 0, 8, 4], [2, 4, 5, 7], [9, 2, 6, 3], [0, 3, 1, 0]], 35);
    test!([[0, 0, 0], [0, 0, 0], [0, 0, 0]], 0);
}
