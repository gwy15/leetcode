/*
 * @lc app=leetcode.cn id=841 lang=rust
 *
 * [841] 钥匙和房间
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut visited = vec![false; n];
        let mut q = VecDeque::new();
        visited[0] = true;
        q.push_back(0);
        while q.len() > 0 {
            let i = q.pop_front().unwrap();
            for &next in rooms[i].iter() {
                let next = next as usize;
                if !visited[next] {
                    visited[next] = true;
                    q.push_back(next);
                }
            }
        }

        visited.iter().all(|&v| v)
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(
                Solution::can_visit_all_rooms(vec2d!$args),
                $ans
            )
        };
    }
    test!([[1], [2], [3], []], true);
    test!([[1, 3], [3, 0, 1], [2], [0]], false);
}
