/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */
struct Solution;
// @lc code=start
impl Solution {
    fn reverse_pos(mut x: i32) -> i32 {
        let mut ans = 0i32;
        while x > 0 {
            let (_ans, o) = ans.overflowing_mul(10);
            if o {
                return 0;
            }
            let (_ans, o) = _ans.overflowing_add(x % 10);
            if o {
                return 0;
            }
            x /= 10;
            ans = _ans;
        }
        ans
    }
    pub fn reverse(x: i32) -> i32 {
        if x == i32::MIN {
            return 0;
        }
        match x {
            // 防止溢出，-2147483648 一定溢出
            i32::MIN => 0,
            x if x < 0 => -Solution::reverse(-x),
            x => Solution::reverse_pos(x),
        }
    }
}

// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:expr, $ans:expr) => {
            assert_eq!(Solution::reverse($args), $ans)
        };
    }
    test!(123, 321);
    test!(-123, -321);
    test!(0, 0);
    test!(120, 21);
    test!(i32::MIN, 0);
    test!(2147483647, 0);
    test!(1463847413, 0);
    test!(2147483619, 0);
    test!(-1147483647, 0);
}
