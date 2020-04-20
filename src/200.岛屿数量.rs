/*
 * @lc app=leetcode.cn id=200 lang=rust
 *
 * [200] 岛屿数量
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    fn mark_island(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
        let (m, n) = (grid.len(), grid[0].len());
        let mut q = VecDeque::new();
        grid[i][j] = '0';
        q.push_back((i, j));
        while q.len() > 0 {
            let (i, j) = q.pop_front().unwrap();
            macro_rules! f {
                ($i:expr, $j:expr, $cond:expr) => {
                    if $cond && grid[$i][$j] == '1' {
                        grid[$i][$j] = '0';
                        q.push_back(($i, $j));
                    }
                };
            }
            f!(i - 1, j, i > 0);
            f!(i + 1, j, i < m - 1);
            f!(i, j - 1, j > 0);
            f!(i, j + 1, j < n - 1);
        }
    }
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut cnt = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    cnt += 1;
                    Solution::mark_island(i, j, &mut grid);
                }
            }
        }

        cnt
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($i:expr, $a:expr) => {
            assert_eq!(Solution::num_islands($i), $a);
        };
    };
    test!(
        vec![vec!['1', '1', '1', '1', '0'], vec!['1', '1', '0', '0', '1']],
        2
    );
}
