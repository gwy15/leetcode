/*
 * @lc app=leetcode.cn id=1488 lang=rust
 *
 * [1488] 避免洪水泛滥
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        use std::collections::{BTreeSet, HashMap};

        type DayT = usize;
        type PoolT = i32;

        let mut available_days = BTreeSet::<DayT>::new();
        let mut last_day_rain = HashMap::<PoolT, DayT>::new();
        let mut ans = Vec::with_capacity(rains.len());
        for _ in 0..rains.len() {
            ans.push(-1);
        }
        for (t, &idx) in rains.iter().enumerate() {
            if idx == 0 {
                // sunny day
                available_days.insert(t);
            } else {
                // rainy day
                if let Some(t) = last_day_rain.get(&idx) {
                    match available_days.range(t..).next() {
                        None => {
                            // no available day found
                            return vec![];
                        }
                        Some(&drain_day) => {
                            // drain
                            ans[drain_day] = idx;
                            available_days.remove(&drain_day);
                        }
                    }
                }
                last_day_rain.insert(idx, t);
            }
        }
        for t in available_days {
            ans[t] = 1;
        }

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($r:tt, $ans:tt) => {
            assert_eq!(
                Solution::avoid_flood(vec!$r),
                vec!$ans
            )
        };
    }
    test!([1, 2, 3, 4], [-1, -1, -1, -1]);
    test!([1, 2, 0, 0, 2, 1], [-1, -1, 2, 1, -1, -1]);
    test!([1, 2, 0, 1, 2], []);
    test!([69, 0, 0, 0, 69], [-1, 69, 1, 1, -1]);
    test!([10, 20, 20], []);
}
