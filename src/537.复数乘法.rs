/*
 * @lc app=leetcode.cn id=537 lang=rust
 *
 * [537] 复数乘法
 */
struct Solution;
// @lc code=start
#[allow(unused)]
impl Solution {
    fn parse_string(s: String) -> (i32, i32) {
        let pos = s.find('+').unwrap();
        let real = s[..pos].parse().unwrap();
        let complex = s[pos + 1..s.len() - 1].parse().unwrap();

        (real, complex)
    }

    pub fn complex_number_multiply(a: String, b: String) -> String {
        let (a1, b1) = Self::parse_string(a);
        let (a2, b2) = Self::parse_string(b);
        let (a, b) = (a1 * a2 - b1 * b2, a1 * b2 + a2 * b1);
        format!("{:0}+{:0}i", a, b)
    }
}
// @lc code=end
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        macro_rules! test {
            ($s:expr, $ans:expr) => {
                assert_eq!(Solution::parse_string($s.into()), $ans);
            };
        };
        test!("1+1i", (1, 1));
        test!("1+-1i", (1, -1));
        test!("0+-2i", (0, -2));
    }

    #[test]
    fn test_solution() {
        macro_rules! test {
            ($a:expr, $b:expr, $ans:expr) => {
                assert_eq!(
                    Solution::complex_number_multiply($a.into(), $b.into()),
                    $ans
                );
            };
        };
        test!("1+1i", "1+1i", "0+2i");
        test!("1+-1i", "1+-1i", "0+-2i");
    }
}
