/*
 * @lc app=leetcode.cn id=1356 lang=rust
 *
 * [1356] 根据数字二进制下 1 的数目排序
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by_key(|&n| {
            let mut ones = 0;
            for i in 0..32 {
                if (n >> i) & 1 == 1 {
                    ones += 1;
                }
            }
            (ones, n)
        });
        arr
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:tt) => {
            assert_eq!(
                Solution::sort_by_bits(vec!$args),
                vec!$ans
            )
        };
    }
    test!([0, 1, 2, 3, 4, 5, 6, 7, 8], [0, 1, 2, 4, 8, 3, 5, 6, 7]);
    test!(
        [1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1],
        [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
    );
    test!([2, 3, 5, 7, 11, 13, 17, 19], [2, 3, 5, 17, 7, 11, 13, 19]);
}
