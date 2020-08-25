/*
 * @lc app=leetcode.cn id=491 lang=rust
 *
 * [491] 递增子序列
 */
struct Solution;
// @lc code=start
impl Solution {
    /// selected: a ascending sub sequence
    fn dfs(selected: &mut Vec<i32>, rest: &[i32], ans: &mut Vec<Vec<i32>>) {
        if rest.len() == 0 {
            if selected.len() >= 2 {
                ans.push(selected.clone());
            }
            return;
        }
        let &last = selected.last().unwrap_or(&i32::min_value());
        // if last <= rest[0] {
        //     selected.push(rest[0]);
        //     Self::dfs(selected, &rest[1..], ans);
        //     selected.pop();
        // }
        // if last != rest[0] {
        //     Self::dfs(selected, &rest[1..], ans);
        // }
        use std::cmp::Ordering::*;
        match last.cmp(&rest[0]) {
            Less => {
                // 选择
                selected.push(rest[0]);
                Self::dfs(selected, &rest[1..], ans);
                selected.pop();
                // 不选择
                Self::dfs(selected, &rest[1..], ans);
            }
            Equal => {
                // 相等要一直选择
                // 即，从任意起点开始（< 情况会分裂），一直到相等终点，从而只遍历一次
                selected.push(rest[0]);
                Self::dfs(selected, &rest[1..], ans);
                selected.pop();
            }
            Greater => {
                // 不选择这个元素
                Self::dfs(selected, &rest[1..], ans);
            }
        }
    }

    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        let mut selected = Vec::new();
        Self::dfs(&mut selected, &nums, &mut ans);

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
        ($args:tt, $ans:tt) => {
            let lhs = HashSet::<Vec<i32>>::from_iter(Solution::find_subsequences(vec!$args).into_iter());
            let rhs = HashSet::from_iter(vec2d!$ans.into_iter());
            assert_eq!(lhs, rhs);
        };
    }
    test!(
        [4, 6, 7, 7],
        [
            [4, 6],
            [4, 6, 7],
            [4, 6, 7, 7],
            [4, 7],
            [4, 7, 7],
            [6, 7],
            [6, 7, 7],
            [7, 7]
        ]
    );
    test!(
        [1, 2, 3, 1, 1],
        [[1, 2], [1, 3], [2, 3], [1, 1], [1, 2, 3], [1, 1, 1]]
    );
}
