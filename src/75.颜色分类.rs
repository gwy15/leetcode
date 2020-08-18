/*
 * @lc app=leetcode.cn id=75 lang=rust
 *
 * [75] 颜色分类
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut ptr0, mut ptr1) = (0, 0);
        for i in 0..nums.len() {
            match nums[i] {
                0 => {
                    nums[ptr1] = 1;
                    ptr1 += 1;
                    nums[ptr0] = 0;
                    ptr0 += 1;
                }
                1 => {
                    nums[ptr1] = 1;
                    ptr1 += 1;
                }
                2 => {
                    // pass
                }
                _ => unreachable!(),
            }
        }
        for i in ptr1..nums.len() {
            nums[i] = 2;
        }
    }

    #[allow(unused)]
    pub fn sort_colors_simple(nums: &mut Vec<i32>) {
        let (mut cnt0, mut cnt1, mut cnt2) = (0, 0, 0);
        for n in nums.iter() {
            match n {
                0 => cnt0 += 1,
                1 => cnt1 += 1,
                2 => cnt2 += 1,
                _ => unreachable!(),
            }
        }
        let mut i = 0;
        for _ in 0..cnt0 {
            nums[i] = 0;
            i += 1;
        }
        for _ in 0..cnt1 {
            nums[i] = 1;
            i += 1;
        }
        for _ in 0..cnt2 {
            nums[i] = 2;
            i += 1;
        }
    }
}
// @lc code=end
