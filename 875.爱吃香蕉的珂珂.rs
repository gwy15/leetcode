/*
 * @lc app=leetcode.cn id=875 lang=rust
 *
 * [875] 爱吃香蕉的珂珂
 */
struct Solution;
// @lc code=start
impl Solution {
    fn can_eat_all(piles: &Vec<i32>, max_time: i32, speed: i32) -> bool {
        let mut t = 0;
        for &pile in piles.iter() {
            t += pile / speed;
            if pile % speed != 0 {
                t += 1;
            }
            if t > max_time {
                return false;
            }
        }
        true
    }
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let total: i32 = i32::max_value();
        let (mut left, mut right) = (0, total);
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if Solution::can_eat_all(&piles, h, mid) {
                right = mid;
            } else {
                left = mid;
            }
        }
        left + 1
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($p:tt, $h:tt, $a:expr) => {
            assert_eq!(
                Solution::min_eating_speed(vec!$p, $h),
                $a
            );
        }
    };
    test!([3, 6, 7, 11], 8, 4);
    test!([30, 11, 23, 4, 20], 5, 30);
    test!([30, 11, 23, 4, 20], 6, 23);
    test!([1], 1, 1);
    test!([i32::max_value(), i32::max_value()], 2, i32::max_value());
    test!(
        [
            332484035, 524908576, 855865114, 632922376, 222257295, 690155293, 112677673, 679580077,
            337406589, 290818316, 877337160, 901728858, 679284947, 688210097, 692137887, 718203285,
            629455728, 941802184
        ],
        823855818,
        14
    );
}
