/*
 * @lc app=leetcode.cn id=1437 lang=rust
 *
 * [1437] 是否所有 1 都至少相隔 k 个元素
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut i = 0;
        while i < n {
            match nums[i] {
                0 => {
                    i += 1;
                }
                1 => {
                    i += 1;
                    for j in 0..k as usize {
                        if i + j < n {
                            if nums[i + j] == 1 {
                                return false;
                            }
                        } else {
                            return true;
                        }
                    }
                    i += k as usize;
                }
                _ => unreachable!(),
            }
        }
        true
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $k:expr, $ans:expr) => {
            assert_eq!(
                Solution::k_length_apart(vec!$n, $k),
                $ans
            );
        }
    };
    test!([1, 0, 0, 0, 1, 0, 0, 1], 2, true);
    test!([1, 0, 0, 1, 0, 1], 2, false);
    test!([1, 1, 1, 1, 1], 0, true);
    test!([0, 1, 0, 1], 1, true);
}
