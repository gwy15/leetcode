/*
 * @lc app=leetcode.cn id=722 lang=rust
 *
 * [722] 删除注释
 */
struct Solution;
// @lc code=start
#[derive(Debug)]
enum State {
    Normal,
    BlockComment,
}

impl Solution {
    #[allow(unused)]
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut ans = Vec::new();
        let mut state = State::Normal;
        let mut buf = String::new();
        macro_rules! push {
            () => {
                if buf.len() > 0 {
                    ans.push(buf);
                    buf = String::new();
                }
            };
        }
        for line in source.into_iter() {
            let mut iter = line.chars().peekable();
            'line: loop {
                match iter.next() {
                    None => {
                        if let State::Normal = state {
                            // end of line, push
                            push!();
                        }
                        break 'line;
                    }
                    Some(c) => {
                        match state {
                            State::Normal => {
                                if c == '/' {
                                    match *iter.peek().unwrap_or(&' ') {
                                        '/' => {
                                            // inline, ignore rest and enter next line
                                            push!();
                                            break 'line;
                                        }
                                        '*' => {
                                            // block comment
                                            iter.next();
                                            state = State::BlockComment;
                                        }
                                        _ => {
                                            buf.push(c);
                                        }
                                    }
                                } else {
                                    buf.push(c);
                                }
                            }
                            State::BlockComment => {
                                if c == '*' && *iter.peek().unwrap_or(&' ') == '/' {
                                    // end block comment
                                    iter.next();
                                    state = State::Normal;
                                }
                                // else ignore
                            }
                        }
                    }
                }
            }
        }
        push!();

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($input:tt, $ans:tt) => {
            let a = (vec!$input).iter().map(|s|s.to_string()).collect();
            let b: Vec<String> = (vec!$ans).iter().map(|s|s.to_string()).collect();
            assert_eq!(
                Solution::remove_comments(a),
                b
            );
        }
    };
    test!(
        [
            "/*Test program */",
            "int main()",
            "{ ",
            "  // variable declaration ",
            "int a, b, c;",
            "/* This is a test",
            "   multiline  ",
            "   comment for ",
            "   testing */",
            "a = b + c;",
            "}"
        ],
        ["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"]
    );
    test!(["a/*comment", "line", "more_comment*/b"], ["ab"]);
    test!(
        [
            "struct Node{",
            "    /*/ declare members;/**/",
            "    int size;",
            "    /**/int val;",
            "};"
        ],
        [
            "struct Node{",
            "    ",
            "    int size;",
            "    int val;",
            "};"
        ]
    );
}
