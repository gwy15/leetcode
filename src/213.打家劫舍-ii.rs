/*
 * @lc app=leetcode.cn id=213 lang=rust
 *
 * [213] 打家劫舍 II
 */
struct Solution;
// @lc code=start
impl Solution {
    fn simple_rob(nums: &[i32]) -> i32 {
        let (mut last, mut cur) = (0, 0);
        for n in nums {
            let tmp = cur.max(last + n);
            last = cur;
            cur = tmp;
        }
        cur
    }
    #[allow(unused)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        match n {
            0 => 0,
            1 => nums[0],
            _ => Self::simple_rob(&nums[0..n - 1]).max(Self::simple_rob(&nums[1..n])),
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:expr) => {
            assert_eq!(
                Solution::rob(vec!$n),
                $ans
            );
        }
    };
    test!([2, 3, 2], 3);
    test!([1, 2, 3, 1], 4);
    test!([], 0);
}
