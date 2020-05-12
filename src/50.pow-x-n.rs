/*
 * @lc app=leetcode.cn id=50 lang=rust
 *
 * [50] Pow(x, n)
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
        if n < 0 {
            n += 1;
            x = 1.0 / x;
            return x * Self::my_pow(x, -n);
        }
        let mut ans = 1.0;
        while n > 0 {
            if n & 1 == 1 {
                ans *= x;
            }
            x *= x;
            n /= 2;
        }
        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($x:expr, $n:expr) => {
            let x = $x;
            let n = $n;
            assert!((Solution::my_pow(x, n) - x.powf(n.into())).abs() < 0.001);
        };
    }
    test!(2.0, 10);
    test!(2.1, 3);
    test!(2.0, -2);
    test!(1.0000001, 10000000);
    test!(1.0, -2147483648);
}
