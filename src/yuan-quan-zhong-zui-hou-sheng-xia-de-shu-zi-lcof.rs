struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        let mut f = 0;
        for nn in 2..=n {
            f = (f + m) % nn;
        }
        f
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $m:expr, $ans:expr) => {
            assert_eq!(Solution::last_remaining($n, $m), $ans);
        };
    };
    test!(5, 3, 3);
    test!(10, 17, 2);
}
