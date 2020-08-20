/*
 * @lc app=leetcode.cn id=985 lang=rust
 *
 * [985] 查询后的偶数和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn sum_even_after_queries(mut a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answers = Vec::with_capacity(queries.len());
        let mut even_sum = a.iter().filter(|&x| x % 2 == 0).sum::<i32>();
        for query in queries {
            let (delta, idx) = (query[0], query[1] as usize);
            let val = a[idx];
            match (val & 1, delta & 1) {
                (0, 0) => {
                    // even to even
                    even_sum += delta;
                }
                (1, 1) => {
                    // odd to even
                    even_sum += val + delta;
                }
                (0, 1) => {
                    // even to odd
                    even_sum -= val;
                }
                (1, 0) => {
                    // odd to odd
                }
                _ => {
                    eprintln!("val={}, delta={}", val, delta);
                    unreachable!()
                }
            }
            a[idx] += delta;
            answers.push(even_sum);
        }
        answers
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($a:tt, $q:tt, $ans:tt) => {
            assert_eq!(
                Solution::sum_even_after_queries(vec!$a, vec2d!$q),
                vec!$ans
            );
        };
    }
    test!(
        [1, 2, 3, 4],
        [[1, 0], [-3, 1], [-4, 0], [2, 3]],
        [8, 6, 2, 4]
    );
}
