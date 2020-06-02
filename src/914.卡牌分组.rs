/*
 * @lc app=leetcode.cn id=914 lang=rust
 *
 * [914] 卡牌分组
 */
struct Solution;
// @lc code=start
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while a != 0 {
            // (a, b) => (b % a, a)
            let tmp = b % a;
            b = a;
            a = tmp;
        }

        b
    }

    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        const MAX_NUMBER: usize = 10_000;
        let mut counter = vec![0; MAX_NUMBER];
        deck.iter().for_each(|&i| {
            counter[i as usize] += 1;
        });
        // find gcd
        let mut gcd = 0;
        for &count in counter.iter().filter(|&&n| n != 0) {
            if gcd == 0 {
                gcd = count;
            } else {
                gcd = Self::gcd(gcd, count);
            }
            if gcd == 1 {
                return false;
            }
        }
        true
    }
}
// @lc code=end
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_gcd() {
        macro_rules! test {
            ($a:expr, $b:expr, $ans:expr) => {
                assert_eq!(Solution::gcd($a, $b), $ans);
            };
        };
        test!(1, 1, 1);
        test!(6, 12, 6);
        test!(4, 81, 1);
        test!(15, 16, 1);
        test!(27 * 2, 27 * 3, 27);
        test!(32 * 3, 32 * 2, 32);
    }

    #[test]
    fn test_solution() {
        macro_rules! test {
            ($d:tt, $ans:expr) => {
                assert_eq!(
                    Solution::has_groups_size_x(vec!$d),
                    $ans
                );
            }
        };
        test!([1, 2, 3, 4, 4, 3, 2, 1], true);
        test!([1, 1, 1, 2, 2, 2, 3, 3], false);
        test!([1], false);
        test!([1, 1], true);
        test!([1, 1, 2, 2, 2, 2], true);
    }
}
