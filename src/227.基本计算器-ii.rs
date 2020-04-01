/*
 * @lc app=leetcode.cn id=227 lang=rust
 *
 * [227] 基本计算器 II
 */

struct Solution;

// @lc code=start

type Stack = Vec<i32>;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut iter = s.chars().peekable();
        let mut stack = Vec::new();
        let mut arg = 0;
        let mut last_op = '+';

        loop {
            let item = iter.next();
            if item.is_none() {
                break;
            }
            let token = item.unwrap();
            match token {
                '0'..='9' => {
                    arg = 10 * arg + (token as i32 - '0' as i32);
                    let next = iter.peek();
                    if next.is_none() || *next.unwrap() < '0' || '9' < *next.unwrap() {
                        let result = match last_op {
                            '+' => arg,
                            '-' => -arg,
                            '*' => stack.pop().unwrap() * arg,
                            '/' => stack.pop().unwrap() / arg,
                            _ => unreachable!(),
                        };
                        stack.push(result);
                        arg = 0;
                    }
                }
                '+' | '-' | '*' | '/' => last_op = token,
                ' ' => {}
                _ => unreachable!(),
            }
        }
        // eval stack
        stack.iter().fold(0, |a, b| a + b)
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr) => {
            assert_eq!(Solution::calculate(stringify!($s).to_owned()), $s);
        };
    };
    test!(1);
    test!(3 + 2 * 2);
    test!(3 / 2);
    test!(3 + 5 / 2);
    test!(2 - 1 / 2);
    test!(1 - 1 + 1);
}
