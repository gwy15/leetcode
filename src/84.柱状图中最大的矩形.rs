/*
 * @lc app=leetcode.cn id=84 lang=rust
 *
 * [84] 柱状图中最大的矩形
 */
struct Solution;
// @lc code=start
impl Solution {
    #[allow(unused)]
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();

        // 确定左边界
        let mut left = vec![0; n];
        let mut stack = vec![];
        for i in 0..n {
            let height = heights[i];
            // 找到小于 height 的位置
            while stack.len() > 0 && height <= heights[*stack.last().unwrap()] {
                stack.pop();
            }
            left[i] = stack.last().map(|&x| x as i32).unwrap_or(-1);
            stack.push(i);
        }
        // 确定右边界
        let mut right = vec![0; n];
        stack = vec![];
        for i in (0..n).rev() {
            let height = heights[i];
            while stack.len() > 0 && height <= heights[*stack.last().unwrap()] {
                stack.pop();
            }
            right[i] = stack.last().map(|&x| x as i32).unwrap_or(n as i32);
            stack.push(i);
        }

        (0..n).fold(0, |ans, i| {
            let area = (right[i] - left[i] - 1) * heights[i];
            ans.max(area)
        })
    }

    pub fn largest_rectangle_area2(mut heights: Vec<i32>) -> i32 {
        heights.push(0);
        let mut ans = 0;
        let mut stack = vec![];
        stack.push((-1, 0));
        for (i, h) in heights.into_iter().enumerate() {
            loop {
                let (_, last_h) = stack.last().unwrap();
                if *last_h > h {
                    // pop
                    let (_, height) = stack.pop().unwrap();
                    // (left_i, <height) ... (_,height) ... (i,<height)
                    //
                    let (left_i, _) = stack.last().unwrap();
                    ans = ans.max((i as i32 - left_i - 1) * height);
                } else {
                    break;
                }
            }
            stack.push((i as i32, h));
        }
        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($h:tt, $ans:expr) => {
            assert_eq!(
                Solution::largest_rectangle_area(vec!$h),
                $ans
            );
        }
    };
    test!([2, 1, 5, 6, 2, 3], 10);
    test!([1], 1);
    test!([], 0);
}
