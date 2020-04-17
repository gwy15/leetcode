/*
 * @lc app=leetcode.cn id=1253 lang=rust
 *
 * [1253] 重构 2 行二进制矩阵
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let n = colsum.len();
        let mut row1 = vec![0; n];
        let mut row2 = vec![0; n];
        for i in 0..n {
            match colsum[i] {
                0 => {}
                2 => {
                    row1[i] = 1;
                    row2[i] = 1;
                    upper -= 1;
                    lower -= 1;
                }
                1 => match upper >= lower {
                    true => {
                        row1[i] = 1;
                        upper -= 1;
                    }
                    false => {
                        row2[i] = 1;
                        lower -= 1;
                    }
                },
                _ => unreachable!(),
            }
            if upper < 0 || lower < 0 {
                break;
            }
        }
        if upper != 0 || lower != 0 {
            return Vec::new();
        }
        vec![row1, row2]
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! vec2d {
        [$(
            [$(
                $i:expr
            ),*]
        ),*] => {
            vec![$(
                vec![$($i),*]
            ),*]
        }
    }
    macro_rules! test {
        ($u:expr, $l:expr, $c:tt, $a:expr) => {
            assert_eq!(
                Solution::reconstruct_matrix($u, $l, vec!$c),
                $a
            );
        }
    };
    test!(2, 1, [1, 1, 1], vec2d![[1, 1, 0], [0, 0, 1]]);
    test!(2, 3, [2, 2, 1, 1], Vec::<Vec<i32>>::new());
    test!(
        5,
        5,
        [2, 1, 2, 0, 1, 0, 1, 2, 0, 1],
        vec2d![
            [1, 1, 1, 0, 0, 0, 1, 1, 0, 0],
            [1, 0, 1, 0, 1, 0, 0, 1, 0, 1]
        ]
    );
    test!(4, 7, [2, 1, 2, 2, 1, 1, 1], Vec::<Vec<i32>>::new());
}
