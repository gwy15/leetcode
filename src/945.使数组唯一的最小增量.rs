/*
 * @lc app=leetcode.cn id=945 lang=rust
 *
 * [945] 使数组唯一的最小增量
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn min_increment_for_unique(mut a: Vec<i32>) -> i32 {
        a.sort_unstable();
        let (mut ans, mut cur_max) = (0, -1);
        for &n in a.iter() {
            if n > cur_max {
                cur_max = n;
                continue;
            } else {
                // set n as cur_max+1
                cur_max += 1;
                ans += cur_max - n;
            }
        }

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:expr) => {
            assert_eq!(
                Solution::min_increment_for_unique(vec!$n),
                $ans
            );
        }
    };
    test!([1, 2, 2], 1);
    test!([3, 2, 1, 2, 1, 7], 6);
}
