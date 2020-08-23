struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 {
            return 0;
        }
        let (mut i, mut j) = (0, n - 1);
        let (mut left, mut right) = (height[0], height[j]);
        let mut sum = 0;
        while i < j {
            // move left
            if left < right {
                sum += left - height[i];
                i += 1;
                left = left.max(height[i]);
            } else {
                sum += right - height[j];
                j -= 1;
                right = right.max(height[j]);
            }
        }
        sum
    }
}
#[test]
fn test_solution() {
    macro_rules! test {
        ($h:tt, $ans:expr) => {
            assert_eq!(
                Solution::trap(vec!$h),
                $ans
            )
        };
    }
    test!([1, 2, 3, 4, 5, 4, 3, 2, 1], 0);
    test!([1, 2, 3, 0, 3, 2, 1], 3);
    test!([], 0);
    test!([1, 0, 1], 1);
    test!([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6);
    test!([4, 2, 3], 1);
}
