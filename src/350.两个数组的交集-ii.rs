/*
 * @lc app=leetcode.cn id=350 lang=rust
 *
 * [350] 两个数组的交集 II
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() > nums2.len() {
            return Self::intersect(nums2, nums1);
        }

        let mut ans = vec![];
        // pre-processing
        let mut times: HashMap<i32, i32> = HashMap::new();
        for n in nums1 {
            *times.entry(n).or_insert(0) += 1;
        }
        // generate
        for n in nums2 {
            if let Some(t) = times.get_mut(&n) {
                ans.push(n);
                *t -= 1;
                if *t == 0 {
                    times.remove(&n);
                }
            }
        }

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n1:tt, $n2:tt, $ans:tt) => {
            assert_eq!(
                Solution::intersect(vec!$n1, vec!$n2),
                vec!$ans
            );
        }
    };
    test!([1, 2, 2, 1], [2, 2], [2, 2]);
    test!([4, 9, 5], [9, 4, 9, 8, 4], [9, 4]);
}
