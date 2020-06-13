/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (0, 1);
        for i in 0..n {
            let c = a + b;
            a = b;
            b = c;
        }
        return b;
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $ans:expr) => {
            assert_eq!(Solution::climb_stairs($n), $ans);
        };
    };
    test!(1, 1);
    test!(2, 2);
    test!(3, 3);
    test!(4, 5);
}
