struct Solution;

impl Solution {
    pub fn maximum(a: i32, b: i32) -> i32 {
        macro_rules! sign {
            ($n:expr) => {{
                (($n as u32) >> 31) as i32
            }};
        }
        macro_rules! bit_if {
            ($cond:expr, $t:expr, $f:expr) => {{
                $cond * $t + (1 - $cond) * $f
            }};
        }
        // overflow not considered
        let a_sub_b = a.overflowing_sub(b).0;
        let is_a_bigger_simple = (sign!(a_sub_b) == 0) as i32;
        // consider overflow now
        let is_sign_different = sign!(a) ^ sign!(b);
        let is_a_positive = (sign!(a) == 0) as i32;

        let is_a_bigger = bit_if!(is_sign_different, is_a_positive, is_a_bigger_simple);
        bit_if!(is_a_bigger, a, b)
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($a:expr, $b:expr) => {
            let ans = Solution::maximum($a, $b);
            assert_eq!(ans, $a.max($b));
        };
    };
    test!(1, 2);
    test!(1, 1);
    test!(1, -1);
    test!(-2, 15);
    test!(2147483647, -2147483648);
}
