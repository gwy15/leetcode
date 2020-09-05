/*
 * @lc app=leetcode.cn id=1254 lang=rust
 *
 * [1254] 统计封闭岛屿的数目
 */
struct Solution;
// @lc code=start
impl Solution {
    fn dfs_color(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, color: i32) {
        let (m, n) = (grid.len(), grid[0].len());
        let mut stack = Vec::new();
        macro_rules! check {
            ($cond:expr, $i:expr, $j:expr) => {
                if $cond && grid[$i][$j] == 0 {
                    grid[$i][$j] = color;
                    stack.push(($i, $j));
                }
            };
        }
        check!(true, i, j);
        while stack.len() > 0 {
            let (i, j) = stack.pop().unwrap();
            check!(i > 0, i - 1, j);
            check!(j > 0, i, j - 1);
            check!(i + 1 < m, i + 1, j);
            check!(j + 1 < n, i, j + 1);
        }
    }
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        // color all borders first (to -1)
        for i in 0..m {
            for &j in &[0, n - 1] {
                if grid[i][j] == 0 {
                    Self::dfs_color(&mut grid, i, j, -1);
                }
            }
        }
        for j in 1..n - 1 {
            for &i in &[0, m - 1] {
                if grid[i][j] == 0 {
                    Self::dfs_color(&mut grid, i, j, -1);
                }
            }
        }
        // count all others
        let mut cnt = 0;
        for i in 1..m - 1 {
            for j in 1..n - 1 {
                if grid[i][j] == 0 {
                    cnt += 1;
                    Self::dfs_color(&mut grid, i, j, cnt + 1);
                }
            }
        }

        cnt
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(
                Solution::closed_island(vec2d!$args),
                $ans
            )
        };
    }
    test!(
        [
            [1, 1, 1, 1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0, 1, 1, 0],
            [1, 0, 1, 0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0, 1, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 0]
        ],
        2
    );
    test!([[0, 0, 1, 0, 0], [0, 1, 0, 1, 0], [0, 1, 1, 1, 0]], 1);
    test!(
        [
            [1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0, 1],
            [1, 0, 1, 1, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1]
        ],
        2
    );
    test!([[1]], 0);
    test!([[1, 0]], 0);
}
