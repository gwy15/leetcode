/*
 * @lc app=leetcode.cn id=1438 lang=rust
 *
 * [1438] 绝对差不超过限制的最长连续子数组
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;

#[derive(Debug)]
struct MaxQueue<T> {
    /// elements
    q: VecDeque<T>,
    /// max elements, a decreasing queue
    max: VecDeque<(T, u32)>,
}

#[allow(unused)]
impl<T> MaxQueue<T>
where
    T: std::cmp::Ord + std::clone::Clone,
{
    /// Create a new max queue
    pub fn new() -> Self {
        MaxQueue {
            q: VecDeque::new(),
            max: VecDeque::new(),
        }
    }
    /// push element to queue back
    pub fn push_back(&mut self, item: T) {
        // maintain max elements
        use std::cmp::Ordering::*;
        let mut count = 1;
        loop {
            if self.max.len() == 0 {
                self.max.push_back((item.clone(), count));
                break;
            }
            let (last_element, last_times) = self.max.back().unwrap();
            match last_element.cmp(&item) {
                Greater => {
                    self.max.push_back((item.clone(), count));
                    break;
                }
                Equal => {
                    // times += count
                    self.max.back_mut().unwrap().1 += count;
                    break;
                }
                Less => {
                    count += last_times;
                    self.max.pop_back().unwrap();
                    continue;
                }
            }
        }

        // push to elements
        self.q.push_back(item);
    }
    /// pop first (front) element from queue
    pub fn pop_front(&mut self) -> Option<T> {
        match self.q.pop_front() {
            None => None,
            Some(item) => {
                let max_element = self.max.front_mut().unwrap();
                if max_element.1 == 1 {
                    self.max.pop_front().unwrap();
                } else {
                    max_element.1 -= 1;
                }

                Some(item)
            }
        }
    }
    /// Length of the queue
    pub fn len(&self) -> usize {
        self.q.len()
    }
    /// Get max element from queue
    pub fn max(&self) -> Option<&T> {
        match self.max.front() {
            None => None,
            Some((max_element, _)) => Some(max_element),
        }
    }
}

#[allow(unused)]
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        use std::cmp::Reverse;
        let mut max = MaxQueue::<i32>::new();
        let mut min = MaxQueue::<Reverse<i32>>::new();
        let mut width = 0;
        let mut max_width = 0;
        // iterate thru nums
        for n in nums.iter() {
            // pop front until valid
            loop {
                // window valid only if .abs() < limit for both min and max
                let mut valid = (max.max().unwrap_or(n) - n).abs() <= limit;
                if valid {
                    if let Some(Reverse(min_element)) = min.max() {
                        valid = (min_element - n).abs() <= limit;
                    }
                }
                // pop front until valid
                if valid {
                    break;
                } else {
                    max.pop_front().unwrap();
                    min.pop_front().unwrap();
                    width -= 1;
                }
            }
            // add n to window
            width += 1;
            max_width = max_width.max(width);
            max.push_back(n.clone());
            min.push_back(Reverse(n.clone()));
        }
        max_width
    }
}
// @lc code=end
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_max_queue() {
        let mut q = MaxQueue::new();
        q.push_back(1);
        assert_eq!(*q.max().unwrap(), 1);
        q.push_back(2);
        assert_eq!(*q.max().unwrap(), 2);
        q.push_back(-1);
        assert_eq!(*q.max().unwrap(), 2);
        // pop 1, => 2, -1
        assert_eq!(q.pop_front().unwrap(), 1);
        assert_eq!(*q.max().unwrap(), 2);
        // pop 2, => -1
        assert_eq!(q.pop_front().unwrap(), 2);
        assert_eq!(*q.max().unwrap(), -1);
    }
    #[test]
    fn test_solution() {
        macro_rules! test {
            ($n:tt, $limit:expr, $ans:expr) => {
                assert_eq!(
                    Solution::longest_subarray(vec!$n, $limit),
                    $ans);
            }
        };
        test!([8, 2, 4, 7], 4, 2);
        test!([10, 1, 2, 4, 7, 2], 5, 4);
        test!([4, 2, 2, 2, 4, 4, 2, 2], 0, 3);
    }
}
