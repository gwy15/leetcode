struct Solution;
impl Solution {
    pub fn ways_to_change(n: i32) -> i32 {
        const MOD: usize = 1_000_000_007;
        const coins: [usize; 3] = [5, 10, 25];

        let n = n as usize;
        let mut dp = vec![1; n + 1];
        dp[0] = 1;
        for &coin in coins.iter() {
            for i in coin..=n {
                dp[i] = (dp[i] + dp[i - coin]) % MOD;
            }
        }
        dp[n] as i32
    }
}
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $a:expr) => {
            assert_eq!(Solution::ways_to_change($n), $a);
        };
    };
    test!(5, 2);
    test!(10, 4);
}
