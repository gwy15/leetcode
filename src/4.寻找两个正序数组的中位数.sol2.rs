/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */
struct Solution;
// @lc code=start
impl Solution {
    fn find_k_smallest(nums1: &[i32], nums2: &[i32], k: usize) -> i32 {
        if nums1.len() > nums2.len() {
            return Self::find_k_smallest(nums2, nums1, k);
        }
        if nums1.len() == 0 {
            return nums2[k];
        }
        if k == 0 {
            return nums1[0].min(nums2[0]);
        }
        // k1 for nums1, k1 <= k2
        let k1 = (nums1.len() - 1).min(k / 2);
        let k2 = (nums2.len() - 1).min(k - 1 - k1);
        if nums1[k1] < nums2[k2] {
            Self::find_k_smallest(&nums1[k1 + 1..], nums2, k - k1 - 1)
        } else {
            Self::find_k_smallest(nums1, &nums2[k2 + 1..], k - k2 - 1)
        }
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        let mid = (m + n) / 2;
        if (m + n) % 2 == 1 {
            Self::find_k_smallest(&nums1, &nums2, mid) as f64
        } else {
            (Self::find_k_smallest(&nums1, &nums2, mid - 1) as f64
                + Self::find_k_smallest(&nums1, &nums2, mid) as f64)
                / 2.
        }
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
    test!([0], [1], 0.5);
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
