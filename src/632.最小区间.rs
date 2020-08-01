/*
 * @lc app=leetcode.cn id=632 lang=rust
 *
 * [632] 最小区间
 */
struct Solution;
// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    #[allow(unused)]
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        // answer
        let mut min_length = i32::max_value();
        let mut min_range = (0, 0);
        // 从小到大遍历全部元素，利用最小堆 (size=k) 维护，复杂度 O(num log k)
        let mut heap = BinaryHeap::new();
        let mut heap_max = nums[0][0];
        for i in 0..n {
            let value = nums[i][0];
            let item = (value, i, 0_usize);
            heap.push(Reverse(item));
            heap_max = heap_max.max(value);
        }
        // 对每个元素，每次 pop 出来，然后添加对应列表的下一个元素，同时维护这 k 个元素的最大值
        loop {
            let Reverse((min, i, mut j)) = heap.pop().unwrap();
            // 区间长度为 max - min
            if heap_max - min < min_length {
                min_length = heap_max - min;
                min_range = (min, heap_max);
            }
            // pop and add next
            j += 1;
            if j == nums[i].len() {
                break;
            }
            let value = nums[i][j];
            heap.push(Reverse((value, i, j)));
            heap_max = heap_max.max(value);
        }

        vec![min_range.0, min_range.1]
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:tt) => {
            assert_eq!(
                Solution::smallest_range(vec2d!$args),
                vec!$ans
            )
        };
    }
    test!(
        [[4, 10, 15, 24, 26], [0, 9, 12, 20], [5, 18, 22, 30]],
        [20, 24]
    );
}
