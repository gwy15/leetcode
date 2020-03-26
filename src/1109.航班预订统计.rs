/*
 * @lc app=leetcode.cn id=1109 lang=rust
 *
 * [1109] 航班预订统计
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut delta = vec![0; n as usize];
        for booking in bookings.into_iter() {
            let i = booking[0];
            let j = booking[1];
            let incr = booking[2];
            delta[(i - 1) as usize] += incr;
            if j < n {
                delta[j as usize] -= incr;
            }
        }
        let mut sum = 0;
        delta
            .into_iter()
            .map(|d| {
                sum += d;
                sum
            })
            .collect()
    }
}
// @lc code=end

#[test]
fn test_solution() {
    assert_eq!(
        Solution::corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5),
        vec![10, 55, 45, 25, 25]
    );
}
