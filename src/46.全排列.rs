/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * [46] 全排列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        match nums.len() {
            0 => return Vec::new(),
            1 => return vec![nums],
            2 => return vec![vec![nums[1], nums[0]], nums],
            _ => {}
        };

        let n = nums.len();
        let mut ans = Vec::new();
        for i in 0..n {
            nums.swap(0, i);
            let rest: Vec<i32> = nums[1..n].iter().map(|&x| x).collect();
            for mut v in Solution::permute(rest) {
                v.push(nums[0]);
                ans.push(v);
            }
        }
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
    }
    macro_rules! test {
        ($v:tt, $a:tt) => {
            assert_eq!(
                Solution::permute(vec!$v),
                vec2d!$a
            );
        }
    };
    test!(
        [1, 2, 3],
        [
            [3, 2, 1],
            [2, 3, 1],
            [3, 1, 2],
            [1, 3, 2],
            [2, 1, 3],
            [1, 2, 3]
        ]
    );
}
