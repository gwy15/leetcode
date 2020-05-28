/*
 * @lc app=leetcode.cn id=334 lang=rust
 *
 * [334] 递增的三元子序列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut small, mut mid) = (i32::max_value(), i32::max_value());
        for n in nums {
            if n > mid {
                return true;
            } else if n < small {
                small = n;
            } else if small < n && n < mid {
                mid = n;
            }
        }

        false
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $ans:expr) => {
            assert_eq!(
                Solution::increasing_triplet(vec!$v),
                $ans
            );
        }
    };
    test!([1, 2, 3, 4, 5], true);
    test!([5, 4, 3, 2, 1], false);
}
