/*
 * @lc app=leetcode.cn id=295 lang=rust
 *
 * [295] 数据流的中位数
 */

// @lc code=start
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    bigger: BinaryHeap<Reverse<i32>>, // 最小堆
    smaller: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder {
            bigger: BinaryHeap::new(),
            smaller: BinaryHeap::new(),
        }
    }

    fn upper_median(&self) -> i32 {
        let Reverse(n) = self.bigger.peek().unwrap();
        *n
    }
    fn lower_median(&self) -> i32 {
        *self.smaller.peek().unwrap()
    }

    fn add_num(&mut self, num: i32) {
        if self.bigger.len() <= self.smaller.len() {
            self.bigger.push(Reverse(num));
        } else {
            self.smaller.push(num);
        }
        // adjust
        if self.bigger.len() >= 1
            && self.smaller.len() >= 1
            && self.upper_median() < self.lower_median()
        {
            let Reverse(a) = self.bigger.pop().unwrap();
            let b = self.smaller.pop().unwrap();
            self.bigger.push(Reverse(b));
            self.smaller.push(a);
        }
    }
    fn find_median(&self) -> f64 {
        match self.bigger.len().cmp(&self.smaller.len()) {
            Ordering::Equal => {
                let bigger = self.upper_median();
                let smaller = self.lower_median();
                (bigger as f64 + smaller as f64) / 2.
            }
            Ordering::Greater => self.upper_median() as f64,
            Ordering::Less => self.lower_median() as f64,
        }
    }
}

// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test_directive {
        ($finder:expr, push, $i:expr) => {
            $finder.add_num($i);
        };
        ($finder:expr, find, $i:expr) => {
            assert_eq!($finder.find_median(), $i);
        };
    };
    macro_rules! test {
        [$(($directive:tt, $value:expr)),*] => {
            let mut finder = MedianFinder::new();
            $(
                println!("testing directive: {} {}", stringify!($directive), $value);
                test_directive!(finder, $directive, $value);
            )*
        }
    };
    test![(push, 1), (push, 2), (find, 1.5), (push, 3), (find, 2.0)];
}
