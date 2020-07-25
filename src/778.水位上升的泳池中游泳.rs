/*
 * @lc app=leetcode.cn id=778 lang=rust
 *
 * [778] 水位上升的泳池中游泳
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    fn reachable(grid: &Vec<Vec<i32>>, t: i32) -> bool {
        let n = grid.len();
        let mut seen = vec![vec![false; n]; n];

        let mut q = VecDeque::new();
        q.push_back((0, 0));
        seen[0][0] = true;
        while q.len() > 0 {
            let (x, y) = q.pop_front().unwrap();
            if x == n - 1 && y == n - 1 {
                return true;
            }
            macro_rules! check {
                ($x:expr, $y:expr, $cond:expr) => {
                    if $cond && !seen[$x][$y] && grid[$x][$y] <= t {
                        q.push_back(($x, $y));
                        seen[$x][$y] = true;
                    }
                };
            }
            // right
            check!(x + 1, y, x + 1 < n);
            check!(x, y + 1, y + 1 < n);
            check!(x - 1, y, x > 0);
            check!(x, y - 1, y > 0);
        }

        false
    }

    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        // bisect
        // [left, right]
        let (mut left, mut right) = (grid[0][0], n * n);
        while left < right {
            let mid = (left + right) / 2;
            let reachable = Self::reachable(&grid, mid);
            eprintln!("t={}, ok={}", mid, reachable);
            if reachable {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
// @lc code=end
mod utils;

#[test]
fn test_solution() {
    macro_rules! test {
        ($grid:tt, $ans:expr) => {
            assert_eq!(
                Solution::swim_in_water(vec2d!$grid),
                $ans
            );
        }
    };
    test!([[0, 2], [1, 3]], 3);
    test!(
        [
            [0, 1, 2, 3, 4],
            [24, 23, 22, 21, 5],
            [12, 13, 14, 15, 16],
            [11, 17, 18, 19, 20],
            [10, 9, 8, 7, 6]
        ],
        16
    );
    test!([[3, 2], [0, 1]], 3);
}
