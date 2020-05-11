struct Solution;

impl Solution {
    pub fn exchange_bits(mut num: i32) -> i32 {
        // for i in 0..16 {
        //     let part = [0, 2, 1, 3][(num >> (2 * i)) as usize & 0b11];
        //     // clear
        //     num &= !(0b11 << (2 * i));
        //     // set
        //     num |= part << (2 * i);
        // }
        // num
        let num = num as u32;
        (((num & 0x55555555) << 1) | ((num & 0xaaaaaaaa) >> 1)) as i32
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $a:expr) => {
            assert_eq!(Solution::exchange_bits($n), $a);
        };
    };
    test!(2, 1);
    test!(3, 3);
    test!(0b01010110, 0b10101001);
}
