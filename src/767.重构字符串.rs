/*
 * @lc app=leetcode.cn id=767 lang=rust
 *
 * [767] 重构字符串
 */

struct Solution;

// @lc code=start
use std::collections::BinaryHeap;

struct CharCount {
    ch: u8,
    count: usize,
}

impl std::cmp::PartialEq for CharCount {
    fn eq(&self, rhs: &Self) -> bool {
        self.count == rhs.count
    }
}

impl std::cmp::Eq for CharCount {}

impl std::cmp::PartialOrd for CharCount {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.count.partial_cmp(&rhs.count)
    }
}

impl std::cmp::Ord for CharCount {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.count.cmp(&rhs.count)
    }
}

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let n = s.len();
        let threshold = (n + 1) / 2;
        let mut letter_counter = vec![0; 255];

        for ch in s.chars() {
            letter_counter[ch as usize] += 1;
        }

        // early deny
        if letter_counter.iter().any(|&c| c > threshold) {
            return "".to_owned();
        }

        // generate max heap
        let mut heap = BinaryHeap::new();
        for (i, &count) in letter_counter.iter().enumerate() {
            if count == 0 {
                continue;
            }
            println!("counter: char {} {} times", i as u8 as char, count);
            heap.push(CharCount { ch: i as u8, count });
        }
        // generate answer
        let mut result = String::with_capacity(n);
        while heap.len() >= 2 {
            let ch1 = heap.pop().unwrap();
            let ch2 = heap.pop().unwrap();
            result.push(ch1.ch as char);
            result.push(ch2.ch as char);
            if ch1.count > 1 {
                heap.push(CharCount {
                    ch: ch1.ch,
                    count: ch1.count - 1,
                });
            }
            if ch2.count > 1 {
                heap.push(CharCount {
                    ch: ch2.ch,
                    count: ch2.count - 1,
                });
            }
        }
        // final char
        if heap.len() != 0 {
            result.push(heap.pop().unwrap().ch as char);
        }
        result
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $ans:expr) => {
            assert_eq!(Solution::reorganize_string($s.to_owned()), $ans.to_owned());
        };
    };
    test!("aab", "aba");
    test!("aaab", "");
}
