/*
 * @lc app=leetcode.cn id=330 lang=rust
 *
 * [330] 按要求补齐数组
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut ans = 0;
        // getable region: [1, right)
        let mut right: i64 = 1;
        let mut i = 0;
        // iterate, where right is the number trying to get
        while right <= n as i64 {
            if i < nums.len() && nums[i] as i64 <= right {
                // can get right with nums[..i] and nums[i]
                right = right + nums[i] as i64;
                i += 1;
            } else {
                // cannot get right, add right to numbers
                eprintln!("add {}", right);
                right = right + right;
                ans += 1;
            }
        }
        eprintln!();
        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($nums:tt, $n:expr, $ans:expr) => {
            assert_eq!(
                Solution::min_patches(vec!$nums, $n),
                $ans
            );
        }
    };
    test!([1, 3], 6, 1);
    test!([1, 5, 10], 20, 2);
    test!([1, 2, 2], 5, 0);
    test!([1, 2], 3, 0);
    test!([1, 2, 8], 15, 1);
    test!([1, 2, 8], 16, 2);
    test!([], 1, 1);
    test!([1, 2, 31, 33], 2147483647, 28);
}
