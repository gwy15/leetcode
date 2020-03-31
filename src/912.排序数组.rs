/*
 * @lc app=leetcode.cn id=912 lang=rust
 *
 * [912] 排序数组
 */

struct Solution;

// @lc code=start
impl Solution {
    /// Pick a pivot and return its index
    fn pick_pivot(nums: &[i32]) -> usize {
        let n = nums.len();
        debug_assert!(n >= 3);
        let (a, b, c) = (nums[0], nums[n / 2], nums[n - 1]);
        let max = a.max(b).max(c);
        if max == a {
            if b > c {
                n / 2
            } else {
                n - 1
            }
        } else if max == b {
            if a > c {
                0
            } else {
                n - 1
            }
        } else {
            if a > b {
                0
            } else {
                n / 2
            }
        }
    }

    fn swap_by_pivot(nums: &mut [i32], mut pivot_index: usize) -> usize {
        if pivot_index != 0 {
            nums.swap(0, pivot_index);
            pivot_index = 0;
        }
        let n = nums.len();
        let pivot = nums[pivot_index];

        // index <left or >right is sorted
        let (mut left, mut right) = (0, n - 1);
        while left < right {
            // pick a right-most index that is less than pivot
            while nums[right] >= pivot && left < right {
                right -= 1
            }
            // pick a left-most index that is greater than pivot
            while nums[left] <= pivot && left < right {
                left += 1;
            }
            // swap if needed
            if left < right {
                nums.swap(left, right);
            }
        }
        nums.swap(left, 0);
        left
    }

    fn sort_raw_array(mut nums: &mut [i32]) {
        // 边界条件
        match nums.len() {
            0 | 1 => return,
            2 => {
                if nums[0] > nums[1] {
                    nums.swap(0, 1);
                }
                return;
            }
            _ => {}
        };
        let pivot_index = Solution::pick_pivot(nums);
        let sorted_bound = Solution::swap_by_pivot(&mut nums, pivot_index);
        Solution::sort_raw_array(&mut nums[..sorted_bound]);
        Solution::sort_raw_array(&mut nums[sorted_bound + 1..]);
    }

    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        // quick sort
        Solution::sort_raw_array(&mut nums[..]);
        nums
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        [$($i:expr),*] => {
            let mut ans = vec![$($i),*];
            ans.sort();
            assert_eq!(
                Solution::sort_array(vec![$($i),*]),
                ans
            );
        };
    }
    test![1];
    test![1, 2];
    test![2, 1];
    test![1, 1, 1, 1];
    test![1, 2, 3, 4];
    test![4, 3, 2, 1];
    test![9, 7, 1, 2, 4, 4, 4, 6];
    test![3, 1, 2, 5, 5, 5, 8];
}
