/*
 * @lc app=leetcode.cn id=1447 lang=rust
 *
 * [1447] 最简分数
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        fn f(a: i32, b: i32) -> String {
            format!("{}/{}", a, b)
        }
        /// greatest common divisor, a < b
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while a != 0 {
                let t = a;
                a = b % a;
                b = t;
            }
            return b;
        }

        let mut ans = vec![];
        for b in 2..=n {
            for a in 1..b {
                if gcd(a, b) == 1 {
                    ans.push(f(a, b));
                }
            }
        }
        ans
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    macro_rules! test {
        ($n:expr, $ans:tt) => {
            let ans = vec_string!$ans;
            let ans = HashSet::<String>::from_iter(ans.into_iter());
            assert_eq!(
                HashSet::from_iter(Solution::simplified_fractions($n).into_iter()),
                ans
            )
        };
    }
    test!(1, []);
    test!(2, ["1/2"]);
    test!(3, ["1/2", "1/3", "2/3"]);
    test!(4, ["1/2", "1/3", "1/4", "2/3", "3/4"]);
}
