/*
 * @lc app=leetcode.cn id=757 lang=rust
 *
 * [757]  设置交集大小至少为2
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        // sort by end
        intervals.sort_by_key(|v| v[1]);
        let mut times = vec![2; n];
        let mut ans = 0;
        // iterate
        for i in 0..n {
            if times[i] == 0 {
                continue;
            }
            let todo = times[i];
            //
            ans += todo;
            for p in 0..todo {
                let point = intervals[i][1] - p;
                for j in i..n {
                    if times[j] > 0 && intervals[j][0] <= point {
                        times[j] -= 1;
                    }
                }
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
        ($seg:tt, $ans:expr) => {
            assert_eq!(
                Solution::intersection_size_two(vec2d!$seg),
                $ans
            );
        }
    };
    test!([[1, 3], [1, 4], [2, 5], [3, 5]], 3);
    test!([[1, 2], [2, 3], [2, 4], [4, 5]], 5);
}
