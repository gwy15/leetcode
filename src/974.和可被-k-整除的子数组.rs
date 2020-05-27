/*
 * @lc app=leetcode.cn id=974 lang=rust
 *
 * [974] 和可被 K 整除的子数组
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        // K <= 10_000
        let mut counter = vec![0; k as usize];
        counter[0] = 1;
        let mut ans = 0;
        //
        let mut sum = 0;
        for n in a.into_iter() {
            sum = (sum + n) % k;
            sum = (sum + k) % k; // guarantee sum >= 0
            ans += counter[sum as usize];
            counter[sum as usize] += 1;
        }

        ans
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($a:tt,$k:expr, $ans:expr) => {
            assert_eq!(
                Solution::subarrays_div_by_k(vec!$a, $k),
                $ans
            );
        }
    };
    test!([4, 5, 0, -2, -3, 1], 5, 7);
    test!([7, 4, -10], 5, 1);
}
