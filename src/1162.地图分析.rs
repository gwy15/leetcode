/*
 * @lc app=leetcode.cn id=1162 lang=rust
 *
 * [1162] 地图分析
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut q = VecDeque::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &item) in row.iter().enumerate() {
                if item == 1 {
                    q.push_back((i, j));
                }
            }
        }

        let (mut dis, mut max_dis) = (1, -1);

        macro_rules! check {
            ($c:expr, $i:expr, $j:expr) => {
                if $c && grid[$i][$j] == 0 {
                    // mark as read
                    grid[$i][$j] = dis + 1;
                    max_dis = dis;
                    q.push_back(($i, $j));
                }
            };
        }

        while q.len() > 0 {
            let q_size = q.len();
            for _ in 0..q_size {
                let (i, j) = q.pop_front().unwrap();
                // check neighbors
                check!(i > 0, i - 1, j);
                check!(i < n - 1, i + 1, j);
                check!(j > 0, i, j - 1);
                check!(j < n - 1, i, j + 1);
            }
            dis += 1;
        }

        max_dis
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($map:tt, $ans:expr) => {
            assert_eq!(
                Solution::max_distance(vec2d!$map),
                $ans
            );
        }
    };
    test!([[1, 0, 1], [0, 0, 0], [1, 0, 1]], 2);
    test!([[1, 0, 0], [0, 0, 0], [0, 0, 0]], 4);
    test!([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]], -1);
    test!([[1, 1], [0, 0]], 1);
}
