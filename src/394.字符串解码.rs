/*
 * @lc app=leetcode.cn id=394 lang=rust
 *
 * [394] 字符串解码
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(i32, String)> = Vec::new();
        // root stack
        stack.push((1, "".into()));
        let mut parsing_num = false;
        for ch in s.chars() {
            match ch {
                '0'..='9' => {
                    if !parsing_num {
                        // push new stack
                        stack.push((0, "".into()));
                        parsing_num = true;
                    }
                    let n: &mut i32 = &mut stack.last_mut().unwrap().0;
                    *n *= 10;
                    *n += ch as i32 - '0' as i32;
                }
                '[' => {
                    // starting to parse expression
                    parsing_num = false;
                }
                ']' => {
                    // pop stack
                    let (times, substring) = stack.pop().unwrap();
                    stack.last_mut().unwrap().1 += &substring.repeat(times as usize);
                }
                _ => {
                    // add character to string
                    stack.last_mut().unwrap().1.push(ch);
                }
            }
        }

        stack[0].1.clone()
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $ans:expr) => {
            assert_eq!(Solution::decode_string($s.into()), $ans.to_string());
        };
    };
    test!("3[a]2[bc]", "aaabcbc");
    test!("3[a2[c]]", "accaccacc");
    test!("2[abc]3[cd]ef", "abcabccdcdcdef");
    test!("abc", "abc");
    test!("abc2[fff]", "abcffffff");
}
