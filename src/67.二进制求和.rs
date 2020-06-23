/*
 * @lc app=leetcode.cn id=67 lang=rust
 *
 * [67] 二进制求和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a: Vec<_> = a.chars().map(|c| c as u8 - '0' as u8).collect();
        let mut b: Vec<_> = b.chars().map(|c| c as u8 - '0' as u8).collect();
        if a.len() > b.len() {
            std::mem::swap(&mut a, &mut b);
        }
        // a.len < b.len

        let mut ans = vec![0_u8; a.len().max(b.len()) + 1];
        let ans_len = ans.len();
        // copy b
        &ans[ans_len - b.len()..].copy_from_slice(&b[..]);
        // add a
        let mut last = 0;
        for i in 0..ans_len {
            let mut digit = ans[ans_len - 1 - i] + last;
            if i < a.len() {
                digit += a[a.len() - 1 - i];
            }
            last = digit / 2;
            ans[ans_len - 1 - i] = digit % 2;
        }

        // remove prefix 0s
        let mut i = 0;
        while i < ans.len() && ans[i] == 0 {
            i += 1;
        }
        if i == ans.len() {
            "0".into()
        } else {
            ans[i..]
                .into_iter()
                .map(|digit| (digit + '0' as u8) as char)
                .collect()
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($a:expr, $b:expr, $ans:expr) => {
            assert_eq!(Solution::add_binary($a.into(), $b.into()), $ans);
        };
    };
    test!("11", "1", "100");
    test!("1010", "1011", "10101");
    test!("1", "1", "10");
    test!("0", "1", "1");
    test!("0", "0", "0");
}
