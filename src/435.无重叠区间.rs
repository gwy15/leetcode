/*
 * @lc app=leetcode.cn id=435 lang=rust
 *
 * [435] 无重叠区间
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() == 0 {
            return 0;
        }
        let mut cnt = 0;
        intervals.sort_unstable_by_key(|r| r[0]);

        let mut last_right = intervals[0][1];
        for i in 1..intervals.len() {
            // compare with last
            let this = &intervals[i];
            let (left, right) = (this[0], this[1]);
            if last_right <= left {
                // [   ]
                //     [   ]
                last_right = right;
                continue;
            }
            cnt += 1;
            last_right = last_right.min(right);
        }

        cnt
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:expr) => {
            assert_eq!(
                Solution::erase_overlap_intervals(vec2d!$n),
                $ans
            );
        }
    };
    test!([[1, 2], [2, 3], [3, 4], [1, 3]], 1);
    test!([[1, 2], [1, 2], [1, 2]], 2);
    test!([[1, 2], [2, 3]], 0);
    test!([], 0);
    test!([[1, 2]], 0);
}
