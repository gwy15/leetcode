/*
 * @lc app=leetcode.cn id=1011 lang=rust
 *
 * [1011] 在 D 天内送达包裹的能力
 */
struct Solution;
// @lc code=start
impl Solution {
    fn can_carry_with_capacity(weights: &Vec<i32>, max_days: i32, capacity: i32) -> bool {
        let (mut day, mut today_weight) = (1, 0);
        for &item in weights.iter() {
            if today_weight + item > capacity {
                today_weight = item;
                day += 1;
                if day > max_days {
                    // println!("cannot carry all");
                    return false;
                }
            } else {
                // println!("putting item {} to day {}", item, day);
                today_weight += item;
            }
        }
        today_weight <= capacity && day <= max_days
    }
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let total = weights.iter().sum();
        if d == 1 {
            return total;
        }
        let max_weight = weights.iter().fold(weights[0], |a, b| a.max(*b));
        let (mut left, mut right) = (max_weight - 1, total);
        // println!("searching left={}, right={}", left, right);
        while right - left > 1 {
            let capacity = left + (right - left + 1) / 2;
            let ok = Solution::can_carry_with_capacity(&weights, d, capacity);
            // println!("carry with capacity {}? {}", capacity, ok);
            match ok {
                true => right = capacity,
                false => left = capacity,
            }
        }
        right
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($w:tt, $d:tt, $ans:tt) => {
            assert_eq!(
                Solution::ship_within_days(vec!$w, $d),
                $ans
            );
        }
    };
    test!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5, 15);
    test!([3, 2, 2, 4, 1, 4], 3, 6);
    test!([1, 2, 3, 1, 1], 4, 3);
}
