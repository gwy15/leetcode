/*
 * @lc app=leetcode.cn id=1284 lang=rust
 *
 * [1284] 转化为全零矩阵的最少反转次数
 */
struct Solution;
// @lc code=start

type State = usize;

impl Solution {
    fn mat_to_state(mat: &Vec<Vec<i32>>) -> State {
        let m = mat.len();
        let n = mat[0].len();
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                res |= mat[i][j] << (i * n + j);
            }
        }
        res as State
    }
    fn flipping_pos(m: usize, n: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut pos = vec![(i, j)];
        if i > 0 {
            pos.push((i - 1, j));
        }
        if j > 0 {
            pos.push((i, j - 1));
        }
        if i < m - 1 {
            pos.push((i + 1, j));
        }
        if j < n - 1 {
            pos.push((i, j + 1))
        }
        pos
    }
    pub fn neighbor_states(m: usize, n: usize, state: State) -> Vec<State> {
        let mut result = Vec::new();
        // trip to flip every bit
        for i in 0..m {
            for j in 0..n {
                let mut neighbor = state;
                for (fi, fj) in Solution::flipping_pos(m, n, i, j) {
                    neighbor ^= 1 << (fi * n + fj);
                }
                result.push(neighbor);
            }
        }
        result
    }
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut min_steps = vec![-1; 2_usize.pow((m * n) as u32)];

        // run BFS
        let mut queue = std::collections::VecDeque::<State>::new();
        queue.push_back(0);
        let mut steps = 0;
        while queue.len() > 0 {
            let qsize = queue.len();
            for _ in 0..qsize {
                let state = queue.pop_front().unwrap();
                if min_steps[state] == -1 {
                    min_steps[state] = steps;
                    for next_state in Solution::neighbor_states(m, n, state) {
                        if min_steps[next_state] == -1 {
                            queue.push_back(next_state);
                        }
                    }
                }
            }
            steps += 1;
        }

        min_steps[Solution::mat_to_state(&mat)]
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mat_to_state() {
        // notice order
        assert_eq!(
            Solution::mat_to_state(&vec![vec![1, 0], vec![1, 0]]),
            0b0101
        );
        assert_eq!(
            Solution::mat_to_state(&vec![vec![1, 0], vec![1, 0], vec![0, 1]]),
            0b100101
        );
    }

    #[test]
    fn test_neighbor_state() {
        assert_eq!(
            Solution::neighbor_states(2, 2, 0b0000),
            vec![0b0111, 0b1011, 0b1101, 0b1110]
        );
    }

    #[test]
    fn test_flipping_pos() {
        assert_eq!(
            Solution::flipping_pos(3, 3, 1, 0),
            vec![(1, 0), (0, 0), (2, 0), (1, 1)]
        );
        assert_eq!(
            Solution::flipping_pos(2, 2, 0, 0),
            vec![(0, 0), (1, 0), (0, 1)]
        );
    }

    #[test]
    fn test_solution() {
        assert_eq!(Solution::min_flips(vec![vec![0, 0], vec![0, 1]]), 3);
        // assert_eq!(Solution::min_flips(vec![vec![0]]), 0);
        // assert_eq!(
        //     Solution::min_flips(vec![vec![1, 1, 1], vec![1, 0, 1], vec![0, 0, 0]]),
        //     6
        // );
        // assert_eq!(Solution::min_flips(vec![vec![1, 0, 0], vec![1, 0, 0]]), -1);
    }
}
