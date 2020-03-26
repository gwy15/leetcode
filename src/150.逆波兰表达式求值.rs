/*
 * @lc app=leetcode.cn id=150 lang=rust
 *
 * [150] 逆波兰表达式求值
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::<i32>::new();
        for token in tokens.into_iter() {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    match token.as_str() {
                        "+" => stack.push(a + b),
                        "-" => stack.push(a - b),
                        "*" => stack.push(a * b),
                        "/" => stack.push(a / b),
                        _ => unreachable!(),
                    }
                }
                number => {
                    stack.push(number.parse::<i32>().unwrap());
                }
            }
        }
        stack[0]
    }
}
// @lc code=end

#[test]
fn test_eval_rpn() {
    macro_rules! v {
        ($($s:expr), *) => {
            {
                let mut v = Vec::new();
                $(
                    v.push($s.to_owned());
                )*
                v
            }
        };
    }
    macro_rules! test {
        ($v:expr, $expect:expr) => {
            assert_eq!(Solution::eval_rpn($v), $expect);
        };
    }

    test!(v!["2", "1", "+", "3", "*"], 9);
    test!(
        v!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"],
        22
    );
}
