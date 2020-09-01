/*
 * @lc app=leetcode.cn id=857 lang=rust
 *
 * [857] 雇佣 K 名工人的最低成本
 */
struct Solution;
// @lc code=start
use std::collections::BinaryHeap;
impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let n = quality.len();
        let mut sorted: Vec<usize> = (0..n).collect();
        sorted.sort_unstable_by(|&lhs, &rhs| {
            // ratio = quality / wage, reversed sort
            let r1 = quality[lhs] as f64 / wage[lhs] as f64;
            let r2 = quality[rhs] as f64 / wage[rhs] as f64;
            r1.partial_cmp(&r2).unwrap()
        });
        // iterate through workers
        let mut best_wage: f64 = 1e20;
        let mut options = BinaryHeap::new();
        for i in sorted.into_iter().rev() {
            let quality = quality[i];
            let wage = wage[i];
            // add to option if less than k options
            options.push(quality);
            if options.len() < k as usize {
                continue;
            } else if options.len() > k as usize {
                options.pop();
            }
            let ratio = quality as f64 / wage as f64;
            // println!("=> workers (quality) = {:?}", options);
            // println!("   ratio = {}", ratio);

            let total_quality: i32 = options.iter().sum();
            let total_wage = total_quality as f64 / ratio;
            best_wage = best_wage.min(total_wage);
        }

        best_wage
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($q:tt, $w:tt, $k:expr, $ans:expr) => {
            assert_eq_float!(
                Solution::mincost_to_hire_workers(vec!$q, vec!$w, $k),
                $ans
            )
        };
    }
    test!([10, 20, 5], [70, 50, 30], 2, 105.0);
    test!([3, 1, 10, 10, 1], [4, 8, 2, 2, 7], 3, 30.666666666667);
    test!([1], [1], 1, 1.0);
}
