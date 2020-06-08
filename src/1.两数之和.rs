/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut n2i: HashMap<i32, i32> = HashMap::new();

        for (i, n) in nums.into_iter().enumerate() {
            let request = target - n;
            if let Some(&j) = n2i.get(&request) {
                return vec![j, i as i32];
            }
            n2i.insert(n, i as i32);
        }
        unreachable!()
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $target:expr, $ans:tt) => {
            assert_eq!(
                Solution::two_sum(vec!$n, $target),
                vec!$ans
            );
        }
    };
    test!([2, 7, 11, 15], 9, [0, 1]);
}
