/*
 * @lc app=leetcode.cn id=1514 lang=rust
 *
 * [1514] 概率最大的路径
 */
struct Solution;
// @lc code=start
#[derive(PartialEq)]
struct F(f64);
impl Eq for F {}
impl std::cmp::PartialOrd for F {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl std::cmp::Ord for F {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
//
use std::collections::BinaryHeap;
impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let n = n as usize;
        let (start, end) = (start as usize, end as usize);

        let mut next: Vec<Vec<(usize, f64)>> = vec![vec![]; n];
        for (edge, prob) in edges.into_iter().zip(succ_prob) {
            let (from, to) = (edge[0] as usize, edge[1] as usize);
            next[from].push((to, prob));
            next[to].push((from, prob));
        }
        // dijkstra
        let mut dist = vec![0.0; n];
        dist[start as usize] = 1.0;
        let mut q = BinaryHeap::new();
        q.push((F(1.0), start));
        while q.len() > 0 {
            let (F(_), i) = q.pop().unwrap();
            // println!("i = {}, p = {}", i, dist[i]);
            if i == end {
                break;
            }
            for &(next_node, edge_prob) in next[i].iter() {
                // relax
                let next_prob = dist[i] * edge_prob;
                if next_prob > dist[next_node] {
                    // println!("  => next={}", next_node);
                    dist[next_node] = next_prob;
                    q.push((F(next_prob), next_node));
                }
            }
        }

        dist[end]
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $edges:tt, $succ_prob:tt, $start:expr, $end:expr, $ans:expr) => {
            assert_eq_float!(
                Solution::max_probability($n, vec2d!$edges, vec!$succ_prob, $start, $end),
                $ans
            );
            println!();
        };
    }
    // test!(3, [[0, 1], [1, 2], [0, 2]], [0.5, 0.5, 0.2], 0, 2, 0.25);
    // test!(3, [[0, 1], [1, 2], [0, 2]], [0.5, 0.5, 0.3], 0, 2, 0.3);
    // test!(3, [[0, 1]], [0.5], 0, 2, 0.00);
    test!(
        5,
        [[1, 4], [2, 4], [0, 4], [0, 3], [0, 2], [2, 3]],
        [0.37, 0.17, 0.93, 0.23, 0.39, 0.04],
        3,
        4,
        0.2139
    );
}
