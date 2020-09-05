/*
 * @lc app=leetcode.cn id=373 lang=rust
 *
 * [373] 查找和最小的K对数字
 */
struct Solution;
// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::FromIterator;
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut ans = vec![];
        if nums2.len() == 0 {
            return ans;
        }
        let mut options = BinaryHeap::new();
        for i in 0..nums1.len().min(k) {
            let sum = nums1[i] + nums2[0];
            options.push((Reverse(sum), i, 0));
        }
        let mut options = BinaryHeap::from_iter(options);
        for i in 0..k {
            if let Some((_, i, j)) = options.pop() {
                ans.push(vec![nums1[i], nums2[j]]);
                let j = j + 1;
                if j < nums2.len() {
                    let sum = nums1[i] + nums2[j];
                    options.push((Reverse(sum), i, j));
                }
            } else {
                break;
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
        ($n1:tt, $n2:tt, $k:expr, $ans:tt) => {
            assert_eq!(
                Solution::k_smallest_pairs(vec!$n1, vec!$n2, $k),
                vec2d!$ans
            )
        };
    }
    test!([1, 7, 11], [2, 4, 6], 3, [[1, 2], [1, 4], [1, 6]]);
    test!([1, 1, 2], [1, 2, 3], 2, [[1, 1], [1, 1]]);
    test!([1, 2], [3], 3, [[1, 3], [2, 3]]);
    assert_eq!(
        Solution::k_smallest_pairs(vec![3, 5, 7, 9], vec![], 1),
        Vec::<Vec<_>>::new()
    );
}
