/*
 * @lc app=leetcode.cn id=475 lang=rust
 *
 * [475] 供暖器
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        houses.sort();
        heaters.sort();

        let n = heaters.len();
        // scan from left to right
        let mut heater_index = 0;

        let mut ans = i32::min_value();
        for house in houses {
            // move heater if necessary
            while heater_index < n - 1 && house >= heaters[heater_index + 1] {
                heater_index += 1;
            }
            let heater = heaters[heater_index];

            let dist = if house <= heater {
                heater - house
            } else {
                if heater_index == n - 1 {
                    house - heater
                } else {
                    i32::min(house - heater, heaters[heater_index + 1] - house)
                }
            };
            // println!("house = {}, heater = {}, dist={}", house, heater, dist);
            ans = ans.max(dist);
        }
        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($houses:expr, $heaters:expr, $ans:expr) => {
            assert_eq!(Solution::find_radius($houses, $heaters), $ans);
        };
    };
    test!(vec![1, 2, 3], vec![2], 1);
    test!(vec![1, 2, 3, 4], vec![1, 4], 1);
}
