/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 * [62] 不同路径
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut last = vec![0; m];
        let mut row = vec![0; m];
        row[0] = 1;
        for _i in 0..n {
            for j in 1..m {
                row[j] = row[j - 1] + last[j];
            }
            last = row;
            row = vec![0; m];
            row[0] = 1;
        }
        last[m - 1]
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($m:expr, $n:expr, $ans:expr) => {
            assert_eq!(Solution::unique_paths($m, $n), $ans)
        };
    }
    test!(3, 2, 3);
    test!(7, 3, 28);
    test!(1, 1, 1);
}
