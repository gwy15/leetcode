/*
 * @lc app=leetcode.cn id=80 lang=rust
 *
 * [80] 删除排序数组中的重复项 II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut i = 0;
        let mut left = 0;
        for right in 0..=n {
            if right == n || nums[right] != nums[left] {
                // new number
                let length = right - left;
                // println!("left = {}, right = {}, length = {}", left, right, length);
                match length {
                    1 => {
                        nums[i] = nums[left];
                        i += 1;
                    }
                    _ => {
                        nums[i] = nums[left];
                        nums[i + 1] = nums[left];
                        i += 2;
                    }
                };
                left = right;
            }
        }

        i as i32
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $a:expr) => {
            let mut v = vec!$v;
            assert_eq!(
                Solution::remove_duplicates(&mut v),
                $a
            );
        }
    };
    test!([1, 1, 1, 2, 2, 3], 5);
    test!([0, 0, 1, 1, 1, 1, 2, 3, 3], 7);
    test!([1, 1, 1, 1, 1, 1], 2);
    test!([], 0);
    test!([1], 1);
}
