/*
 * @lc app=leetcode.cn id=1388 lang=rust
 *
 * [1388] 3n 块披萨
 */
struct Solution;
// @lc code=start
use std::collections::BinaryHeap;
use std::iter::FromIterator;
impl Solution {
    #[allow(unused)]
    pub fn max_size_slices(mut slices: Vec<i32>) -> i32 {
        let n = slices.len();
        // make linked list
        let mut left = Vec::from_iter((0..n).map(|i| (n + i - 1) % n));
        let mut right = Vec::from_iter((0..n).map(|i| (i + 1) % n));
        //
        let mut taken = vec![false; n];
        let mut heap = BinaryHeap::from_iter(slices.iter().enumerate().map(|(i, &item)| (item, i)));
        let mut ans = 0;
        for _ in 0..(n / 3) {
            let pos = loop {
                // find max value
                let (_, i) = heap.pop().unwrap();
                if !taken[i] {
                    break i;
                }
            };
            ans += slices[pos];
            let left_pos = left[pos];
            let right_pos = right[pos];
            // strip from linked list
            taken[left_pos] = true;
            taken[right_pos] = true;
            left[pos] = left[left_pos];
            right[pos] = right[right_pos];
            left[right[pos]] = pos;
            right[left[pos]] = pos;
            // add back for regret
            slices[pos] = slices[left_pos] + slices[right_pos] - slices[pos];
            heap.push((slices[pos], pos));
        }

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $ans:expr) => {
            assert_eq!(
                Solution::max_size_slices(vec!$n),
                $ans
            );
        }
    };
    test!([1, 2, 3, 4, 5, 6], 10);
    test!([8, 9, 8, 6, 1, 1], 16);
    test!([4, 1, 2, 5, 8, 3, 1, 9, 7], 21);
    test!([3, 1, 2], 3);
    test!([], 0);
}
