/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */
struct Solution;
// @lc code=start
use std::cmp::Ordering::*;
impl Solution {
    #[allow(unused)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        // 保证 nums1 更短
        if m > n {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }
        if m == 0 {
            if n % 2 == 1 {
                return nums2[n / 2] as f64;
            } else {
                return (nums2[n / 2 - 1] + nums2[n / 2]) as f64 / 2.0;
            }
        }

        const INF: i32 = i32::max_value();

        // 二分查找
        // [left, right)
        let (mut left, mut right) = (0, m + 1);
        while left < right - 1 {
            //
            let mid = (left + right) / 2;
            let i2 = (m + n + 1) / 2 - mid;
            // 计算
            let n1_l = if mid == 0 { -INF } else { nums1[mid - 1] };
            let n2_r = if i2 == n { INF } else { nums2[i2] };

            if n1_l <= n2_r {
                left = mid;
            } else {
                right = mid;
            }
        }
        // answer
        let i2 = (m + n + 1) / 2 - left;
        let n1_l = if left == 0 { -INF } else { nums1[left - 1] };
        let n2_r = if i2 == n { INF } else { nums2[i2] };
        let n2_l = if i2 == 0 { -INF } else { nums2[i2 - 1] };
        let n1_r = if left == m { INF } else { nums1[left] };
        let mid1 = i32::max(n1_l, n2_l);
        let mid2 = i32::min(n1_r, n2_r);
        // eprintln!("n1_l={}, n1_r={}, n2_l={}, n2_r={}", n1_l, n1_r, n2_l, n2_r);
        // eprintln!("l={}, mid1={}, mid2={}", left, mid1, mid2);
        return if (m + n) % 2 == 1 {
            mid1 as f64
        } else {
            (mid1 + mid2) as f64 / 2.0
        };
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n1:tt, $n2:tt, $ans:expr) => {
            assert_eq!(
                Solution::find_median_sorted_arrays(vec!$n1, vec!$n2),
                $ans
            );
        }
    };
    test!([1, 3], [2], 2.0);
    test!([1, 2], [3, 4], 2.5);
    test!([1, 2, 3, 4], [], 2.5);
    test!([1, 1, 1], [1, 1, 1], 1.0);
    test!([1, 3, 5, 7], [2, 4, 6, 8], 4.5);
    test!([1, 3, 5, 7], [2, 4, 6], 4.0);
    test!([3], [-2, -1], -1.0);
    test!([1, 2], [1, 1], 1.0);
    test!([3], [1, 2, 4], 2.5);
    test!([2], [1, 3, 4, 5], 3.0);
    test!([1, 3], [2, 4, 5], 3.0);
}
