/*
 * @lc app=leetcode.cn id=1041 lang=rust
 *
 * [1041] 困于环中的机器人
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn is_robot_bounded(instructions: String) -> bool {
        let direction: Vec<(i32, i32)> = vec![
            (0, 1),  // N
            (1, 0),  // East
            (0, -1), // South
            (-1, 0), // West
        ];
        let (mut x, mut y) = (0, 0);
        let mut dir = 0;
        for ch in instructions.chars() {
            match ch {
                'G' => {
                    x += direction[dir].0;
                    y += direction[dir].1;
                }
                'L' => {
                    if dir == 0 {
                        dir = 3;
                    } else {
                        dir -= 1;
                    }
                }
                'R' => {
                    dir += 1;
                    dir %= 4;
                }
                _ => unreachable!(),
            }
        }
        let divergence = (x, y) != (0, 0) && dir == 0;
        !divergence
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $a:expr) => {
            assert_eq!(Solution::is_robot_bounded($s.into()), $a);
        };
    };
    test!("GGLLGG", true);
    test!("GG", false);
    test!("", true);
    test!("GL", true);
}
