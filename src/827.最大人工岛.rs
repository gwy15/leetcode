/*
 * @lc app=leetcode.cn id=827 lang=rust
 *
 * [827] 最大人工岛
 */
struct Solution;
// @lc code=start
impl Solution {
    fn color(color: i32, grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut stack = Vec::new();
        let mut area = 0;
        macro_rules! check {
            ($cond:expr, $i:expr, $j:expr) => {
                if $cond && grid[$i][$j] == 1 {
                    // mark as visited
                    area += 1;
                    grid[$i][$j] = color;
                    stack.push(($i, $j));
                }
            };
        }
        // dfs
        check!(true, i, j);
        while stack.len() > 0 {
            let (i, j) = stack.pop().unwrap();
            check!(i + 1 < m, i + 1, j);
            check!(j + 1 < n, i, j + 1);
            check!(i > 0, i - 1, j);
            check!(j > 0, i, j - 1);
        }
        area
    }
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        // set grid 1 => [2,...]
        let mut areas = vec![0, 0];
        let (m, n) = (grid.len(), grid[0].len());
        let mut color = 2;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    // dfs search
                    let area = Self::color(color, &mut grid, i, j);
                    areas.push(area);
                    color += 1;
                }
            }
        }
        let mut max_area = *areas.iter().max().unwrap_or(&0);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    let mut visited = vec![];
                    // this grid
                    let mut area = 1;
                    macro_rules! check {
                        ($cond:expr, $i:expr, $j:expr) => {
                            if $cond && grid[$i][$j] != 0 {
                                let c = grid[$i][$j] as usize;
                                if visited.iter().all(|&i| i != c) {
                                    area += areas[c];
                                    visited.push(c);
                                }
                            }
                        };
                    }
                    // visit neighbors
                    check!(i > 0, i - 1, j);
                    check!(j > 0, i, j - 1);
                    check!(i + 1 < m, i + 1, j);
                    check!(j + 1 < n, i, j + 1);

                    max_area = max_area.max(area);
                }
            }
        }

        max_area
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($grid:tt, $ans:expr) => {
            assert_eq!(
                Solution::largest_island(vec2d!$grid),
                $ans
            )
        };
    }
    test!([[1, 0], [0, 1]], 3);
    test!([[1, 1], [1, 0]], 4);
    test!([[1, 1], [1, 1]], 4);
    test!([[0]], 1);
    test!([[1]], 1);
}
