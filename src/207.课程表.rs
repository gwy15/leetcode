/*
 * @lc app=leetcode.cn id=207 lang=rust
 *
 * [207] 课程表
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
use std::iter::FromIterator;
impl Solution {
    pub fn can_finish(n: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        // form graph
        let mut next = vec![Vec::new(); n];
        let mut in_degree = vec![0; n];
        for edge in prerequisites.iter() {
            // (target <- course)
            next[edge[1] as usize].push(edge[0] as usize);
            in_degree[edge[0] as usize] += 1;
        }
        let mut q = VecDeque::from_iter((0..n).filter(|&i| in_degree[i] == 0));
        let mut finished = 0;
        while q.len() > 0 {
            let course = q.pop_front().unwrap();
            // mark course as finished
            finished += 1;
            // mark prerequisites as finished
            for &target in next[course].iter() {
                in_degree[target] -= 1;
                if in_degree[target] == 0 {
                    q.push_back(target);
                }
            }
        }

        finished == n
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $p:tt, $ans:expr) => {
            assert_eq!(
                Solution::can_finish($n, vec2d!$p),
                $ans
            );
        }
    };
    test!(2, [[1, 0]], true);
    test!(2, [[1, 0], [0, 1]], false);
}
