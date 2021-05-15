#[allow(unused)]
struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn magic_tower(mut nums: Vec<i32>) -> i32 {
        if nums.iter().map(|i| *i as i64).sum::<i64>() < 0 {
            return -1;
        }

        let mut cur_hp = 1i64;
        let mut visited_monsters = BinaryHeap::new();
        let mut adjust_times = 0;

        let mut i = 0;
        while i < nums.len() {
            let next_block = nums[i] as i64;
            i += 1;

            cur_hp += next_block;
            if next_block < 0 {
                visited_monsters.push(-next_block);
            }
            while cur_hp <= 0 {
                if let Some(badass) = visited_monsters.pop() {
                    cur_hp += badass;
                    adjust_times += 1;
                } else {
                    return -1;
                }
            }
        }
        adjust_times
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(
                Solution::magic_tower(vec!$args),
                $ans
            )
        };
    }
    test!([100, 100, 100, -250, -60, -140, -50, -50, 100, 150], 1);
    test!([-200, -300, 400, 0], -1);
    test!([100, 100, 100, -100, -100, -100], 0);
}
