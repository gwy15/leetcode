/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */
struct Solution;
// @lc code=start
use std::cmp::Ordering;
impl Solution {
    #[inline]
    fn two_sum_sorted(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let (mut left, mut right) = (0, n - 1);
        let mut ans = Vec::new();
        while left < right {
            let (a, b) = (nums[left], nums[right]);
            let sum = a + b;
            if sum == target {
                ans.push(vec![nums[left], nums[right]]);
            }
            match sum.cmp(&target) {
                Ordering::Equal | Ordering::Less => {
                    // move left
                    while nums[left] == a && left < right {
                        left += 1;
                    }
                }
                Ordering::Greater => {
                    // move right
                    while nums[right] == b && left < right {
                        right -= 1;
                    }
                }
            }
        }
        ans
    }
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n < 3 {
            return vec![];
        }
        nums.sort_unstable();
        let mut ans = vec![];
        let mut i = 0;
        while i < n - 2 {
            let first = nums[i];
            if first > 0 {
                break;
            }
            for mut rest in Self::two_sum_sorted(&nums[i + 1..], -first) {
                rest.insert(0, first);
                ans.push(rest);
            }
            while nums[i] == first && i < n - 2 {
                i += 1;
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
