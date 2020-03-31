/*
 * @lc app=leetcode.cn id=962 lang=rust
 *
 * [962] 最大宽度坡
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_width_ramp(a: Vec<i32>) -> i32 {
        let n = a.len();
        // make decreasing stack
        let mut stack = {
            let mut stack = Vec::new();
            let mut head = a[0];
            stack.push((0, a[0]));
            for (i, &v) in a.iter().enumerate() {
                if v < head {
                    stack.push((i, v));
                    head = v;
                }
            }
            stack
        };

        let mut max_ramp = 0;
        let tail = i32::min_value();
        for j in (0..n).rev() {
            if a[j] <= tail {
                continue;
            }
            let tail = a[j];
            // pop until value > tail
            while !stack.is_empty() {
                let (i, head) = stack[stack.len() - 1];
                if head <= tail {
                    max_ramp = max_ramp.max(j - i);
                    stack.pop();
                } else {
                    break;
                }
            }
        }
        max_ramp as i32
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ([$($i:expr),*], $ans:expr) => {
            assert_eq!(
                Solution::max_width_ramp(vec![$($i),*]),
                $ans
            );
        };
    };
    test!([1], 0);
    test!([1, 2], 1);
    test!([2, 1], 0);
    test!([6, 0, 8, 2, 1, 5], 4);
    test!([9, 8, 1, 0, 1, 9, 4, 0, 4, 1], 7);
}
