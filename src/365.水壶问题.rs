/*
 * @lc app=leetcode.cn id=365 lang=rust
 *
 * [365] 水壶问题
 */
struct Solution;
// @lc code=start
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b > 0 {
            // (a, b) = (b, a % b);
            let tmp = a % b;
            a = b;
            b = tmp;
        }
        a
    }

    pub fn can_measure_water(max_X: i32, max_Y: i32, z: i32) -> bool {
        if max_Y > max_X {
            return Solution::can_measure_water(max_Y, max_X, z);
        }
        if z == 0 || z == max_X + max_Y {
            return true;
        }
        if z > max_X + max_Y {
            return false;
        }
        // a max_X + b max_Y = z
        // Bézout's identity
        z % Solution::gcd(max_X, max_Y) == 0
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($x:expr, $y:expr, $z:expr, $ans:expr) => {
            assert_eq!(Solution::can_measure_water($x, $y, $z), $ans);
        };
    };
    test!(3, 5, 4, true);
    test!(2, 6, 5, false);
}
