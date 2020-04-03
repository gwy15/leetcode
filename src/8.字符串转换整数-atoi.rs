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
    Positive(i32),
    Negative(i32),
    End(i32),
}

impl Solution {
    fn overflowing_mul10(a: i32) -> (i32, bool) {
        match a.overflowing_mul(10) {
            (_, true) => match a > 0 {
                true => (i32::max_value(), true),
                false => (i32::min_value(), true),
            },
            (ans, false) => (ans, false),
        }
    }

    fn overflowing_mul10_add(a: i32, b: i32) -> (i32, bool) {
        match Solution::overflowing_mul10(a) {
            (o, true) => (o, true),
            (a, false) => match a.overflowing_add(b) {
                (_, true) => (i32::max_value(), true),
                (ans, false) => (ans, false),
            },
        }
    }

    fn overflowing_mul10_sub(a: i32, b: i32) -> (i32, bool) {
        match Solution::overflowing_mul10(a) {
            (o, true) => (o, true),
            (a, false) => match a.overflowing_sub(b) {
                (_, true) => (i32::min_value(), true),
                (ans, false) => (ans, false),
            },
        }
    }

    pub fn my_atoi(s: String) -> i32 {
        let mut status = Status::Waiting;
        for ch in s.chars() {
            status = match status {
                Status::Waiting => match ch {
                    ' ' => continue,
                    '-' => Status::Negative(0),
                    '+' => Status::Positive(0),
                    '0'..='9' => Status::Positive(c2i(ch)),
                    _ => Status::End(0),
                },
                Status::Positive(a) => match ch {
                    '0'..='9' => match Solution::overflowing_mul10_add(a, c2i(ch)) {
                        (o, true) => Status::End(o),
                        (ans, false) => Status::Positive(ans),
                    },
                    _ => Status::End(a),
                },
                Status::Negative(a) => match ch {
                    '0'..='9' => match Solution::overflowing_mul10_sub(a, c2i(ch)) {
                        (o, true) => Status::End(o),
                        (ans, false) => Status::Negative(ans),
                    },
                    _ => Status::End(a),
                },
                Status::End(_) => break,
            };
        }
        match status {
            Status::Positive(ans) | Status::Negative(ans) | Status::End(ans) => ans,
            Status::Waiting => 0,
        }
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
    test!("-2147483648", -2147483648);
    test!("  ", 0);
}
