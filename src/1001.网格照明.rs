/*
 * @lc app=leetcode.cn id=1001 lang=rust
 *
 * [1001] 网格照明
 */
struct Solution;
// @lc code=start
use std::collections::{HashMap, HashSet};
static DELTAS: [(i32, i32); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
impl Solution {
    #[allow(unused)]
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut counter_row = HashMap::<i32, usize>::new();
        let mut counter_col = HashMap::<i32, usize>::new();
        let mut counter_diag = HashMap::<i32, usize>::new();
        let mut counter_rdiag = HashMap::<i32, usize>::new();
        let mut lamp_pos = HashSet::new();
        for lamp in lamps {
            let (x, y) = (lamp[0], lamp[1]);
            *counter_row.entry(x).or_default() += 1;
            *counter_col.entry(y).or_default() += 1;
            *counter_diag.entry(x + y).or_default() += 1;
            *counter_rdiag.entry(x - y + (n - 1)).or_default() += 1;
            lamp_pos.insert((x, y));
        }
        let mut answer = vec![];

        for query in queries {
            let (x, y) = (query[0], query[1]);
            let lightened = counter_row.get(&x).is_some()
                || counter_col.get(&y).is_some()
                || counter_diag.get(&(x + y)).is_some()
                || counter_rdiag.get(&(x - y + (n - 1))).is_some();
            if !lightened {
                answer.push(0);
                continue;
            }
            // lightened
            answer.push(1);
            // turn off neighbor lights
            macro_rules! close_light {
                ($counter:expr, $index:expr) => {
                    if $counter[$index] == 1 {
                        $counter.remove($index);
                    } else {
                        *$counter.get_mut($index).unwrap() -= 1;
                    }
                };
            }
            for delta in DELTAS.iter() {
                let (x, y) = (x + delta.0, y + delta.1);
                if 0 <= x && x < n && 0 <= y && y < n {
                    if lamp_pos.contains(&(x, y)) {
                        // close (x, y)
                        lamp_pos.remove(&(x, y));
                        // remove counters
                        close_light!(counter_row, &x);
                        close_light!(counter_col, &y);
                        close_light!(counter_diag, &(x + y));
                        close_light!(counter_rdiag, &(x - y + (n - 1)));
                    }
                }
            }
        }

        answer
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $lamps:tt, $queries:tt, $ans:tt) => {
            assert_eq!(
                Solution::grid_illumination($n, vec2d!$lamps, vec2d!$queries),
                vec!$ans
            )
        };
    }
    test!(5, [[0, 0], [4, 4]], [[1, 1], [1, 0]], [1, 0]);
    test!(5, [[0, 0], [0, 4]], [[0, 4], [0, 1], [1, 4]], [1, 1, 0]);
}
