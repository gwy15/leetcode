/*
 * @lc app=leetcode.cn id=210 lang=rust
 *
 * [210] 课程表 II
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut sorted = Vec::new();
        let mut in_rank = vec![0; n];
        let mut next: Vec<Vec<i32>> = vec![Vec::new(); n];
        let mut zero_in_rank_nodes = VecDeque::new();
        // init graph
        for edge in prerequisites.iter() {
            // i <- j
            let (i, j) = (edge[0], edge[1]);
            next[j as usize].push(i);
            in_rank[i as usize] += 1;
        }
        // init zero in-rank nodes
        for i in 0..n {
            if in_rank[i] == 0 {
                zero_in_rank_nodes.push_back(i);
            }
        }
        // sort
        while zero_in_rank_nodes.len() > 0 {
            let i = zero_in_rank_nodes.pop_front().unwrap();
            sorted.push(i as i32);
            // clear in rank
            for &j in &next[i] {
                in_rank[j as usize] -= 1;
                if in_rank[j as usize] == 0 {
                    zero_in_rank_nodes.push_back(j as usize);
                }
            }
        }
        if sorted.len() != n {
            return vec![];
        }
        sorted
    }
}
// @lc code=end
#[test]
fn test() {
    macro_rules! vec2d {
        [$(
            [$(
                $i:expr
            ),*]
        ),*] => {
            vec![$(
                vec![$($i),*]
            ),*]
        }
    }
    macro_rules! test {
        ($n:expr, $pre:tt, $ans:tt) => {
            assert_eq!(
                Solution::find_order($n, vec2d!$pre),
                vec!$ans
            );
        };
    }
    test!(2, [[1, 0]], [0, 1]);
    test!(4, [[1, 0], [2, 0], [3, 1], [3, 2]], [0, 1, 2, 3]);
    // no answer
    test!(3, [[1, 0], [1, 2], [0, 1]], []);
}
