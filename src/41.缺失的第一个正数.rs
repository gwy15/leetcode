/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 */
struct Solution;
// @lc code=start
#[allow(unused)]
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        for i in 0..nums.len() {
            loop {
                let x = nums[i];
                if 0 < x && x < n && nums[x as usize - 1] != x {
                    nums.swap(i, x as usize - 1);
                } else {
                    break;
                }
            }
        }
        for (i, x) in nums.into_iter().enumerate() {
            if x != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        return n + 1;
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:expr) => {
            assert_eq!(
                Solution::first_missing_positive(vec!$n),
                $ans
            );
        }
    };
    test!([1, 2, 0], 3);
    test!([3, 4, -1, 1], 2);
    test!([7, 8, 9, 11, 12], 1);
    test!([1, 2, 3], 4);
    test!([], 1);
    test!([-1, 4, 2, 1, 9, 10], 3);
    test!([2, 1], 3);
    test!(
        [11, 1, 6, 11, 5, 5, -6, 9, -3, 9, 5, 4, 2, -8, 16, -6, -4, 2, 3],
        7
    );
}
