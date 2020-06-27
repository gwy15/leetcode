/*
 * @lc app=leetcode.cn id=1140 lang=rust
 *
 * [1140] 石子游戏 II
 */
struct Solution;
// @lc code=start
#[allow(unused)]
impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();

        let mut post_sum = vec![0; n + 1];
        for i in (0..n).rev() {
            post_sum[i] = post_sum[i + 1] + piles[i];
        }

        let mut dp = vec![vec![0; n + 1]; n];
        for i in (0..n).rev() {
            for m in (1..=n).rev() {
                // 可以拿到 [i, i + x]，其中 x ∈ [1, 2m]
                if i + 2 * m >= n {
                    dp[i][m] = post_sum[i];
                    continue;
                }
                let opponent_worst = (1..=2 * m).map(|x| dp[i + x][x.max(m)]).min().unwrap();
                dp[i][m] = post_sum[i] - opponent_worst;
            }
        }
        dp[0][1]
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($piles:tt, $ans:expr) => {
            assert_eq!(
                Solution::stone_game_ii(vec!$piles),
                $ans
            );
        }
    };
    test!([2, 7, 9, 4, 4], 10);
    test!([10], 10);
    test!([0, 0, 0, 100, 0], 100);
    test!([0, 0, 100], 0);
    test!([9, 2, 2, 8, 3, 7, 9, 9], 29);
    test!(
        [
            7468, 6245, 9261, 3958, 1986, 1074, 5677, 9386, 1408, 1384, 8811, 3885, 9678, 8470,
            8893, 7514, 4941, 2148, 5217, 5425, 5307, 747, 1253, 3518, 5238, 5834, 9133, 8391,
            6100, 3362, 7807, 2581, 6121, 7684, 8744, 9584, 4068, 7204, 4285, 8635
        ],
        115357
    );
}
