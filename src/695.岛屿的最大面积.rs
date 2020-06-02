/*
 * @lc app=leetcode.cn id=695 lang=rust
 *
 * [695] 岛屿的最大面积
 */
struct Solution;
// @lc code=start
#[allow(unused)]
impl Solution {
    fn search_island(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut area = 0;

        let mut q = std::collections::VecDeque::new();

        macro_rules! check {
            ($cond:expr, $i:expr, $j:expr) => {
                if $cond && grid[$i][$j] == 1 {
                    grid[$i][$j] = 2;
                    area += 1;
                    q.push_back(($i, $j));
                }
            };
        }

        check!(true, i, j);

        while q.len() > 0 {
            let (i, j) = q.pop_back().unwrap();
            check!(i > 0, i - 1, j);
            check!(j > 0, i, j - 1);
            check!(i < m - 1, i + 1, j);
            check!(j < n - 1, i, j + 1);
        }

        area
    }
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut max_area = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    // search
                    let area = Self::search_island(&mut grid, i, j);
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
                Solution::max_area_of_island(vec2d!$grid),
                $ans
            );
        }
    };
    test!(
        [
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ],
        6
    );
    test!([[0, 0, 0, 0, 0, 0, 0, 0]], 0);
}
