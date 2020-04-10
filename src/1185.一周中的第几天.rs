/*
 * @lc app=leetcode.cn id=1185 lang=rust
 *
 * [1185] 一周中的第几天
 */
struct Solution;

// @lc code=start
const days: [&'static str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

impl Solution {
    fn is_leap_year(year: i32) -> bool {
        match year {
            2000 => true,
            2100 => false,
            n => n % 4 == 0,
        }
    }
    fn julianday(day: i32, month: i32, year: i32) -> usize {
        let mut n = 0;
        for y in 1971..year {
            n += 365 + Solution::is_leap_year(y) as usize;
        }
        for m in 1..month {
            n += match m {
                2 => 28 + Solution::is_leap_year(year) as usize,
                1 | 3 | 5 | 7 | 8 | 10 => 31,
                4 | 6 | 9 | 11 => 30,
                _ => unreachable!(),
            }
        }
        n += day as usize - 1;
        n
    }
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let offset = 5; // friday
        let index = offset + Solution::julianday(day, month, year);
        days[index % 7].to_string()
    }
}
// @lc code=end
#[test]
fn test_solution() {
    assert_eq!(Solution::is_leap_year(2008), true);
    assert_eq!(Solution::julianday(1, 1, 1971), 0);
    assert_eq!(Solution::day_of_the_week(10, 4, 2020), "Friday");
}
