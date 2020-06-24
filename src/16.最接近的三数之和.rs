/*
 * @lc app=leetcode.cn id=16 lang=rust
 *
 * [16] 最接近的三数之和
 */
struct Solution;
// @lc code=start
#[allow(unused)]
impl Solution {
    fn two_sum_closest_in_sorted(nums: &[i32], target: i32) -> i32 {
        let n = nums.len();
        let mut ans: i32 = nums[..2].iter().sum();

        let (mut i, mut j) = (0, n - 1);
        while i < j {
            let (left, right) = (nums[i], nums[j]);
            let cur_result = left + right;
            if (cur_result - target).abs() < (ans - target).abs() {
                ans = cur_result;
            }

            use std::cmp::Ordering::*;
            match cur_result.cmp(&target) {
                Less => {
                    // move left
                    while i < j && nums[i] == left {
                        i += 1;
                    }
                }
                Equal => return target,
                Greater => {
                    // move right
                    while i < j && nums[j] == right {
                        j -= 1;
                    }
                }
            }
        }

        ans
    }

    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        if n < 3 {
            panic!("???");
        }

        let mut result: i32 = nums[..3].iter().sum();

        let mut i = 0;
        while i < n - 2 {
            let first = nums[i];
            let closest_result =
                first + Self::two_sum_closest_in_sorted(&nums[i + 1..], target - first);
            if (closest_result - target).abs() < (result - target).abs() {
                result = closest_result;
            }
            if result == target {
                return result;
            }
            while i < n - 2 && nums[i] == first {
                i += 1;
            }
        }

        result
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($nums:tt, $target:expr, $ans:expr) => {
            assert_eq!(
                Solution::three_sum_closest(vec!$nums, $target),
                $ans
            );
        }
    };
    test!([-1, 2, 1, -4], 1, 2);
    test!([1, 2, 3], 0, 6);
}
