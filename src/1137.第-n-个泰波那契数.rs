/*
 * @lc app=leetcode.cn id=1137 lang=rust
 *
 * [1137] 第 N 个泰波那契数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn tribonacci(m: i32) -> i32 {
        let n = 37;
        let mut res = vec![0; n + 1];
        res[0] = 0;
        res[1] = 1;
        res[2] = 1;
        for i in 3..=n {
            res[i] = res[i - 1] + res[i - 2] + res[i - 3];
            // println!("res[{}] = {}", i, res[i]);
        }
        res[m as usize]
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($i:expr, $ans:expr) => {
            assert_eq!(Solution::tribonacci($i), $ans);
        };
    };
    test!(4, 4);
}
