/*
 * @lc app=leetcode.cn id=1386 lang=rust
 *
 * [1386] 安排电影院座位
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    fn max_number_of_families_sparse(n: usize, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        reserved_seats.iter().for_each(|pair| {
            let (x, y) = (pair[0], pair[1]);
            if 2 <= y && y <= 9 {
                *map.entry(x - 1).or_insert(0) |= 1 << (y - 2);
            }
        });
        let occupied_seats = map.iter().fold(0, |ans, (_, row)| {
            if row & 0x0F == 0 || row & 0xF0 == 0 || row & 0b111100 == 0 {
                ans + 1
            } else {
                ans + 2
            }
        });
        2 * n as i32 - occupied_seats
    }
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        if n > 10 * reserved_seats.len() {
            Self::max_number_of_families_sparse(n, reserved_seats)
        } else {
            let mut occupied = vec![0; n.min(n)];
            for pair in reserved_seats {
                let (x, y) = (pair[0], pair[1]);
                if 2 <= y && y <= 9 {
                    occupied[(x - 1) as usize] |= 1 << (y - 2);
                }
            }
            occupied.into_iter().fold(0, |ans, row| {
                if row == 0 {
                    ans + 2
                } else if (row & 0x0F) == 0 || (row & 0xF0) == 0 || (row & 0b111100) == 0 {
                    ans + 1
                } else {
                    ans
                }
            })
        }
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $s:tt, $ans:expr) => {
            assert_eq!(
                Solution::max_number_of_families($n, vec2d!$s),
                $ans
            );
        }
    };
    test!(3, [[1, 2], [1, 3], [1, 8], [2, 6], [3, 1], [3, 10]], 4);
    test!(2, [[2, 1], [1, 8], [2, 6]], 2);
    test!(4, [[4, 3], [1, 4], [4, 6], [1, 7]], 4);
    test!(
        1000000000,
        [
            [559099, 6],
            [693306, 1],
            [782664, 9],
            [607404, 5],
            [135009, 10],
            [469442, 5],
            [485990, 7],
            [932212, 3],
            [505531, 7]
        ],
        1999999993
    );
}
