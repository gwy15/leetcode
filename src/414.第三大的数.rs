/*
 * @lc app=leetcode.cn id=414 lang=rust
 *
 * [414] 第三大的数
 */
struct Solution;
// @lc code=start
impl Solution {
    fn find(nums: &[i32], upper: i32) -> i32 {
        let mut max = upper;
        for &n in nums.iter() {
            if n < upper && (n > max || (max == upper)) {
                max = n;
            }
        }
        max
    }
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let max = Self::find(&nums, i32::max_value());
        let max2 = Self::find(&nums, max);
        let max3 = Self::find(&nums, max2);
        // eprintln!("{}, {}, {}", max, max2, max3);
        if max3 == max2 {
            max
        } else {
            max3
        }
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:expr) => {
            assert_eq!(
                Solution::third_max(vec!$n),
                $ans
            );
        }
    };
    test!([3, 2, 1], 1);
    test!([1, 2], 2);
    test!([2, 2, 3, 1], 1);
    test!([1], 1);
    test!([1, 2, -2147483648], -2147483648);
}
