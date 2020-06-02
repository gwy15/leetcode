/*
 * @lc app=leetcode.cn id=1337 lang=rust
 *
 * [1337] 方阵中战斗力最弱的 K 行
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut arr: Vec<(i32, usize)> = mat
            .iter()
            .enumerate()
            .map(|(i, v)| (v.iter().sum(), i))
            .collect();
        arr.sort_unstable();

        arr.iter()
            .take(k as usize)
            .map(|&(_, i)| i as i32)
            .collect()
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($mat:tt, $k:expr, $ans:expr) => {
            assert_eq!(
                Solution::k_weakest_rows(vec2d!$mat, $k),
                $ans
            );
        }
    };
    test!(
        [
            [1, 1, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 0, 0, 0],
            [1, 1, 1, 1, 1]
        ],
        3,
        [2, 0, 3]
    );
    test!(
        [[1, 0, 0, 0], [1, 1, 1, 1], [1, 0, 0, 0], [1, 0, 0, 0]],
        2,
        [0, 2]
    );
}
