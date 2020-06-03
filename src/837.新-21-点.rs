/*
 * @lc app=leetcode.cn id=837 lang=rust
 *
 * [837] 新21点
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn new21_game(n: i32, k: i32, w: i32) -> f64 {
        let (n, k, w) = (n as usize, k as usize, w as usize);
        let mut p = vec![0.0; k + w + 1];
        p[0] = 1.0;

        let w_t = 1.0 / w as f64;
        let mut window = if k > 0 { 1.0 } else { 0.0 };
        for i in 1..(k + w) {
            // sum a window of max length W
            p[i] = window * w_t;
            // add p[i] to window
            if i < k {
                window += p[i];
            }
            // drop first from window
            if i >= w {
                window -= p[i - w];
            }
        }

        p[k..=n].iter().sum()
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($nkw:tt, $ans:expr) => {
            assert_eq_float!(Solution::new21_game$nkw, $ans);
        };
    };
    test!((10, 1, 10), 1.0);
    test!((6, 1, 10), 0.6);
    test!((21, 17, 10), 0.73278);
    test!((1, 0, 1), 1.0);
    test!((1, 0, 2), 1.0);
}
