#![allow(unused)]
struct Solution;

impl Solution {
    fn sum_digits(mut i: usize) -> usize {
        let mut sum = 0;
        while i != 0 {
            sum += i % 10;
            i /= 10;
        }
        sum
    }
    fn can_move_to(i: usize, j: usize, k: usize) -> bool {
        Solution::sum_digits(i) + Solution::sum_digits(j) <= k
    }
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        if k == 0 {
            return 1;
        }
        let m = m as usize;
        let n = n as usize;
        let mut visited = vec![vec![false; n]; m];
        let mut sum = 0;
        // BFS
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0));
        while queue.len() != 0 {
            let (i, j) = queue.pop_front().unwrap();
            match visited[i][j] {
                true => continue,
                false => {
                    visited[i][j] = true;
                    match Solution::can_move_to(i, j, k as usize) {
                        true => sum += 1,
                        false => continue,
                    }
                    // next level
                    if 0 < i {
                        queue.push_back((i - 1, j));
                    }
                    if i < m - 1 {
                        queue.push_back((i + 1, j));
                    }
                    if 0 < j {
                        queue.push_back((i, j - 1));
                    }
                    if j < n - 1 {
                        queue.push_back((i, j + 1));
                    }
                }
            }
        }
        sum
    }
}

#[test]
fn test_sum_digits() {
    assert_eq!(Solution::sum_digits(1), 1);
    assert_eq!(Solution::sum_digits(10), 1);
    assert_eq!(Solution::sum_digits(19), 10);
    assert_eq!(Solution::sum_digits(2028), 12);
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($m:expr, $n:expr, $k:expr, $ans:expr) => {
            assert_eq!(Solution::moving_count($m, $n, $k), $ans);
        };
    };
    // test!(2, 3, 1, 3);
    // test!(3, 1, 0, 1);
    test!(16, 8, 4, 15);
}
