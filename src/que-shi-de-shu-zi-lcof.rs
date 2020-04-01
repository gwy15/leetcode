struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        if nums[0] != 0 {
            return 0;
        }
        let mut left = 0;
        let mut right = nums.len();
        // [left, right)
        while left + 1 < right {
            let i = (left + right) / 2;
            if nums[i] == i as i32 {
                left = i;
            } else {
                right = i;
            }
        }
        left as i32 + 1
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
    test!([0], 1);
    test!([1], 0);
    test!([0, 1, 3], 2);
    test!([0, 1, 2, 3, 4, 5, 6, 8, 9], 7);
}
