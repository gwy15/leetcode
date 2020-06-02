struct Solution;

// code start
impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        // find all odd factors
        for n in 1..=target {
            if n & 1 == 0 || target % n != 0 {
                continue;
            }
            // two type,
            // A, length=n, odd
            if n != 1 {
                let mid = target / n;
                let start = mid - (n - 1) / 2;
                if start > 0 {
                    let end = mid + (n - 1) / 2;
                    ans.push((start..=end).collect());
                }
            }
            // B, length = 2 * target / n, even
            let length = 2 * target / n;
            let mid = (n - 1) / 2;
            let start = mid - ((length / 2) - 1);
            if start > 0 {
                let end = mid + (length / 2);
                ans.push((start..=end).collect());
            }
        }
        ans.sort_unstable_by_key(|v: &Vec<i32>| v[0]);
        ans
    }
}
//
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $ans:tt) => {
            assert_eq!(Solution::find_continuous_sequence($n), vec2d!$ans);
        };
    };
    test!(9, [[2, 3, 4], [4, 5]]);
    test!(15, [[1, 2, 3, 4, 5], [4, 5, 6], [7, 8]]);
    test!(10, [[1, 2, 3, 4]]);
}
