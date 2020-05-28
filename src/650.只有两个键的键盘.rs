/*
 * @lc app=leetcode.cn id=650 lang=rust
 *
 * [650] 只有两个键的键盘
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn min_steps(mut n: i32) -> i32 {
        let upper = ((n + 1) as f64).sqrt();
        let mut ans = 0;
        // println!("factor {}", n);
        for i in 2..=(upper as i32) {
            while n % i == 0 {
                print!("{} ", i);
                ans += i;
                n /= i;
            }
        }
        // println!("{}", n);
        if n != 1 {
            ans += n;
        }

        ans
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $ans:expr) => {
            assert_eq!(Solution::min_steps($n), $ans);
        };
    };
    test!(3, 3);
    test!(4, 4);
    test!(12, 3 + 2 + 2);
    test!(40, 5 + 2 + 2 + 2);
}
