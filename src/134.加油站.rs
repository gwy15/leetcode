/*
 * @lc app=leetcode.cn id=134 lang=rust
 *
 * [134] 加油站
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut sum = 0;
        let (mut min_sum, mut ans) = (0, 0);
        (0..n).for_each(|i| {
            sum += gas[i] - cost[i];
            if sum < min_sum {
                min_sum = sum;
                ans = (i + 1) % n;
            }
        });
        if sum < 0 {
            return -1;
        } else {
            ans as i32
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($g:tt, $c:tt, $ans:expr) => {
            assert_eq!(
                Solution::can_complete_circuit(vec!$g, vec!$c),
                $ans
            );
        }
    };
    test!([1, 2, 3, 4, 5], [3, 4, 5, 1, 2], 3);
    test!([2, 3, 4], [3, 4, 3], -1);
}
