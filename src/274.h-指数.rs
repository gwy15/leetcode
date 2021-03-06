/*
 * @lc app=leetcode.cn id=274 lang=rust
 *
 * [274] H 指数
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        let n = citations.len();
        citations.sort();
        let (mut left, mut right) = (0, n + 1);
        while left < right - 1 {
            let k = left + (right - left) / 2;
            if citations[n - k] >= k as i32 {
                left = k;
            } else {
                right = k;
            }
        }
        left as i32
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($c:tt, $ans:expr) => {
            assert_eq!(
                Solution::h_index(vec!$c),
                $ans
            );
        }
    };
    test!([3, 0, 6, 1, 5], 3);
    test!([], 0);
    test!([1], 1);
    test!([2], 1);
}
