/*
 * @lc app=leetcode.cn id=1177 lang=rust
 *
 * [1177] 构建回文串检测
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // preprocessing
        let mut pre_sum: Vec<i32> = vec![0; s.len() + 1];
        for (i, b) in s.into_bytes().into_iter().enumerate() {
            let bit = b - b'a'; // 0..=26
            pre_sum[i + 1] = pre_sum[i] ^ (1 << bit);
        }

        let mut ans = Vec::with_capacity(queries.len());
        // query
        for query in queries.into_iter() {
            let (left, right, k) = (query[0] as usize, query[1] as usize, query[2] as u32);

            let diff = (pre_sum[right + 1] ^ pre_sum[left]).count_ones();
            ans.push(k >= diff / 2);
        }
        ans
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $q:tt, $ans:tt) => {
            assert_eq!(
                Solution::can_make_pali_queries($s.into(), vec2d!$q),
                vec!$ans
            );
        }
    };
    test!(
        "abcda",
        [[3, 3, 0], [1, 2, 0], [0, 3, 1], [0, 3, 2], [0, 4, 1]],
        [true, false, false, true, true]
    );
}
