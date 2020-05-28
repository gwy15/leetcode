/*
 * @lc app=leetcode.cn id=1276 lang=rust
 *
 * [1276] 不浪费原料的汉堡制作方案
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        if tomato_slices % 2 == 1 {
            return Vec::new();
        }
        let big_mac = tomato_slices / 2 - cheese_slices;
        let whopper = cheese_slices - big_mac;
        if big_mac >= 0 && whopper >= 0 {
            return vec![big_mac, whopper];
        } else {
            return vec![];
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($a:expr, $b:expr, $ans:tt) => {
            assert_eq!(
                Solution::num_of_burgers($a, $b),
                vec!$ans
            );
        }
    };
    test!(16, 7, [1, 6]);
    test!(17, 4, []);
    test!(4, 17, []);
    test!(0, 0, [0, 0]);
    test!(2, 1, [0, 1]);
}
