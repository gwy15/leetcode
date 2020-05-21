/*
 * @lc app=leetcode.cn id=747 lang=rust
 *
 * [747] 至少是其他数字两倍的最大数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => return -1,
            1 => return 0,
            _ => {}
        };
        let mut max = vec![(0, nums[0]), (1, nums[1])];
        if max[0].1 < max[1].1 {
            max.swap(0, 1);
        }

        for (i, &n) in nums.iter().enumerate().skip(2) {
            if n >= max[0].1 {
                max[1] = (i, n);
                max.swap(0, 1);
            } else if n > max[1].1 {
                max[1] = (i, n);
            }
        }
        if max[0].1 >= 2 * max[1].1 {
            return max[0].0 as i32;
        } else {
            return -1;
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($nums:tt, $ans:expr) => {
            assert_eq!(
                Solution::dominant_index(vec!$nums),
                $ans
            );
        }
    };
    test!([3, 6, 1, 0], 1);
    test!([1, 2, 3, 4], -1);
    test!([1], 0);
    test!([], -1);
}
