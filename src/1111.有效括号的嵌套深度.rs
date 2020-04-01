/*
 * @lc app=leetcode.cn id=1111 lang=rust
 *
 * [1111] 有效括号的嵌套深度
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut result = Vec::new();
        result.reserve(seq.len());
        let mut a = 0;
        let mut b = 0;
        for ch in seq.chars() {
            match ch {
                '(' => {
                    if a > b {
                        b += 1;
                        result.push(1);
                    } else {
                        a += 1;
                        result.push(0);
                    }
                }
                ')' => {
                    if a > b {
                        a -= 1;
                        result.push(0);
                    } else {
                        b -= 1;
                        result.push(1);
                    }
                }
                _ => unreachable!(),
            }
        }
        result
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $ans:expr) => {
            assert_eq!(Solution::max_depth_after_split($s.to_owned()), $ans);
        };
    };
    // test!("(()())", vec![0, 1, 1, 1, 1, 0]);
    // test!("()(())()", vec![0, 0, 0, 1, 1, 0, 1, 1]);
}
