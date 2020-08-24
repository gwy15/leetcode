/*
 * @lc app=leetcode.cn id=752 lang=rust
 *
 * [752] 打开转盘锁
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    /// "1234" -> 1234
    fn string_to_number(s: String) -> usize {
        let mut n = 0;
        for ch in s.chars() {
            n = 10 * n + (ch as u8 - '0' as u8) as usize;
        }
        n
    }

    #[inline(always)]
    fn neighbors(x: usize) -> [usize; 8] {
        let (a, b, c, d) = (x / 1000, (x / 100) % 10, (x / 10) % 10, x % 10);
        #[inline(always)]
        fn array_to_number(a: usize, b: usize, c: usize, d: usize) -> usize {
            1000 * a + 100 * b + 10 * c + d
        }
        [
            array_to_number((a + 1) % 10, b, c, d),
            array_to_number(a, (b + 1) % 10, c, d),
            array_to_number(a, b, (c + 1) % 10, d),
            array_to_number(a, b, c, (d + 1) % 10),
            //
            array_to_number((a + 9) % 10, b, c, d),
            array_to_number(a, (b + 9) % 10, c, d),
            array_to_number(a, b, (c + 9) % 10, d),
            array_to_number(a, b, c, (d + 9) % 10),
        ]
    }

    #[allow(unused)]
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let target = Self::string_to_number(target);
        // init map
        let mut visitable = vec![true; 1_0000];
        for deadend in deadends {
            visitable[Self::string_to_number(deadend)] = false;
        }
        if visitable[0] == false {
            return -1;
        }
        //
        if target == 0 {
            return 0;
        }
        let mut cnt = 1;
        // begin BFS
        let mut q = VecDeque::new();
        visitable[0] = false;
        q.push_back(0);

        while q.len() > 0 {
            for _ in 0..q.len() {
                let x = q.pop_front().unwrap();
                // visit neighbors
                for &neighbor in &Self::neighbors(x) {
                    if neighbor == target {
                        return cnt;
                    }
                    if visitable[neighbor] {
                        visitable[neighbor] = false;
                        q.push_back(neighbor);
                    }
                }
            }
            cnt += 1;
        }

        -1
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($strings:tt, $target:expr, $ans:expr) => {
            assert_eq!(
                Solution::open_lock(vec_string!$strings, $target.into()),
                $ans
            )
        };
    }
    test!(["0201", "0101", "0102", "1212", "2002"], "0202", 6);
    test!(["0201", "0101", "0102", "1212", "2002"], "0000", 0);
    test!(["0201", "0101", "0102", "1212", "2002"], "0090", 1);
    test!(["0000"], "8888", -1);
}
