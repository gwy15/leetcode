/*
 * @lc app=leetcode.cn id=202 lang=rust
 *
 * [202] 快乐数
 */
struct Solution;
// @lc code=start
impl Solution {
    fn next(mut n: i32) -> i32 {
        let mut s = 0;
        while n > 0 {
            s += (n % 10) * (n % 10);
            n /= 10;
        }
        s
    }
    pub fn is_happy(n: i32) -> bool {
        let mut n1 = n;
        let mut n2 = n;
        loop {
            n1 = Self::next(Self::next(n1));
            n2 = Self::next(n2);
            if n1 == 1 || n2 == 1 {
                return true;
            }
            if n1 == n2 {
                return false;
            }
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $a:expr) => {
            assert_eq!(Solution::is_happy($n), $a);
        };
    };
    test!(19, true);
    test!(1, true);
    test!(0, false);
}
