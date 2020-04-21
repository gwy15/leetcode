/*
 * @lc app=leetcode.cn id=1248 lang=rust
 *
 * [1248] 统计「优美子数组」
 */
struct Solution;
// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        // find odd numbers
        let mut indexes = vec![-1];
        for (i, num) in nums.iter().enumerate() {
            if num % 2 == 1 {
                indexes.push(i as i32);
            }
        }
        indexes.push(nums.len() as i32);
        // early deny
        let n = indexes.len() - 2;
        if n < k {
            return 0;
        }
        // sum
        let mut cnt = 0;
        for i in 0..(n - k + 1) {
            //
            let left_length = indexes[i + 1] - indexes[i];
            let right_length = indexes[k + i + 1] - indexes[k + i];
            cnt += left_length * right_length;
        }
        cnt
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $k:expr, $a:expr) => {
            assert_eq!(
                Solution::number_of_subarrays(vec!$n, $k),
                $a
            );
        }
    };
    test!([1, 1, 2, 1, 1], 3, 2);
    test!([2, 4, 6], 1, 0);
    test!([2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2, 16);
    test!([1, 0, 0, 0], 1, 4);
    test!([0], 1, 0);
    test!([1], 1, 1);
}
