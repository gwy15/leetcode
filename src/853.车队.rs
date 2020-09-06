/*
 * @lc app=leetcode.cn id=853 lang=rust
 *
 * [853] 车队
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        use std::cmp::Reverse;
        let n = position.len();
        // sort by position from near to far
        let mut pairs: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();

        pairs.sort_unstable_by_key(|&(p, v)| Reverse(p));
        let reach_times: Vec<f64> = pairs
            .into_iter()
            .map(|(p, v)| (target - p) as f64 / v as f64)
            .collect();

        let mut last = 0.0;
        let mut cnt = 0;
        for t in reach_times {
            if t > last {
                last = t;
                cnt += 1;
            } else {
                // pass
            }
        }

        cnt
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        (target=$target:expr, position=$position:tt, speed=$speed:tt, $ans:expr) => {
            assert_eq!(
                Solution::car_fleet($target, vec!$position, vec!$speed),
                $ans
            )
        };
    }
    test!(
        target = 12,
        position = [10, 8, 0, 5, 3],
        speed = [2, 4, 1, 1, 3],
        3
    );
    test!(target = 10, position = [], speed = [], 0);
    test!(target = 10, position = [1], speed = [1], 1);
}
