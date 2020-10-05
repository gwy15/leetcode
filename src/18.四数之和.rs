/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 */
struct Solution;
// @lc code=start
use std::cmp::Ordering::*;
impl Solution {
    fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut ans = vec![];
        if n < 2 {
            return ans;
        }
        let (mut i, mut j) = (0, n - 1);
        while i < j {
            let s = nums[i] + nums[j];
            match s.cmp(&target) {
                Equal => {
                    ans.push(vec![nums[i], nums[j]]);
                    let n = nums[i];
                    while i < j && n == nums[i] {
                        i += 1;
                    }
                }
                Less => {
                    let n = nums[i];
                    while i < j && n == nums[i] {
                        i += 1;
                    }
                }
                Greater => {
                    let n = nums[j];
                    while i < j && n == nums[j] {
                        j -= 1;
                    }
                }
            }
        }
        ans
    }

    fn three_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut ans = vec![];
        if n < 3 {
            return ans;
        }
        let mut i = 0;
        while i < n {
            for rest in Self::two_sum(&nums[i + 1..], target - nums[i]) {
                ans.push(vec![nums[i], rest[0], rest[1]]);
            }
            let start = nums[i];
            while i < nums.len() && start == nums[i] {
                i += 1;
            }
        }
        ans
    }

    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let n = nums.len();
        let mut ans = vec![];
        if n < 4 {
            return ans;
        }
        let mut i = 0;
        while i < n {
            for rest in Self::three_sum(&nums[i + 1..], target - nums[i]) {
                ans.push(vec![nums[i], rest[0], rest[1], rest[2]]);
            }
            let start = nums[i];
            while i < nums.len() && start == nums[i] {
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
        ($args:tt, $target:expr, $ans:tt) => {
            assert_eq!(
                Solution::four_sum(vec!$args, $target),
                vec2d!$ans
            )
        };
    }
    test!(
        [1, 0, -1, 0, -2, 2],
        0,
        [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
    );
}
