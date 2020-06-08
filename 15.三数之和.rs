/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */
struct Solution;
// @lc code=start
use std::cmp::Ordering;
impl Solution {
    fn two_sum_sorted(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let (mut left, mut right) = (0, n - 1);
        let mut ans = Vec::new();
        while left < right {
            let sum = nums[left] + nums[right];
            if sum == target {
                ans.push(vec![nums[left], nums[right]]);
            }
            match sum.cmp(&target) {
                Ordering::Equal | Ordering::Less => {
                    left += 1;
                    while left < n - 1 && nums[left - 1] == nums[left] {
                        left += 1;
                    }
                }
                Ordering::Greater => {
                    right -= 1;
                    while right > 0 && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }
            }
        }
        ans
    }
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        if n < 3 {
            return vec![];
        }
        let mut ans = vec![];
        for i in 0..n - 2 {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
            let first = nums[i];
            let target = -first;
            for mut rest in Self::two_sum_sorted(&nums[i + 1..], target) {
                rest.insert(0, first);
                ans.push(rest);
            }
        }
        ans
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:tt) => {
            let ans: Vec<Vec<i32>> = vec2d!$ans;
            assert_eq!(
                Solution::three_sum(vec!$n),
                ans
            );
        }
    };
    test!([-1, 0, 1, 2, -1, -4], [[-1, -1, 2], [-1, 0, 1]]);
    test!([1, 1, 1], []);
    test!([-1, 0, 1], [[-1, 0, 1]]);
    test!([], []);
    test!([0, 0, 0, 0], [[0, 0, 0]]);
    test!([0, 1, 1], []);
}
