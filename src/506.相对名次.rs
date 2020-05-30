/*
 * @lc app=leetcode.cn id=506 lang=rust
 *
 * [506] 相对名次
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    #[allow(unused)]
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let mut sorted = nums.clone();
        sorted.sort_by_key(|x| -x);
        let mut mapper: HashMap<i32, usize> = HashMap::new();
        sorted.iter().enumerate().for_each(|(ranking, &num)| {
            mapper.insert(num, ranking);
        });

        nums.iter()
            .map(|num| match mapper[num] {
                0 => "Gold Medal".into(),
                1 => "Silver Medal".into(),
                2 => "Bronze Medal".into(),
                i => format!("{}", i + 1),
            })
            .collect()
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:tt) => {
            assert_eq!(
                Solution::find_relative_ranks(vec!$n),
                vec!$ans
            );
        }
    };
    test!(
        [5, 4, 3, 2, 1],
        ["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
    );
}
