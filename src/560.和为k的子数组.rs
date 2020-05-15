/*
 * @lc app=leetcode.cn id=560 lang=rust
 *
 * [560] 和为K的子数组
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut sum = 0;
        let mut solutions = 0;
        for num in nums.iter() {
            // mark index that starts from i
            *counter.entry(sum + k).or_insert(0) += 1;
            // maintain pre sum
            sum += num;
            // ends with i
            if let Some(count) = counter.get(&sum) {
                solutions += count;
            }
        }
        solutions
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $k:expr, $a:expr) => {
            assert_eq!(
                Solution::subarray_sum(vec!$v, $k),
                $a
            );
        }
    };
    test!([1, 1, 1], 2, 2);
    test!([1, -1, 0], 1, 1);
    test!([1, -1, 0], 0, 3);
}
