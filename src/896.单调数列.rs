/*
 * @lc app=leetcode.cn id=896 lang=rust
 *
 * [896] 单调数列
 */
struct Solution;
// @lc code=start
enum Status {
    Unknown,
    Ascending,
    Descending,
}
impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        use std::cmp::Ordering;
        let mut status = Status::Unknown;
        for i in 0..a.len() - 1 {
            match status {
                Status::Unknown => {
                    status = match a[i].cmp(&a[i + 1]) {
                        Ordering::Less => Status::Ascending,
                        Ordering::Equal => Status::Unknown,
                        Ordering::Greater => Status::Descending,
                    }
                }
                Status::Ascending => {
                    if a[i] > a[i + 1] {
                        return false;
                    }
                }
                Status::Descending => {
                    if a[i] < a[i + 1] {
                        return false;
                    }
                }
            }
        }
        true
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $a:expr) => {
            assert_eq!(
                Solution::is_monotonic(vec!$v),
                $a
            );
        }
    };
    test!([1, 2, 2, 3], true);
    test!([6, 5, 4, 4], true);
    test!([1, 3, 2], false);
    test!([1, 2, 4, 5], true);
    test!([1, 1, 1], true);
}
