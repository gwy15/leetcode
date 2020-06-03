struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn merge(a: &mut Vec<i32>, m: i32, b: &mut Vec<i32>, n: i32) {
        use std::cmp::Ordering::*;

        let (mut m, mut n) = (m as usize, n as usize);
        let mut pos = m + n;
        while m > 0 && n > 0 {
            // compare and move
            pos -= 1;
            a[pos] = match a[m - 1].cmp(&b[n - 1]) {
                Greater | Equal => {
                    // move m
                    m -= 1;
                    a[m]
                }
                Less => {
                    // move n
                    n -= 1;
                    b[n]
                }
            };
        }
        // move rest of b array
        for i in 0..n {
            a[i] = b[i];
        }
    }
}

// test
#[test]
fn test_solution() {
    macro_rules! test {
        ($a:tt, $m:expr, $b:tt, $n:expr, $ans:tt) => {
            let mut a = vec!$a;
            let mut b = vec!$b;
            Solution::merge(&mut a, $m, &mut b, $n);
            assert_eq!(a, vec!$ans);
        }
    };
    test!([1, 2, 3, 0, 0, 0], 3, [2, 5, 6], 3, [1, 2, 2, 3, 5, 6]);
}
