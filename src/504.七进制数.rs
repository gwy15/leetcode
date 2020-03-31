/*
 * @lc app=leetcode.cn id=504 lang=rust
 *
 * [504] 七进制数
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        if num == 0 {
            return "0".to_owned();
        }
        let neg = num < 0;
        if neg {
            num = -num;
        }
        let mut digits = Vec::new();
        while num != 0 {
            digits.push(('0' as u8 + (num % 7) as u8) as char);
            num /= 7;
        }
        if neg {
            digits.push('-');
        }
        digits.into_iter().rev().collect()
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $s:expr) => {
            assert_eq!(Solution::convert_to_base7($n), $s.to_owned());
        };
    };
    test!(0, "0");
    test!(-1, "-1");
    test!(100, "202");
    test!(-7, "-10");
    test!(49, "100");
}
