/*
 * @lc app=leetcode.cn id=994 lang=rust
 *
 * [994] 腐烂的橘子
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut fresh_orange_count = 0;
        let mut q = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                match grid[i][j] {
                    0 => {}
                    1 => fresh_orange_count += 1,
                    2 => {
                        q.push_back((i, j));
                    }
                    _ => unreachable!(),
                }
            }
        }

        macro_rules! spread {
            ($c:expr, $i:expr, $j:expr) => {
                if $c && grid[$i][$j] == 1 {
                    fresh_orange_count -= 1;
                    grid[$i][$j] = 2;
                    q.push_back(($i, $j));
                }
            };
        }

        let mut t = 0;
        while q.len() > 0 && fresh_orange_count > 0 {
            let q_size = q.len();
            for _ in 0..q_size {
                let (i, j) = q.pop_front().unwrap();
                // 感染周围的橘子
                spread!(i > 0, i - 1, j);
                spread!(i < m - 1, i + 1, j);
                spread!(j > 0, i, j - 1);
                spread!(j < n - 1, i, j + 1);
            }
            t += 1;
        }
        if fresh_orange_count > 0 {
            -1
        } else {
            t
        }
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($grid:tt, $ans:expr) => {
            assert_eq!(
                Solution::oranges_rotting(vec2d!$grid),
                $ans
            );
        }
    };
    test!([[2, 1, 1], [1, 1, 0], [0, 1, 1]], 4);
    test!([[2, 1, 1], [0, 1, 1], [1, 0, 1]], -1);
    test!([[0, 2]], 0);
    test!([[1, 2]], 1);
}
