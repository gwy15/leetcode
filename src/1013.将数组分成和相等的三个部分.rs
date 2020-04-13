/*
 * @lc app=leetcode.cn id=1013 lang=rust
 *
 * [1013] 将数组分成和相等的三个部分
 */

struct Solution;
// @lc code=start
impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let sum = a.iter().fold(0, |x, y| x + y);
        if sum % 3 != 0 {
            return false;
        }
        let expected_part_sum = sum / 3;

        let mut part_sum = 0;
        let mut parts = 0;
        for n in a.iter() {
            part_sum += n;
            if part_sum == expected_part_sum {
                part_sum = 0;
                parts += 1;
                if parts == 3 {
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
        ($v:tt, $ans:expr) => {
            assert_eq!(
                Solution::can_three_parts_equal_sum(vec!$v),
                $ans
            );
        }
    };
    test!([0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1], true);
    test!([0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1], false);
    test!([3, 3, 6, 5, -2, 2, 5, 1, -9, 4], true);
    test!([1, -1, 1, -1], false);
}
