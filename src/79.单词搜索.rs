/*
 * @lc app=leetcode.cn id=79 lang=rust
 *
 * [79] 单词搜索
 */
struct Solution;
// @lc code=start
impl Solution {
    fn search(
        board: &Vec<Vec<char>>,
        word: &[char],
        visited: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
    ) -> bool {
        let (m, n) = (board.len(), board[0].len());
        // end
        if word.len() == 0 {
            return true;
        }
        macro_rules! check {
            ($cond:expr, $i:expr, $j:expr) => {
                if $cond && !visited[$i][$j] {
                    if Self::search(board, &word[1..], visited, $i, $j) {
                        return true;
                    }
                }
            };
        }
        // check if (i, j) matches
        if board[i][j] == word[0] {
            if word.len() == 1 {
                return true;
            }
            visited[i][j] = true;
            check!(i > 0, i - 1, j);
            check!(j > 0, i, j - 1);
            check!(i + 1 < m, i + 1, j);
            check!(j + 1 < n, i, j + 1);
            visited[i][j] = false;
        }
        return false;
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let (m, n) = (board.len(), board[0].len());
        let word: Vec<char> = word.chars().collect();
        let mut visited = vec![vec![false; n]; m];

        // search
        for i in 0..m {
            for j in 0..n {
                if Self::search(&board, &word, &mut visited, i, j) {
                    return true;
                }
            }
        }

        false
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($board:tt, $word:expr, $ans:expr) => {
            let board = (vec!$board).into_iter().map(|s| s.chars().collect()).collect();
            assert_eq!(
                Solution::exist(board, $word.into()),
                $ans
            )
        };
    }
    test!(["ABCE", "SFCS", "ADEE"], "ABCCED", true);
    test!(["ABCE", "SFCS", "ADEE"], "SEE", true);
    test!(["ABCE", "SFCS", "ADEE"], "ABCB", false);
    test!(["a"], "", true);
    test!(["a"], "a", true);
}
