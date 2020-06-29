/*
 * @lc app=leetcode.cn id=215 lang=rust
 *
 * [215] 数组中的第K个最大元素
 */
struct Solution;
// @lc code=start
use rand::prelude::*;

#[allow(unused)]
impl Solution {
    fn find_kth_smallest(nums: &mut [i32], k: usize) -> i32 {
        let n = nums.len();
        if k == 0 {
            return *nums.into_iter().min().unwrap();
        }
        // quit sort
        let pivot = nums[0];
        let (mut i, mut j) = (1, n - 1);
        while i < j {
            while i < j && nums[i] <= pivot {
                i += 1;
            }
            while i < j && pivot <= nums[j] {
                j -= 1;
            }
            nums.swap(i, j);
        }

        // [0, mid), mid(pivot), [mid+1, n)
        let mid = if nums[i] <= pivot { i } else { i - 1 };
        // swap pivot to mid
        nums.swap(0, mid);

        // recursive
        if k == mid {
            return pivot;
        } else if k < mid {
            // sort left
            Self::find_kth_smallest(&mut nums[0..mid], k)
        } else {
            Self::find_kth_smallest(&mut nums[mid + 1..n], k - mid - 1)
        }
    }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut rng = thread_rng();
        nums.shuffle(&mut rng);

        let n = nums.len();
        let k = n - k as usize;
        Self::find_kth_smallest(&mut nums, k)
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $k:expr, $ans:expr) => {
            assert_eq!(
                Solution::find_kth_largest(vec!$v, $k),
                $ans
            );
            eprintln!();
        }
    };
    test!([3, 2, 1], 1, 3);
    test!([3, 2, 1], 2, 2);
    test!([3, 2, 1], 3, 1);
    test!([3, 2, 1, 5, 6, 4], 2, 5);
    test!([3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4);
    test!([1], 1, 1);
}
