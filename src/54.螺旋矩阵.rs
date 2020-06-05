/*
 * @lc app=leetcode.cn id=54 lang=rust
 *
 * [54] 螺旋矩阵
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 {
            return vec![];
        }
        let (m, n) = (matrix.len(), matrix[0].len());

        let (mut up, mut down) = (0, m);
        let (mut left, mut right) = (0, n);

        let mut ans = Vec::new();
        loop {
            // rightwards
            for j in left..right {
                ans.push(matrix[up][j]);
            }
            up += 1;
            if up >= down {
                break;
            }
            // downwards
            for i in up..down {
                ans.push(matrix[i][right - 1]);
            }
            right -= 1;
            if left >= right {
                break;
            }
            // leftwards
            for j in (left..right).rev() {
                ans.push(matrix[down - 1][j]);
            }
            down -= 1;
            if up >= down {
                break;
            }
            // upwards
            for i in (up..down).rev() {
                ans.push(matrix[i][left]);
            }
            left += 1;
            if left >= right {
                break;
            }
        }

        ans
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:tt) => {
            assert_eq!(
                Solution::spiral_order(vec2d!$n),
                vec!$ans
            );
        }
    };
    test!(
        [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        [1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
    test!(
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
    test!([[1]], [1]);
    test!([[1, 2]], [1, 2]);
    test!([], []);
    test!([[3], [2]], [3, 2]);
}
