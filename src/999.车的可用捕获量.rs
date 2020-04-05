/*
 * @lc app=leetcode.cn id=999 lang=rust
 *
 * [999] 车的可用捕获量
 */
struct Solution;
// @lc code=start
impl Solution {
    fn find_rook(board: &Vec<Vec<char>>) -> (usize, usize) {
        for (i, row) in board.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch == 'R' {
                    return (i, j);
                }
            }
        }
        unreachable!();
    }
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let (m, n) = (board.len(), board[0].len());
        let (i, j) = Solution::find_rook(&board);

        let mut sum = 0;

        macro_rules! run {
            ($iter:expr) => {{
                let mut iter = $iter;
                loop {
                    match iter.next() {
                        Some((x, y)) => match board[x][y] {
                            '.' => continue,
                            'B' => break 0,
                            'p' => break 1,
                            _ => unreachable!(),
                        },
                        None => break 0,
                    }
                }
            }};
        }

        sum += run!((0..i).rev().map(|x| (x, j)));
        sum += run!((i + 1..m).map(|x| (x, j)));
        sum += run!((0..j).rev().map(|y| (i, y)));
        sum += run!((j + 1..n).map(|y| (i, y)));

        sum
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        (
            [
                $(
                    [$($i:expr),*]
                ),*
            ]
            ,
            $ans:expr
        ) => {
            assert_eq!(Solution::num_rook_captures(
                vec![
                    $(
                        vec![
                            $($i),*
                        ]
                    ),*
                ]
            ), $ans);
        };
    };
    test!(
        [
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', 'p', '.', '.', '.', '.'],
            ['.', '.', '.', 'R', '.', '.', '.', 'p'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', 'p', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.']
        ],
        3
    );
}
