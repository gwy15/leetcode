/*
 * @lc app=leetcode.cn id=1561 lang=rust
 *
 * [1561] 你可以获得的最大硬币数目
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        let n = piles.len();
        (0..n / 3).map(|i| piles[n - 1 - (2 * i + 1)]).sum()
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(
                Solution::max_coins(vec!$args),
                $ans
            )
        };
    }
    test!([2, 4, 1, 2, 7, 8], 9);
    test!([2, 4, 5], 4);
    test!([9, 8, 7, 6, 5, 1, 2, 3, 4], 18)
}
