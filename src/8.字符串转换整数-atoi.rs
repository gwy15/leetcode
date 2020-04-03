/*
 * @lc app=leetcode.cn id=8 lang=rust
 *
 * [8] 字符串转换整数 (atoi)
 */
struct Solution;
// @lc code=start

fn c2i(c: char) -> i32 {
    (c as u8 - '0' as u8) as i32
}

enum Status {
    Waiting,
    Positive,
    Negative,
    End,
}

impl Solution {
    fn mul10(a: i32) -> i32 {
        let (ans, overflowing) = a.overflowing_mul(10);
        match overflowing {
            true => match a > 0 {
                true => i32::max_value(),
                false => i32::min_value(),
            },
            false => ans,
        }
    }

    fn mul10_add(a: i32, b: i32) -> i32 {
        let a = Solution::mul10(a);
        let (ans, o) = a.overflowing_add(b);
        match o {
            true => i32::max_value(),
            false => ans,
        }
    }

    fn mul10_sub(a: i32, b: i32) -> i32 {
        let a = Solution::mul10(a);
        let (ans, o) = a.overflowing_sub(b);
        match o {
            true => i32::min_value(),
            false => ans,
        }
    }

    pub fn my_atoi(s: String) -> i32 {
        let mut n: i32 = 0;
        let mut status = Status::Waiting;
        for ch in s.chars() {
            match &status {
                Status::Waiting => match ch {
                    ' ' => continue,
                    '-' => status = Status::Negative,
                    '+' => status = Status::Positive,
                    '0'..='9' => {
                        status = Status::Positive;
                        n = c2i(ch);
                    }
                    _ => break,
                },
                Status::Positive => match ch {
                    '0'..='9' => {
                        n = Solution::mul10_add(n, c2i(ch));
                        if n == i32::max_value() {
                            break;
                        }
                    }
                    _ => break,
                },
                Status::Negative => match ch {
                    '0'..='9' => {
                        n = Solution::mul10_sub(n, c2i(ch));
                        if n == i32::min_value() {
                            break;
                        }
                    }
                    _ => break,
                },
                Status::End => unreachable!(),
            }
        }
        status = Status::End;
        n
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $n:expr) => {
            assert_eq!(Solution::my_atoi($s.to_owned()), $n);
        };
    };
    test!("42", 42);
    test!("0", 0);
    test!("-0", 0);
    test!("   -42", -42);
    test!("+42", 42);
    test!("4193 with words", 4193);
    test!("words and 987", 0);
    test!("-91283472332", -2147483648);
}
