/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return Vec::new();
        }
        let mut ans = Vec::new();
        // sort by left
        intervals.sort();

        let (mut left, mut right) = (intervals[0][0], intervals[0][1]);
        for seg in intervals.into_iter() {
            let (l, r) = (seg[0], seg[1]);
            if l <= right {
                right = right.max(r);
            } else {
                ans.push(vec![left, right]);
                left = l;
                right = r;
            }
        }
        ans.push(vec![left, right]);
        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! vec2d {
        [$(
            [$(
                $i:expr
            ),*]
        ),*] => {
            vec![$(
                vec![$($i),*]
            ),*]
        }
    };
    macro_rules! test {
        ($v:expr, $a:expr) => {
            assert_eq!(Solution::merge($v), $a);
        };
    };
    test!(
        vec2d![[1, 3], [2, 6], [8, 10], [15, 18]],
        vec2d![[1, 6], [8, 10], [15, 18]]
    );
    test!(vec2d![[1, 4], [4, 5]], vec2d![[1, 5]]);
    test!(vec2d![[1, 1]], vec2d![[1, 1]]);
    test!(Vec::new(), Vec::<Vec<i32>>::new());
}
