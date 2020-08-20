/*
 * @lc app=leetcode.cn id=991 lang=rust
 *
 * [991] 坏了的计算器
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn broken_calc(x: i32, mut y: i32) -> i32 {
        let mut cnt = 0;
        while y > x {
            if y & 1 == 1 {
                y += 1;
                cnt += 1;
            } else {
                y /= 2;
                cnt += 1;
            }
        }
        cnt + x - y
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($x:expr, $y:expr, $ans:expr) => {
            assert_eq!(Solution::broken_calc($x, $y), $ans)
        };
    }
    test!(2, 3, 2);
    test!(5, 8, 2);
    test!(3, 10, 3);
    test!(1024, 1, 1023);
}
