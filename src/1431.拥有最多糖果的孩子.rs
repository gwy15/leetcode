/*
 * @lc app=leetcode.cn id=1431 lang=rust
 *
 * [1431] 拥有最多糖果的孩子
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candy = *candies.iter().max().unwrap();

        candies
            .into_iter()
            .map(|n| (max_candy - n) <= extra_candies)
            .collect()
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($candies:tt, $extra:expr, $ans:tt) => {
            assert_eq!(
                Solution::kids_with_candies(vec!$candies, $extra),
                vec!$ans
            );
        }
    };
    test!([2, 3, 5, 1, 3], 3, [true, true, true, false, true]);
    test!([4, 2, 1, 1, 2], 1, [true, false, false, false, false]);
    test!([12, 1, 12], 10, [true, false, true]);
}
