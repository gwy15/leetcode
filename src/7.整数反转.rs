/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut stack = Vec::new();
        let mut ans: i32 = 0;
        if x >= 0 {
            while x > 0 {
                stack.push(x % 10);
                x /= 10;
            }

            for b in stack {
                match ans.checked_mul(10).and_then(|a| a.checked_add(b)) {
                    Some(c) => {
                        ans = c;
                    }
                    None => {
                        return 0;
                    }
                }
            }
        } else {
            while x < 0 {
                let digit = (10 - (x % 10)) % 10;
                stack.push(digit);
                x = (x + digit) / 10;
            }
            for b in stack {
                match ans.checked_mul(10).and_then(|a| a.checked_sub(b)) {
                    Some(c) => {
                        ans = c;
                    }
                    None => {
                        return 0;
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $ans:expr) => {
            assert_eq!(Solution::reverse($n), $ans);
        };
    };
    test!(123, 321);
    test!(0, 0);
    test!(-123, -321);
    test!(120, 21);
    test!(i32::max_value(), 0);
    test!(i32::min_value(), 0);
}
