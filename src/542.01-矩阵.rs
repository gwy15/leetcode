/*
 * @lc app=leetcode.cn id=542 lang=rust
 *
 * [542] 01 矩阵
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut visited = vec![vec![false; n]; m];

        let mut queue = VecDeque::new();
        for (i, row) in matrix.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                if v == 0 {
                    queue.push_back((i, j));
                }
            }
        }
        let mut dist = 0;
        while queue.len() > 0 {
            let qsize = queue.len();
            for _ in 0..qsize {
                let (i, j) = queue.pop_front().unwrap();
                if visited[i][j] {
                    continue;
                }
                visited[i][j] = true;
                matrix[i][j] = dist;
                // push neighbors
                let mut f = |x: usize, y: usize| {
                    if !visited[x][y] {
                        queue.push_back((x, y));
                    }
                };
                if i > 0 {
                    f(i - 1, j);
                }
                if j > 0 {
                    f(i, j - 1);
                }
                if i < m - 1 {
                    f(i + 1, j);
                }
                if j < n - 1 {
                    f(i, j + 1);
                }
            }
            dist += 1;
        }

        matrix
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! vec2 {
        [$(
            [$(
                $i:expr
            ),*]
        ),*] => {
            vec![
                $(
                    vec![$($i),*]
                ),*
            ]
        };
    }
    macro_rules! test {
        ($v:tt, $a:tt) => {
            assert_eq!(Solution::update_matrix($v), $a);
        };
    };
    let v = vec2![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
    let a = vec2![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
    test!(v, a);
    let v = vec2![[0, 0, 0], [0, 1, 0], [1, 1, 1]];
    let a = vec2![[0, 0, 0], [0, 1, 0], [1, 2, 1]];
    test!(v, a);
}
