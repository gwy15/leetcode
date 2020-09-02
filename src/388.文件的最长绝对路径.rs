/*
 * @lc app=leetcode.cn id=388 lang=rust
 *
 * [388] 文件的最长绝对路径
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut stack = Vec::new();
        let mut cur_len = 0;
        let mut max_len = 0;
        for entry in input.split('\n') {
            let mut level = 1;
            let mut chars = entry.chars().peekable();
            while let Some('\t') = chars.peek() {
                level += 1;
                chars.next();
            }
            let name: String = chars.collect();
            //
            while stack.len() >= level {
                let last = stack.pop().unwrap();
                cur_len -= last + 1;
            }
            // is file
            if name.find('.').is_some() {
                let length = cur_len + name.len();
                max_len = max_len.max(length);
            } else {
                stack.push(name.len());
                cur_len += name.len() + 1;
            }
        }
        max_len as i32
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:expr, $ans:expr) => {
            assert_eq!(Solution::length_longest_path($args.to_string()), $ans)
        };
    }
    test!(
        "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext",
        32
    );
    test!("dir", 0);
    test!("dir/a.jpg", 9);
}
