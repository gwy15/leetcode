struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = card_points.len();
        let mut sum: i32 = card_points[..k].iter().sum();
        let mut max_sum = sum;
        for i in 0..k {
            sum = sum - card_points[k - 1 - i] + card_points[n - 1 - i];
            max_sum = max_sum.max(sum);
        }
        max_sum
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $k:expr, $a:expr) => {
            assert_eq!(
                Solution::max_score(vec!$v, $k),
                $a
            );
        }
    };
    test!([1, 2, 3, 4, 5, 6, 1], 3, 12);
    test!([2, 2, 2], 2, 4);
    test!([9, 7, 7, 9, 7, 7, 9], 7, 55);
    test!([1, 1000, 1], 1, 1);
    test!([1, 79, 80, 1, 1, 1, 200, 1], 3, 202);
}
