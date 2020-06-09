struct Solution;

impl Solution {
    fn translate(nums: &[i32]) -> i32 {
        match nums.len() {
            0 | 1 => 1,
            _ => match nums[0] {
                1 => Self::translate(&nums[1..]) + Self::translate(&nums[2..]),
                2 => match nums[1] {
                    0..=5 => Self::translate(&nums[1..]) + Self::translate(&nums[2..]),
                    _ => Self::translate(&nums[1..]),
                },
                _ => Self::translate(&nums[1..]),
            },
        }
    }
    pub fn translate_num(mut num: i32) -> i32 {
        let mut digits = Vec::new();
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();
        Self::translate(&digits)
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $ans:expr) => {
            assert_eq!(Solution::translate_num($n), $ans);
        };
    };
    test!(12258, 5);
    test!(0, 1);
    test!(12, 2);
}
