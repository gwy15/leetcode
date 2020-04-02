/*
 * @lc app=leetcode.cn id=289 lang=rust
 *
 * [289] 生命游戏
 */

struct Solution;

// @lc code=start
impl Solution {
    fn transfer_from_dead(around: i32) -> i32 {
        match around {
            3 => 1,
            _ => 0,
        }
    }

    fn transfer_from_live(around: i32) -> i32 {
        match around {
            2..=3 => 1,
            _ => 0,
        }
    }

    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        // count thru board
        for i in 0..m {
            for j in 0..n {
                let mut count = 0;
                // count neighbors
                for di in [-1, 0, 1].iter() {
                    let ii = i + di;
                    if ii < 0 || ii >= m {
                        continue;
                    }
                    for dj in [-1, 0, 1].iter() {
                        let jj = j + dj;
                        if jj < 0 || jj >= n {
                            continue;
                        }
                        if board[ii as usize][jj as usize] & 1 == 1 {
                            count += 1;
                        }
                    }
                }
                count -= board[i as usize][j as usize];
                board[i as usize][j as usize] += count << 16;
            }
        }
        // update
        for i in 0..m as usize {
            for j in 0..n as usize {
                board[i][j] = match board[i][j] & 1 == 1 {
                    true => Solution::transfer_from_live(board[i][j] >> 16),
                    false => Solution::transfer_from_dead(board[i][j] >> 16),
                };
            }
        }
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! vec2d {
        [
            $(
                [$(
                    $i:expr
                ),*]
            ),*
        ] => {
            vec![$(
                vec![$($i),*]
            ),*]
        }
    };
    macro_rules! test {
        (
            $input:expr, $output:expr
        ) => {{
            let mut i = $input;
            Solution::game_of_life(&mut i);
            assert_eq!(i, $output)
        }};
    };
    test!(
        vec2d![[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]],
        vec2d![[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]]
    );
}
