/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 */
struct Solution;
// @lc code=start
impl Solution {
    fn helper(digits: &[i32]) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }

        let chars: &[char] = match digits[digits.len() - 1] {
            2 => &['a', 'b', 'c'],
            3 => &['d', 'e', 'f'],
            4 => &['g', 'h', 'i'],
            5 => &['j', 'k', 'l'],
            6 => &['m', 'n', 'o'],
            7 => &['p', 'q', 'r', 's'],
            8 => &['t', 'u', 'v'],
            9 => &['w', 'x', 'y', 'z'],
            _ => unreachable!(),
        };
        if digits.len() == 1 {
            return chars.iter().map(|c| c.to_string()).collect();
        } else {
            let mut result = vec![];
            let rest = Self::helper(&digits[..digits.len() - 1]);
            for s in rest {
                for &ch in chars {
                    let mut s = s.clone();
                    s.push(ch);
                    result.push(s);
                }
            }

            return result;
        }
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits: Vec<i32> = digits.chars().map(|c| (c as u8 - b'0') as i32).collect();
        Self::helper(&digits)
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:tt) => {
            let ans: Vec<&str> = vec!$ans;
            assert_eq!(
                Solution::letter_combinations($args.to_string()),
                ans
            )
        };
    }
    test!("23", ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
    test!("", []);
    test!("2", ["a", "b", "c"]);
}
