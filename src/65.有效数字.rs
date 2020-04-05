/*
 * @lc app=leetcode.cn id=65 lang=rust
 *
 * [65] 有效数字
 */
struct Solution;
// @lc code=start
enum State {
    Begin,
    LeadingSign,
    LeadingPoint,
    LegitInteger,
    LegitFloat,
    SciFormESign,
    SciFormExpLeadingSign,
    LegitSciForm,
    End,
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state = State::Begin;
        for ch in s.chars() {
            state = match state {
                State::Begin => match ch {
                    '-' | '+' => State::LeadingSign,
                    '.' => State::LeadingPoint,
                    '0'..='9' => State::LegitInteger,
                    ' ' => State::Begin,
                    _ => return false,
                },
                State::LeadingSign => match ch {
                    '.' => State::LeadingPoint,
                    '0'..='9' => State::LegitInteger,
                    _ => return false,
                },
                State::LeadingPoint => match ch {
                    '0'..='9' => State::LegitFloat,
                    _ => return false,
                },
                State::LegitInteger => match ch {
                    '0'..='9' => State::LegitInteger,
                    '.' => State::LegitFloat,
                    'e' => State::SciFormESign,
                    ' ' => State::End,
                    _ => return false,
                },
                State::LegitFloat => match ch {
                    '0'..='9' => State::LegitFloat,
                    'e' => State::SciFormESign,
                    ' ' => State::End,
                    _ => return false,
                },
                State::SciFormESign => match ch {
                    '-' | '+' => State::SciFormExpLeadingSign,
                    '0'..='9' => State::LegitSciForm,
                    _ => return false,
                },
                State::SciFormExpLeadingSign => match ch {
                    '0'..='9' => State::LegitSciForm,
                    _ => return false,
                },
                State::LegitSciForm => match ch {
                    '0'..='9' => State::LegitSciForm,
                    ' ' => State::End,
                    _ => return false,
                },
                State::End => match ch {
                    ' ' => State::End,
                    _ => return false,
                },
            };
        }
        match state {
            State::LegitFloat | State::LegitInteger | State::LegitSciForm | State::End => true,
            _ => false,
        }
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $ans:expr) => {
            assert_eq!(Solution::is_number($s.to_owned()), $ans);
        };
    };
    test!("0", true);
    test!(" 0.1 ", true);
    test!("abc", false);
    test!("1 a", false);
    test!("2e10", true);
    test!(" -90e3   ", true);
    test!(" 1e", false);
    test!("e3", false);
    test!("6e-1", true);
    test!(" 99e2.5  ", false);
    test!("55.5e-93", true);
    test!(" --6", false);
    test!("-+3", false);
    test!("9.e-3", true);
    test!(".9e3", true);
    test!(".9", true);
    test!("9.", true);
    test!("-e3", false);
    test!("e3", false);
    test!("-.9", true);
    test!("+.8e+3", true);
}
