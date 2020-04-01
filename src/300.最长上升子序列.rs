/*
 * @lc app=leetcode.cn id=300 lang=rust
 *
 * [300] 最长上升子序列
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut end_len = Vec::new();
        for num in nums.iter() {
            // bigger than last one
            if end_len.is_empty() || num > end_len[end_len.len() - 1] {
                end_len.push(num);
                continue;
            }
            match end_len.binary_search(&num) {
                // end_len[i] == num, discard
                Ok(_) => continue,
                // end_len[i] < num < end_len[j]
                Err(j) => end_len[j] = num,
            }
        }
        end_len.len() as i32
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ([$($i:expr),*], $ans:expr) => {
            assert_eq!(
                Solution::length_of_lis(vec![$($i),*]),
                $ans
            );
        }
    };
    test!([10, 9, 2, 5, 3, 7, 101, 18], 4);
    test!([1], 1);
    test!([9, 8, 7, 6, 5, 4, 3, 2, 1, 6], 2);
}
