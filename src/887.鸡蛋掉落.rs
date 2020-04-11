/*
 * @lc app=leetcode.cn id=887 lang=rust
 *
 * [887] 鸡蛋掉落
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(non_snake_case)]
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let N = n as usize;
        let K = N.min(k as usize);
        let mut f_k1: Vec<i32> = (0..=n).collect();
        for _k in 2..=K {
            // f_k[n] = f(n, k)
            let mut f_k = vec![0; N + 1];

            // calculate f(N, K) = 1 + min_m(
            //     max(f_k1(m-1), f(N-m))
            // )
            let mut m = 1;
            for n in 1..=N {
                // calculating f(n, k) = 1 + min_m(
                //     max(f_k1[m-1], f_k[n - m])
                // )
                let f = |m| i32::max(f_k1[m - 1], f_k[n - m]);
                // increase m until it reaches best solution
                while m < n && f(m + 1) < f(m) {
                    m = m + 1;
                }
                f_k[n] = 1 + f(m);
            }
            // println!("{} eggs, steps={:?}", _k, f_k);
            // assign f_k1 to f_k
            f_k1 = f_k;
        }
        f_k1[N]
    }
}
// @lc code=end
#[test]
fn test_solution() {
    assert_eq!(Solution::super_egg_drop(1, 2), 2);
    assert_eq!(Solution::super_egg_drop(2, 6), 3);
    assert_eq!(Solution::super_egg_drop(3, 14), 4);
}
