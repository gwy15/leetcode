/*
 * @lc app=leetcode.cn id=399 lang=rust
 *
 * [399] 除法求值
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
type Edge = (String, f64);
type Edges = HashMap<String, Edge>;
#[allow(unused)]
impl Solution {
    fn find(x: &str, edges: &mut Edges) -> Edge {
        let edge = edges.entry(x.into()).or_insert_with(|| (x.into(), 1.0));
        let (parent, times) = (edge.clone());

        if x == parent {
            (parent, times)
        } else {
            let (root, parent_times) = Self::find(&parent, edges);
            let times = parent_times * times;
            edges.insert(x.into(), (root.clone(), times));
            (root, times)
        }
    }

    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let n = equations.len();
        let mut parent: Edges = HashMap::new();
        for i in 0..n {
            let eq = &equations[i];
            let (a, b) = (&eq[0], &eq[1]);
            let k = values[i];
            // a / b = k or a = k b

            // find a, b root
            let (root_a, k_a) = Self::find(a, &mut parent);
            let (root_b, k_b) = Self::find(b, &mut parent);
            if root_a == root_b {
                continue;
            }
            // a = r_a * k_a
            //   = k * b
            //   = k * k_b * root_b
            // root_a = k * k_b / k_a * root_b
            parent.insert(root_a, (root_b, k * k_b / k_a));
        }

        let mut ans = Vec::new();
        for query in queries {
            let (a, b) = (&query[0], &query[1]);
            if !parent.contains_key(a) || !parent.contains_key(b) {
                ans.push(-1.0);
                continue;
            }
            let (root_a, k_a) = Self::find(a, &mut parent);
            let (root_b, k_b) = Self::find(b, &mut parent);
            if root_a != root_b {
                ans.push(-1.0);
            } else {
                // a = k_a root, b = k_b root
                // a / b = k_a / k_b
                ans.push(k_a / k_b);
            }
        }

        ans
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($eq:tt, $v:tt, $q:tt, $ans:tt) => {
            let cvt = |vv: Vec<Vec<&str>>| vv.into_iter().map(|v| v.into_iter().map(|s| s.to_owned()).collect()).collect();
            assert_eq!(
                Solution::calc_equation(cvt(vec2d!$eq), vec!$v, cvt(vec2d!$q)),
                vec!$ans
            );
        }
    };
    test!(
        [["a", "b"], ["b", "c"]],
        [2.0, 3.0],
        [["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]],
        [6.0, 0.5, -1.0, 1.0, -1.0]
    );
}
