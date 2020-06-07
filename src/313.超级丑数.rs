/*
 * @lc app=leetcode.cn id=313 lang=rust
 *
 * [313] 超级丑数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let n = n as usize;
        let m = primes.len();

        let mut nums = vec![1];
        nums.reserve(n);

        let mut ptr = vec![0; m];

        let mut tmp = vec![0; m];
        for _ in 1..n {
            let mut next_ugly = i32::max_value();
            for i in 0..m {
                tmp[i] = primes[i] * nums[ptr[i]];
                if tmp[i] < next_ugly {
                    next_ugly = tmp[i];
                }
            }
            nums.push(next_ugly);
            for i in 0..m {
                if tmp[i] == next_ugly {
                    ptr[i] += 1;
                }
            }
        }

        nums[n - 1]
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $p:tt, $ans:expr) => {
            assert_eq!(Solution::nth_super_ugly_number($n, vec!$p), $ans);
        };
    };
    test!(12, [2, 7, 13, 19], 32);
}
