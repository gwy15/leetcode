/*
 * @lc app=leetcode.cn id=717 lang=rust
 *
 * [717] 1比特与2比特字符
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let n = bits.len();
        let mut i = 0;
        while i < n - 1 {
            i += bits[i] as usize + 1;
        }
        i == n - 1
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($b:tt, $a:expr) => {
            assert_eq!(
                Solution::is_one_bit_character(vec!$b),
                $a
            );
        }
    };
    test!([1, 0, 0], true);
    test!([1, 1, 1, 0], false);
    test!([0], true);
    test!([0, 0], true);
}
