struct Solution;

impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        // find flag
        let xor = nums.iter().fold(0, |a, b| a ^ b);
        let mut flag = 0;
        for i in 0..32 {
            if (xor >> i) & 1 == 1 {
                flag = 1 << i;
                break;
            }
        }
        //
        let (mut xor1, mut xor2) = (0, 0);
        for n in nums {
            if n & flag != 0 {
                xor1 ^= n;
            } else {
                xor2 ^= n;
            }
        }
        vec![xor1, xor2]
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $a:expr) => {
            assert_eq!(
                Solution::single_numbers(vec!$n),
                $a
            );
        }
    };
    test!([1, 2], [1, 2]);
    test!([4, 1, 4, 6], [1, 6]);
}
