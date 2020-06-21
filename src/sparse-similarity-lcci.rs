struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn compute_similarities(docs: Vec<Vec<i32>>) -> Vec<String> {
        let docs: Vec<HashSet<i32>> = docs.into_iter().map(|v| HashSet::from_iter(v)).collect();
        let mut ans = Vec::new();
        for i in 0..docs.len() {
            if docs[i].len() == 0 {
                continue;
            }
            for j in i + 1..docs.len() {
                if docs[j].len() == 0 {
                    continue;
                }
                let intersec = docs[i].intersection(&docs[j]).count();
                if intersec == 0 {
                    continue;
                }
                let union = docs[i].len() + docs[j].len() - intersec;
                ans.push(format!(
                    "{},{}: {:.4}",
                    i,
                    j,
                    intersec as f64 / union as f64 + 1e-9
                ));
            }
        }
        ans
    }
}

mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $ans:tt) => {
            assert_eq!(
                Solution::compute_similarities(vec2d!$v),
                vec!$ans
            );
        }
    };
    test!(
        [
            [14, 15, 100, 9, 3],
            [32, 1, 9, 3, 5],
            [15, 29, 2, 6, 8, 7],
            [7, 10]
        ],
        ["0,1: 0.2500", "0,2: 0.1000", "2,3: 0.1429"]
    );
}
