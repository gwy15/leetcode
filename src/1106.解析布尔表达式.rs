/*
 * @lc app=leetcode.cn id=1106 lang=rust
 *
 * [1106] 解析布尔表达式
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        Solution::parse_from_index(&expression.chars().collect(), 0).0
    }

    /// Parse expression from [i]
    /// return parsed result and consumed length.
    /// e.g., parse_from_index("&(t,t)", 2) = (true, 1)
    pub fn parse_from_index(expression: &Vec<char>, i: usize) -> (bool, usize) {
        let mut consumed = 0;
        match expression[i] {
            't' => (true, 1),
            'f' => (false, 1),
            '!' => {
                // let left_bracket = expression[i + 1];
                consumed += 2;
                let reversed_result = Solution::parse_from_index(expression, i + 2);
                consumed += reversed_result.1;
                // right bracket
                consumed += 1;
                (!reversed_result.0, consumed)
            }
            '&' | '|' => {
                // let left_bracket = expression[i + 1];
                consumed += 2;
                let mut j = i + 2;
                let mut args = Vec::new();
                loop {
                    if expression[j] == ',' {
                        consumed += 1;
                        j += 1;
                        continue;
                    } else if expression[j] == ')' {
                        consumed += 1;
                        break;
                    }
                    let arg_result = Solution::parse_from_index(expression, j);
                    args.push(arg_result.0);
                    consumed += arg_result.1;
                    j += arg_result.1; // and a comma
                }
                match expression[i] {
                    '&' => (args.iter().all(|&x| x), consumed),
                    '|' => (args.iter().any(|&x| x), consumed),
                    _ => unreachable!(),
                }
            }
            _ => {
                println!("unexpected char: {}", expression[i]);
                unreachable!()
            }
        }
    }
}
// @lc code=end

#[test]
fn test_parse() {
    macro_rules! test {
        ($s:expr, $expect:expr) => {
            assert_eq!(Solution::parse_bool_expr($s.to_owned()), $expect);
        };
    }
    test!("!(f)", true);
    test!("|(f,t)", true);
    test!("&(t,f)", false);
    test!("|(&(t,f,t),!(t))", false);
    test!("|(f,&(t,t))", true);
}
