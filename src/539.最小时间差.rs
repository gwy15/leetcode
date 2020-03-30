/*
 * @lc app=leetcode.cn id=539 lang=rust
 *
 * [539] 最小时间差
 */

struct Solution;

// @lc code=start
impl Solution {
    fn time_string_to_timestamp(time_string: String) -> usize {
        const OFFSET: usize = '0' as usize;
        let bytes: Vec<usize> = time_string
            .into_bytes()
            .into_iter()
            .map(|x| x as usize)
            .collect();
        let result = 60 * (10 * (bytes[0] - OFFSET) + bytes[1] - OFFSET)
            + (10 * (bytes[3] - OFFSET) + bytes[4] - OFFSET);
        result
    }

    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        const MINUTES: usize = 24 * 60;
        let mut seen = vec![false; MINUTES];
        for t_string in time_points.into_iter() {
            let t = Solution::time_string_to_timestamp(t_string);
            if seen[t] {
                return 0;
            } else {
                seen[t] = true;
            }
        }

        // find first seen and last seen
        let first_time = (0..MINUTES).find(|&t| seen[t]).unwrap();
        let last_time = (0..MINUTES).rev().find(|&t| seen[t]).unwrap();

        let mut min_gap = first_time + MINUTES - last_time;

        let mut last = first_time;
        for t in first_time + 1..last_time + 1 {
            if seen[t] {
                min_gap = min_gap.min(t - last);
                last = t;
            }
        }
        min_gap as i32
    }
}
// @lc code=end

#[test]
fn test_convert() {
    macro_rules! test {
        ($s:expr, $ans:expr) => {
            assert_eq!(Solution::time_string_to_timestamp($s.to_owned()), $ans);
        };
    }
    test!("00:00", 0);
    test!("01:00", 60);
    test!("12:23", 12 * 60 + 23);
}

#[test]
fn test_solution() {
    macro_rules! test {
        (($($t:expr),*), $ans:expr) => {
            assert_eq!(
                Solution::find_min_difference(vec![
                    $($t.to_owned()),*
                ]),
                $ans
            );
        };
    }
    test!(("23:59", "00:00"), 1);
    test!(("12:00", "13:01"), 61);
    test!(("12:00", "12:00"), 0);
}
