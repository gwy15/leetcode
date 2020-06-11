/*
 * @lc app=leetcode.cn id=739 lang=rust
 *
 * [739] 每日温度
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = vec![];
        let mut ans = vec![0; t.len()];
        for i in (0..t.len()).rev() {
            let temp = t[i];
            while stack.len() > 0 && temp >= stack.last().unwrap().0 {
                stack.pop();
            }
            //
            match stack.last() {
                None => {
                    stack.push((temp, i));
                }
                Some((_, day)) => {
                    let gap = day - i;
                    ans[i] = gap as i32;
                    stack.push((temp, i));
                }
            }
        }

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($t:tt, $ans:tt) => {
            assert_eq!(Solution::daily_temperatures(vec!$t), vec!$ans);
        };
    };
    test!([73, 74, 75, 71, 69, 72, 76, 73], [1, 1, 4, 2, 1, 1, 0, 0]);
}
