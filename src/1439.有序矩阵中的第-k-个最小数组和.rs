/*
 * @lc app=leetcode.cn id=1439 lang=rust
 *
 * [1439] 有序矩阵中的第 k 个最小数组和
 */
struct Solution;
// @lc code=start
use std::iter::FromIterator;
impl Solution {
    fn merge(previous_sum: Vec<i32>, row: &[i32], k: usize) -> Vec<i32> {
        // merge sum
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut h = vec![];
        for (i, previous) in previous_sum.iter().enumerate() {
            let sum = previous + row[0];
            h.push((Reverse(sum), i, 0));
        }
        let mut heap = BinaryHeap::from_iter(h);
        //
        let mut ans = vec![];
        for i in 0..k {
            match heap.pop() {
                None => return ans,
                Some((Reverse(sum), i, mut j)) => {
                    ans.push(sum);
                    j += 1;
                    if j < row.len() {
                        let next_sum = previous_sum[i] + row[j];
                        heap.push((Reverse(next_sum), i, j));
                    }
                }
            }
        }
        ans
    }
    /// 对每一行，都找到其以上组成中，最小的 k 个和
    /// 复杂度：每一行都是 k log k 复杂度进行合并
    /// 一共 m 行，所以复杂度为 m k log k
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let (m, n) = (mat.len(), mat[0].len());
        let mut previous_sum: Vec<i32> = mat[0][..k.min(n)].iter().map(|&n| n).collect();
        for i in 1..m {
            previous_sum = Self::merge(previous_sum, &mat[i], k);
        }

        previous_sum[k - 1]
    }

    /// 二分查找 sum
    /// 复杂度：
    ///   二分是 log(max_sum) = log(5000 m)
    ///   每次二分查找和 <= sum 的数组数量，注意到数组数量只要 >k，就相当于 k+1
    ///   所以每次二分查找的复杂度最大都是 O(min(k, ))
    ///
    pub fn kth_smallest2() {}
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, k=$k:expr, $ans:expr) => {
            assert_eq!(
                Solution::kth_smallest(vec2d!$args, $k),
                $ans
            )
        };
    }
    test!([[1, 3, 11], [2, 4, 6]], k = 5, 7);
    test!([[1, 3, 11], [2, 4, 6]], k = 9, 17);
    test!([[1, 10, 10], [1, 4, 5], [2, 3, 6]], k = 7, 9);
    test!([[1, 1, 10], [2, 2, 9]], k = 7, 12);
}
