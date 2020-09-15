/*
 * @lc app=leetcode.cn id=37 lang=rust
 *
 * [37] 解数独
 */
struct Solution;
// @lc code=start
#[derive(Debug)]
struct State {
    row: [[bool; 9]; 9],
    col: [[bool; 9]; 9],
    block: [[bool; 9]; 9],
}
impl State {
    pub fn new() -> Self {
        Self {
            row: [[false; 9]; 9],
            col: [[false; 9]; 9],
            block: [[false; 9]; 9],
        }
    }
    fn block_mut(&mut self, i: usize, j: usize) -> &mut [bool; 9] {
        let idx = (i / 3) + 3 * (j / 3);
        &mut self.block[idx]
    }
    fn block(&self, i: usize, j: usize) -> &[bool; 9] {
        let idx = (i / 3) + 3 * (j / 3);
        &self.block[idx]
    }
    fn can_fill(&self, i: usize, j: usize, num: usize) -> bool {
        let idx = num - 1;
        !(self.row[i][idx] || self.col[j][idx] || self.block(i, j)[idx])
    }
}

impl Solution {
    pub fn dfs(board: &mut Vec<Vec<usize>>, empties: &[(usize, usize)], state: &mut State) -> bool {
        if empties.len() == 0 {
            // solution found
            return true;
        }
        // find next position
        let (i, j) = empties[0];
        for fill in 1..=9 {
            if state.can_fill(i, j, fill) {
                board[i][j] = fill;
                // update state
                state.row[i][fill - 1] = true;
                state.col[j][fill - 1] = true;
                state.block_mut(i, j)[fill - 1] = true;
                if Self::dfs(board, &empties[1..], state) {
                    return true;
                }
                // clear state
                state.row[i][fill - 1] = false;
                state.col[j][fill - 1] = false;
                state.block_mut(i, j)[fill - 1] = false;
            }
        }
        return false;
    }

    #[allow(unused)]
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut board_num = vec![vec![0; 9]; 9];
        let mut empties = vec![];
        let mut state = State::new();
        for i in 0..9 {
            for j in 0..9 {
                match board[i][j] {
                    '1'..='9' => {
                        let num = (board[i][j] as u8 - '0' as u8) as usize;
                        board_num[i][j] = num;
                        state.row[i][num - 1] = true;
                        state.col[j][num - 1] = true;
                        state.block_mut(i, j)[num - 1] = true;
                    }
                    '.' => {
                        board_num[i][j] = 0;
                        empties.push((i, j));
                    }
                    _ => unreachable!(),
                }
            }
        }
        Self::dfs(&mut board_num, &empties, &mut state);

        for i in 0..9 {
            for j in 0..9 {
                board[i][j] = (board_num[i][j] as u8 + '0' as u8) as char;
            }
        }
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:tt) => {
            let board = vec_string!$args;
            let ans = vec_string!$ans;
            let mut board = board.into_iter().map(|s| s.chars().collect()).collect();
            let ans: Vec<Vec<char>> = ans.into_iter().map(|s| s.chars().collect()).collect();
            Solution::solve_sudoku(&mut board);
            for line in board.iter() {
                for ch in line.iter() {
                    print!("{}", ch);
                }
                println!();
            }
            assert_eq!(
                board, ans
            );
        };
    }
    test!(
        [
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79"
        ],
        [
            "534678912",
            "672195348",
            "198342567",
            "859761423",
            "426853791",
            "713924856",
            "961537284",
            "287419635",
            "345286179"
        ]
    );
}
