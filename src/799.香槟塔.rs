/*
 * @lc app=leetcode.cn id=799 lang=rust
 *
 * [799] 香槟塔
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn champagne_tower(poured: i32, I: i32, query_glass: i32) -> f64 {
        // symmetric
        let J = match query_glass > I / 2 {
            true => I - query_glass,
            false => query_glass,
        } as usize;
        //
        let mut dp = vec![poured as f64];
        for i in 0..I as usize {
            // next level (i+1) has i+2 items
            let mut tmp = vec![0.0; i + 2];
            // overflow to next level
            for j in 0..J.min(i) + 1 {
                if dp[j] > 1.0 {
                    let overflow = (dp[j] - 1.) / 2.;
                    tmp[j] += overflow;
                    if j + 1 <= J {
                        tmp[j + 1] += overflow;
                    }
                }
            }
            // write to dp
            dp = tmp;
        }
        dp[J].min(1.0)
    }
}
// @lc code=end

#[test]
fn test() {
    macro_rules! test {
        (($p:expr, $i:expr, $j:expr), $ans:expr) => {
            assert_eq!(Solution::champagne_tower($p, $i, $j), $ans);
        };
    }
    test!((1, 1, 1), 0.0);
    test!((2, 1, 1), 0.5);
    test!((99, 3, 3), 1.0);
    test!((0, 2, 1), 0.0);
    test!((1, 2, 1), 0.0);
    test!((4, 2, 1), 0.5);
}
