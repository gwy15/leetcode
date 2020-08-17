/*
 * @lc app=leetcode.cn id=31 lang=rust
 *
 * [31] 下一个排列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // 1, 2, 3, 4 => 4, 3, 2, 1
        // ... 3 [7, _4_, 2] => ... 4 [7 3 2] => ... 4 [2 3 7]
        // 1. find first number that is not ascending backwards
        // 2. find the first number that is greater than that.
        // 3. swap
        // 4. reverse rest numbers to make it descending backwards.
        let n = nums.len();
        // i < i+1 > > >
        let mut i = n;
        for j in (0..n - 1).rev() {
            if nums[j] < nums[j + 1] {
                i = j;
                break;
            }
        }
        // not found, the max permutation.
        if i == n {
            nums.reverse();
            return;
        }
        // find the first number that is > nums[i]
        for j in (i..n).rev() {
            if nums[j] > nums[i] {
                // swap i, j
                nums.swap(i, j);
                // swap rest ([i+1..])
                nums[i + 1..].reverse();
                break;
            }
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:tt) => {
            let mut n = vec!$args;
            Solution::next_permutation(&mut n);
            assert_eq!(
                n, vec!$ans
            );
        };
    }
    test!([1, 2, 3], [1, 3, 2]);
    test!([3, 2, 1], [1, 2, 3]);
    test!([1, 1, 5], [1, 5, 1]);
    test!([1], [1]);
    test!([1, 2], [2, 1]);
}
