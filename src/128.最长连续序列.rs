/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] 最长连续序列
 */
struct Solution;
// @lc code=start
use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums);
        let mut ans = 0;
        for &num in set.iter() {
            // not start
            if set.contains(&(num - 1)) {
                continue;
            }
            let mut length = 1;
            let mut x = num + 1;
            while set.contains(&x) {
                length += 1;
                x += 1;
            }
            ans = ans.max(length);
        }

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:expr) => {
            assert_eq!(
                Solution::longest_consecutive(vec!$n),
                $ans
            );
        }
    };
    test!([100, 4, 200, 1, 3, 2], 4);
    test!([], 0);
    test!([1], 1);
    test!([1, 2, 3], 3);
}
