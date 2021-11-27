/*
 * @lc app=leetcode.cn id=781 lang=rust
 *
 * [781] 森林中的兔子
 */

struct Solution;

// @lc code=start
use std::collections::BTreeMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut m = BTreeMap::<i32, i32>::new();
        for i in answers {
            *m.entry(i + 1).or_default() += 1;
        }
        let mut ans = 0;
        for (group_size, reporter) in m.into_iter() {
            fn ceiling_div(a: i32, b: i32) -> i32 {
                (a + b - 1) / b
            }

            ans += group_size * ceiling_div(reporter, group_size);
        }
        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(
                Solution::num_rabbits(vec!$args),
                $ans
            )
        };
    }
    test!([1, 1, 2], 5);
    test!([10, 10, 10], 11);
    test!([], 0);
    test!([1, 0, 1, 0, 0], 5);
}
