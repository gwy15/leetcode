/*
 * @lc app=leetcode.cn id=1103 lang=rust
 *
 * [1103] 分糖果 II
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let p = (candies as f64 * 2.0 + 0.25).sqrt() - 0.5;
        let p = p.floor() as i32;
        let remaining = candies - p * (p + 1) / 2;
        let (rows, cols) = (p / num_people, p % num_people);

        // distribute
        let mut ans = vec![0; num_people as usize];
        let common = num_people * rows * (rows - 1) / 2;
        for i in 0..num_people {
            ans[i as usize] = (i + 1) * rows + common;
            if i < cols {
                ans[i as usize] += i + 1 + rows * num_people;
            }
        }
        ans[cols as usize] += remaining;

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($candies:expr, $n:expr, $ans:tt) => {
            assert_eq!(
                Solution::distribute_candies($candies, $n),
                vec!$ans
            );
        }
    };
    test!(7, 4, [1, 2, 3, 1]);
    test!(10, 3, [5, 2, 3]);
}
