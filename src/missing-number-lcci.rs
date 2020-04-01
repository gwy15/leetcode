struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let expected_sum = n * (n + 1) / 2;
        expected_sum - nums.into_iter().fold(0, |s, x| s + x)
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ([$($i:expr),*], $ans:expr) => {
            assert_eq!(
                Solution::missing_number(vec![$($i),*]),
                $ans
            );
        }
    };
    test!([3, 0, 1], 2);
}
