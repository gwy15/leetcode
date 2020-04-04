/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */
struct Solution;

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut sum = 0;
        let n = height.len();
        if n == 0 {
            return 0;
        }
        // find max
        let max_height = *height.iter().max().unwrap();

        macro_rules! iter {
            ($range:expr) => {{
                let mut index = 0;
                let mut cur_max = 0;
                for i in $range {
                    let h = height[i];
                    cur_max = cur_max.max(h);
                    if cur_max < max_height {
                        sum += cur_max - h;
                    } else {
                        index = i;
                        break;
                    }
                }
                index
            }};
        }
        // left to right
        let left_max_index = iter!(0..n);
        // right to left
        let right_max_index = iter!((left_max_index + 1..n).rev());
        // middle, if any
        for i in left_max_index + 1..right_max_index {
            sum += max_height - height[i];
        }

        sum
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ([$($i:expr),*], $ans:expr) => {
            assert_eq!(
                Solution::trap(vec![$($i),*]),
                $ans
            );
        }
    };
    test!([], 0);
    test!([0], 0);
    test!([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6);
    test!([1, 2, 2, 1], 0);
}
