/*
 * @lc app=leetcode.cn id=493 lang=rust
 *
 * [493] 翻转对
 */
struct Solution;
// @lc code=start
#[allow(unused)]
impl Solution {
    fn merge_sort(nums: &mut [i32]) -> usize {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }
        let mid = n / 2;

        // divide and conquer
        let left = Self::merge_sort(&mut nums[..mid]);
        let right = Self::merge_sort(&mut nums[mid..]);
        // stats reversed pairs
        let ans = left + right + {
            let mut ans = 0;
            let mut j = mid;
            for i in 0..mid {
                while j < n && nums[i] as i64 > 2 * nums[j] as i64 {
                    j += 1;
                }
                ans += j - mid;
            }
            ans
        };

        // merge sort
        let mut sorted = Vec::with_capacity(n);
        let (mut i, mut j) = (0, mid);
        while i < mid && j < n {
            if nums[i] > nums[j] {
                sorted.push(nums[j]);
                j += 1;
            } else {
                sorted.push(nums[i]);
                i += 1;
            }
        }
        // the rest
        for k in (i..mid).chain(j..n) {
            sorted.push(nums[k]);
        }
        // copy back
        for i in 0..n {
            nums[i] = sorted[i];
        }
        // return ans
        ans
    }
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        Self::merge_sort(&mut nums) as i32
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:expr) => {
            assert_eq!(
                Solution::reverse_pairs(vec!$n),
                $ans
            );
        }
    };
    test!([1, 3, 2, 3, 1], 2);
    test!([2, 4, 3, 5, 1], 3);
    test!([7, 3, 1], 2 + 1);
    test!([63, 31, 15, 7, 3, 1], 5 + 4 + 3 + 2 + 1);
    test!([], 0);
    test!([1], 0);
    test!([3, 1], 1);
    test!(
        [2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647],
        0
    );
}
