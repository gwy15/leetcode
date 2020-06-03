/*
 * @lc app=leetcode.cn id=836 lang=rust
 *
 * [836] 矩形重叠
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let (x1_a, y1_a, x2_a, y2_a) = (rec1[0], rec1[1], rec1[2], rec1[3]);
        let (x1_b, y1_b, x2_b, y2_b) = (rec2[0], rec2[1], rec2[2], rec2[3]);
        macro_rules! is_pair {
            ($x1:expr, $x2:expr) => {
                $x1 < $x2
            };
        }

        // x overlapped?
        let x_overlapped = is_pair!(x1_a, x2_b) && is_pair!(x1_b, x2_a);
        if !x_overlapped {
            return false;
        }

        let y_overlapped = is_pair!(y1_a, y2_b) && is_pair!(y1_b, y2_a);
        y_overlapped
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($rec1:tt, $rec2:tt, $ans:expr) => {
            assert_eq!(
                Solution::is_rectangle_overlap(vec!$rec1, vec!$rec2),
                $ans
            );
        }
    };
    test!([0, 0, 2, 2], [1, 1, 3, 3], true);
    test!([0, 0, 1, 1], [1, 0, 2, 1], false);
    test!(
        [-257926405, -680763313, 702840196, 454409669],
        [-275916328, -417802221, 22808107, 675629469],
        true
    );
}
