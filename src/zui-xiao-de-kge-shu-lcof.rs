struct Solution;

impl Solution {
    fn get_least_numbers_sorting(mut arr: Vec<i32>, k: usize) -> Vec<i32> {
        arr.sort_unstable();
        arr.truncate(k);
        arr
    }
    fn get_least_numbers_heap(arr: Vec<i32>, k: usize) -> Vec<i32> {
        use std::collections::BinaryHeap;
        // 最大堆
        let mut heap = BinaryHeap::with_capacity(k);
        for num in arr {
            if heap.len() < k {
                heap.push(num);
                continue;
            }
            if let Some(&top) = heap.peek() {
                if top > num {
                    heap.pop();
                    heap.push(num);
                }
            }
        }

        heap.into_vec()
    }
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if k >= arr.len() / 8 {
            Self::get_least_numbers_sorting(arr, k)
        } else {
            Self::get_least_numbers_heap(arr, k)
        }
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $k:expr, $ans:tt) => {
            let mut ans = Solution::get_least_numbers_heap(vec!$n, $k);
            ans.sort_unstable();
            assert_eq!(
                ans,
                vec!$ans
            );
            assert_eq!(
                Solution::get_least_numbers_sorting(vec!$n, $k),
                vec!$ans
            );
        }
    };
    test!([3, 2, 1], 2, [1, 2]);
    test!([0, 1, 2, 1], 1, [0]);
}
