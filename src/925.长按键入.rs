/*
 * @lc app=leetcode.cn id=925 lang=rust
 *
 * [925] 长按键入
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut name = name.into_bytes();
        let mut typed = typed.into_bytes();

        let (m, n) = (name.len(), typed.len());
        let (mut i, mut j) = (0, 0);
        while i < m && j < n {
            // get char group
            let ch = name[i];
            let mut cnt = 0;
            while i < m && name[i] == ch {
                i += 1;
                cnt += 1;
            }
            let mut cnt2 = 0;
            while j < n && typed[j] == ch {
                j += 1;
                cnt2 += 1;
            }
            if cnt2 < cnt {
                return false;
            }
        }

        i == m && j == n
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $typed:expr, $ans:expr) => {
            assert_eq!(
                Solution::is_long_pressed_name($n.to_string(), $typed.to_string()),
                $ans
            );
        };
    }
    test!("alex", "aaleex", true);
    test!("saeed", "saaedd", false);
    test!("leelee", "lleeelee", true);
    test!("", "", true);
    test!("", "a", false);
    test!("laiden", "laiden", true);
    test!("pyplrz", "ppyyplr", false);
}
