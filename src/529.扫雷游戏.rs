/*
 * @lc app=leetcode.cn id=529 lang=rust
 *
 * [529] 扫雷游戏
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
#[allow(non_upper_case_globals)]
mod Blocks {
    pub const UnrevealedMine: char = 'M';
    pub const UnrevealedEmpty: char = 'E';
    pub const RevealedBlank: char = 'B';
    pub const RevealedMine: char = 'X';
}
impl Solution {
    #[allow(unused)]
    pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let (m, n) = (board.len(), board[0].len());
        let mut q = VecDeque::new();
        let (i, j) = (click[0] as usize, click[1] as usize);
        q.push_back((i, j));
        while !q.is_empty() {
            // i, j is not revealed
            let (i, j) = q.pop_front().unwrap();
            match board[i][j] {
                Blocks::UnrevealedMine => {
                    board[i][j] = Blocks::RevealedMine;
                    return board;
                }
                Blocks::UnrevealedEmpty => {
                    let mut mine_count = 0;
                    let mut neighbor_empty = Vec::with_capacity(8);
                    macro_rules! check {
                        ($i:expr, $j:expr) => {
                            if board[$i][$j] == Blocks::UnrevealedMine {
                                mine_count += 1;
                            } else if board[$i][$j] == Blocks::UnrevealedEmpty {
                                neighbor_empty.push(($i, $j));
                            }
                        };
                    };
                    macro_rules! check_col {
                        (0) => {
                            if j > 0 {
                                check!(i, j - 1);
                            }
                            if j + 1 < n {
                                check!(i, j + 1);
                            }
                        };
                        ($i:expr) => {
                            if j > 0 {
                                check!($i, j - 1);
                            }
                            check!($i, j);
                            if j + 1 < n {
                                check!($i, j + 1);
                            }
                        };
                    }
                    // find neighbor empty blocks
                    if i > 0 {
                        check_col!(i - 1);
                    }
                    check_col!(0);
                    if i + 1 < m {
                        check_col!(i + 1);
                    }
                    // all neighbors are empty
                    match mine_count {
                        0 => {
                            board[i][j] = Blocks::RevealedBlank;
                            for item in neighbor_empty.into_iter() {
                                q.push_back(item);
                            }
                        }
                        1..=8 => {
                            board[i][j] = ('0' as u8 + mine_count as u8) as char;
                        }
                        _ => unreachable!(),
                    }
                }
                '1'..='8' | Blocks::RevealedBlank => {
                    // pass
                    eprintln!("char is {}", board[i][j]);
                }
                _ => {
                    eprintln!("char is {}", board[i][j]);
                    unreachable!()
                }
            }
            //
        }

        board
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($board:tt, $click:tt, $ans:tt) => {
            let board = vec2d!$board;
            let ans = vec2d!$ans;
            assert_eq!(
                Solution::update_board(board, vec!$click),
                ans
            )
        };
    }
    test!(
        [
            ['E', 'E', 'E', 'E', 'E'],
            ['E', 'E', 'M', 'E', 'E'],
            ['E', 'E', 'E', 'E', 'E'],
            ['E', 'E', 'E', 'E', 'E']
        ],
        [3, 0],
        [
            ['B', '1', 'E', '1', 'B'],
            ['B', '1', 'M', '1', 'B'],
            ['B', '1', '1', '1', 'B'],
            ['B', 'B', 'B', 'B', 'B']
        ]
    );
    test!(
        [
            ['B', '1', 'E', '1', 'B'],
            ['B', '1', 'M', '1', 'B'],
            ['B', '1', '1', '1', 'B'],
            ['B', 'B', 'B', 'B', 'B']
        ],
        [1, 2],
        [
            ['B', '1', 'E', '1', 'B'],
            ['B', '1', 'X', '1', 'B'],
            ['B', '1', '1', '1', 'B'],
            ['B', 'B', 'B', 'B', 'B']
        ]
    );
}
