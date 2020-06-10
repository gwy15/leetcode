/*
 * @lc app=leetcode.cn id=803 lang=rust
 *
 * [803] 打砖块
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    fn count(grid: &mut Vec<Vec<i32>>, target: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut q = VecDeque::new();
        let mut cnt = 0;

        macro_rules! check {
            ($cond:expr, $i:expr, $j:expr) => {
                if $cond && grid[$i][$j] == target {
                    grid[$i][$j] = target + 1;
                    q.push_back(($i, $j));
                    cnt += 1;
                }
            };
        }

        for j in 0..n {
            check!(true, 0, j);
        }

        while q.len() > 0 {
            let (i, j) = q.pop_back().unwrap();
            check!(0 < i, i - 1, j);
            check!(0 < j, i, j - 1);
            check!(i < m - 1, i + 1, j);
            check!(j < n - 1, i, j + 1);
        }

        cnt
    }
    pub fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = Vec::new();
        for (times, hit) in hits.into_iter().enumerate() {
            let (i, j) = (hit[0] as usize, hit[1] as usize);
            grid[i][j] = 0;
            //
            let target = 1 + times as i32;
            Self::count(&mut grid, target);

            // now find block with value=target
            let mut cnt = 0;
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == target {
                        cnt += 1;
                    }
                }
            }
            ans.push(cnt);
        }

        ans
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($grid:tt, $hits:tt, $ans:tt) => {
            assert_eq!(
                Solution::hit_bricks(vec2d!$grid, vec2d!$hits),
                vec!$ans
            );
        }
    };
    test!([[1, 0, 0, 0], [1, 1, 1, 0]], [[1, 0]], [2]);
    test!([[1, 0, 0, 0], [1, 1, 0, 0]], [[1, 1], [1, 0]], [0, 0]);
    test!([[1, 0, 1], [1, 1, 1]], [[0, 0], [0, 2], [1, 1]], [0, 3, 0]);
}
