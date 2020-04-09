/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Solution::generate_parenthesis_with(0, n, "".to_owned())
    }

    fn generate_parenthesis_with(level: i32, rest: i32, prefix: String) -> Vec<String> {
        if level == 0 && rest == 0 {
            return vec![prefix];
        }
        let mut result = Vec::new();
        if rest > 0 {
            // and go up
            let new_prefix = prefix.clone() + "(";
            result.extend(Solution::generate_parenthesis_with(
                level + 1,
                rest - 1,
                new_prefix,
            ));
        }
        if level > 0 {
            let new_prefix = prefix + ")";
            result.extend(Solution::generate_parenthesis_with(
                level - 1,
                rest,
                new_prefix,
            ));
        }
        return result;
    }
}
// @lc code=end
#[test]
fn test_solution() {
    let result = Solution::generate_parenthesis(3);
    assert_eq!(
        result,
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
}
