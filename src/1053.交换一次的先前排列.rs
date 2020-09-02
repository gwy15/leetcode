/*
 * @lc app=leetcode.cn id=1053 lang=rust
 *
 * [1053] 交换一次的先前排列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn prev_perm_opt1(mut a: Vec<i32>) -> Vec<i32> {
        let n = a.len();

        for i in (1..n).rev() {
            // find [i-1] > [i]
            if a[i - 1] > a[i] {
                // find the first j that is < a[i-1]
                let mut j = n - 1;
                while a[j] >= a[i - 1] {
                    j -= 1;
                }
                while a[j - 1] == a[j] {
                    j -= 1;
                }
                //
                a.swap(i - 1, j);
                break;
            }
        }

        a
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:tt) => {
            assert_eq!(Solution::prev_perm_opt1(vec!$args), vec!$ans);
        };
    }
    test!([3, 2, 1], [3, 1, 2]);
    test!([1, 2, 3], [1, 2, 3]);
    test!([1, 9, 4, 6, 7], [1, 7, 4, 6, 9]);
    test!([3, 1, 1, 3], [1, 3, 1, 3]);
    test!([1], [1]);
    test!([3, 1, 2], [2, 1, 3]);
    test!([1, 1], [1, 1]);
}
