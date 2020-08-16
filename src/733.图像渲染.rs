/*
 * @lc app=leetcode.cn id=733 lang=rust
 *
 * [733] 图像渲染
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let (m, n) = (image.len(), image[0].len());
        let (sr, sc) = (sr as usize, sc as usize);
        let mut stack = vec![];
        let old_color = image[sr][sc];
        if old_color == new_color {
            return image;
        }
        macro_rules! check {
            ($cond:expr, $x:expr, $y:expr) => {
                if $cond && image[$x][$y] == old_color {
                    stack.push(($x, $y));
                    image[$x][$y] = new_color;
                }
            };
        }

        check!(true, sr, sc);
        while stack.len() > 0 {
            let (x, y) = stack.pop().unwrap();
            check!(x > 0, x - 1, y);
            check!(y > 0, x, y - 1);
            check!(x + 1 < m, x + 1, y);
            check!(y + 1 < n, x, y + 1);
        }

        image
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, ($sr:expr, $sc:expr, $color:expr), $ans:tt) => {
            let im = vec2d!$args;
            assert_eq!(
                Solution::flood_fill(im, $sr, $sc, $color),
                vec2d!$ans
            )
        };
    }
    test!(
        [[1, 1, 1], [1, 1, 0], [1, 0, 1]],
        (1, 1, 2),
        [[2, 2, 2], [2, 2, 0], [2, 0, 1]]
    );
    test!([[0, 0, 0], [0, 1, 1]], (1, 1, 1), [[0, 0, 0], [0, 1, 1]]);
}
