/*
 * @lc app=leetcode.cn id=354 lang=rust
 *
 * [354] 俄罗斯套娃信封问题
 */
struct Solution;
// @lc code=start
impl Solution {
    fn longest_ascending_sequence(width: Vec<i32>) -> i32 {
        let mut endings = Vec::new();
        for a in width {
            match endings.binary_search(&a) {
                Ok(_found_index) => {} // do nothing
                Err(insert_index) => {
                    if insert_index == endings.len() {
                        endings.push(a);
                    } else {
                        endings[insert_index] = a;
                    }
                }
            }
        }
        endings.len() as i32
    }

    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        // convert into widths
        envelopes.sort_unstable_by_key(|v| (v[0], Reverse(v[1])));
        // get width
        let width: Vec<i32> = envelopes.into_iter().map(|v| v[1]).collect();
        println!("{:?}", width);

        Self::longest_ascending_sequence(width)
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(
                Solution::max_envelopes(vec2d!$args),
                $ans
            )
        };
    }
    test!([[5, 4], [6, 4], [6, 7], [2, 3]], 3);
    test!([[5, 4]], 1);
    test!([[4, 5], [4, 6], [6, 7], [2, 3], [1, 1]], 4);
}
