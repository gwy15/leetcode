/*
 * @lc app=leetcode.cn id=33 lang=rust
 *
 * [33] 搜索旋转排序数组
 */
struct Solution;
// @lc code=start
impl Solution {
    fn complex_search(nums: &[i32], target: i32, base: i32) -> i32 {
        // println!("searching nums={:?}, target={}, base={}", nums, target, base);
        let n = nums.len();
        match n {
            0 => return -1,
            1 => return if nums[0] == target { base } else { -1 },
            _ => {}
        }
        if nums[0] == target {
            return base;
        }

        let mid = n / 2;
        if nums[mid] == target {
            return base + mid as i32;
        }

        if target > nums[0] {
            if target < nums[mid] {
                Solution::simple_search(&nums[1..mid], target, base + 1)
            } else {
                if nums[mid] > nums[0] {
                    Solution::complex_search(&nums[mid + 1..], target, base + mid as i32 + 1)
                } else {
                    Solution::complex_search(&nums[1..mid], target, base + 1)
                }
            }
        } else {
            if target > nums[mid] {
                Solution::simple_search(&nums[mid + 1..], target, base + mid as i32 + 1)
            } else {
                if nums[mid] > nums[0] {
                    Solution::complex_search(&nums[mid + 1..], target, base + mid as i32 + 1)
                } else {
                    Solution::complex_search(&nums[1..mid], target, base + 1)
                }
            }
        }
    }

    fn simple_search(nums: &[i32], target: i32, base: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(i) => base + i as i32,
            Err(_) => -1,
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        match n {
            0 => return -1,
            1 => return if nums[0] == target { 0 } else { -1 },
            _ => {}
        }
        if nums[0] < nums[n - 1] {
            Solution::simple_search(&nums[..], target, 0)
        } else {
            Solution::complex_search(&nums[..], target, 0)
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $t:expr, $a:expr) => {
            assert_eq!(
                Solution::search(vec!$v, $t),
                $a
            );
        }
    };
    test!([4, 5, 6, 7, 0, 1, 2], 0, 4);
    test!([4, 5, 6, 7, 0, 1, 2], 3, -1);
    test!([1, 2, 3], 2, 1);
    test!([1], 1, 0);
    test!([5, 1, 2, 3, 4], 3, 3);
    test!([], 1, -1);
    test!([3, 1], 3, 0);
    test!([3, 5, 1], 1, 2);
}
