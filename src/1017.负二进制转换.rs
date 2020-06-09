/*
 * @lc app=leetcode.cn id=1017 lang=rust
 *
 * [1017] 负二进制转换
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        let mut s: Vec<char> = vec![];

        let mut i = 0;
        // let mut base = 1;
        while n > 0 {
            // base *= 2;
            let digit = n % 2;
            s.push(('0' as u8 + digit as u8) as char);
            if i % 2 == 0 {
                // 偶数位，(-2)^2k，正常操作
                n /= 2;
            } else {
                // 奇数位，(-2)^(2k+1)
                n += 1; // 反向操作似神仙
                n /= 2;
            }

            i += 1;
        }
        if s.len() == 0 {
            return "0".into();
        }

        s.into_iter().rev().collect()
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $ans:expr) => {
            assert_eq!(Solution::base_neg2($n), $ans);
        };
    };
    test!(2, "110");
    test!(3, "111");
    test!(4, "100");
    test!(8, "11000");
    test!(0, "0");
    // test!(-2, "10");
    // test!(-4, "1100");
}
