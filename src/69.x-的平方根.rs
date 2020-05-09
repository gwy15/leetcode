/*
 * @lc app=leetcode.cn id=69 lang=rust
 *
 * [69] x 的平方根
 */
struct Solution;
// @lc code=start
impl Solution {
    /// x >= 25
    fn _sqrt(x: i32) -> i32 {
        // [left, right)
        let (mut left, mut right) = (5, x / 4);
        while left < right - 1 {
            let mid = left + (right - left) / 2;
            let (mid_squared, overflowed) = mid.overflowing_mul(mid);
            if !overflowed && mid_squared <= x {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }
    pub fn my_sqrt(x: i32) -> i32 {
        match x {
            0 => 0,
            1..=3 => 1,
            4..=8 => 2,
            9..=15 => 3,
            16..=24 => 4,
            _ => Self::_sqrt(x),
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($x:expr, $a:expr) => {
            assert_eq!(Solution::my_sqrt($x), $a);
        };
    };
    for i in 0..1000 {
        let ans = (i as f64).sqrt() as i32;
        test!(i, ans);
    }
    test!(2147395599, 46339);
}
