/*
 * @lc app=leetcode.cn id=264 lang=rust
 *
 * [264] 丑数 II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut nums = vec![1];
        let mut ptr = vec![0, 0, 0];
        while nums.len() < n {
            let result: Vec<i32> = (0..3).map(|i| [2, 3, 5][i] * nums[ptr[i]]).collect();
            let next_ugly = *result.iter().min().unwrap();
            nums.push(next_ugly);
            for i in 0..3 {
                if result[i] == next_ugly {
                    ptr[i] += 1;
                }
            }
        }

        nums[nums.len() - 1]
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $ans:expr) => {
            assert_eq!(Solution::nth_ugly_number($n), $ans);
        };
    };
    test!(10, 12);
}
