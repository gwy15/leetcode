/*
 * @lc app=leetcode.cn id=794 lang=rust
 *
 * [794] 有效的井字游戏
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let chars: Vec<Vec<_>> = board
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect();
        let x_count: i32 = chars
            .iter()
            .map(|r| r.iter().filter(|&&c| c == 'X').count() as i32)
            .sum();
        let o_count: i32 = chars
            .iter()
            .map(|r| r.iter().filter(|&&c| c == 'O').count() as i32)
            .sum();
        dbg!(x_count, o_count);
        if (x_count != o_count) && (x_count - o_count != 1) {
            return false;
        }

        macro_rules! success {
            ($ty:literal, $x:literal, $y:literal, $offset_x:literal, $offset_y:literal) => {{
                chars[$x][$y] == $ty
                    && chars[($x + $offset_x) as usize][($y + $offset_y) as usize] == $ty
                    && chars[($x + 2 * $offset_x) as usize][($y + 2 * $offset_y) as usize] == $ty
            }};
        }
        let x_success = [
            //
            success!('X', 0, 0, 1, 0),
            success!('X', 0, 1, 1, 0),
            success!('X', 0, 2, 1, 0),
            //
            success!('X', 0, 0, 0, 1),
            success!('X', 1, 0, 0, 1),
            success!('X', 2, 0, 0, 1),
            //
            success!('X', 0, 0, 1, 1),
            success!('X', 2, 0, -1i32, 1),
        ]
        .iter()
        .filter(|&&b| b)
        .count();
        let o_success = [
            //
            success!('O', 0, 0, 1, 0),
            success!('O', 0, 1, 1, 0),
            success!('O', 0, 2, 1, 0),
            //
            success!('O', 0, 0, 0, 1),
            success!('O', 1, 0, 0, 1),
            success!('O', 2, 0, 0, 1),
            //
            success!('O', 0, 0, 1, 1),
            success!('O', 2, 0, -1i32, 1),
        ]
        .iter()
        .filter(|&&b| b)
        .count();
        dbg!(x_success, o_success);
        if x_success > 0 && o_success > 0 {
            return false;
        }

        // 如果先手胜，那一定有
        if x_success > 0 {
            return x_count == o_count + 1;
        }
        if o_success > 0 {
            return x_count == o_count;
        }

        true
    }
}
// @lc code=end
mod utils;

#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(Solution::valid_tic_tac_toe(vec_string!$args), $ans)
        };
    }
    test!(["O  ", "   ", "   "], false);
    test!(["XOX", " X ", "   "], false);
    test!(["XXX", "   ", "OOO"], false);
    test!(["XOX", "O O", "XOX"], true);
    test!(["XXX", "XOO", "OO "], false);
}
