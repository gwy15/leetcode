/*
 * @lc app=leetcode.cn id=547 lang=rust
 *
 * [547] 朋友圈
 */
struct Solution;
// @lc code=start
#[allow(unused)]
impl Solution {
    #[inline]
    fn find(x: usize, f: &mut [usize]) -> usize {
        if f[x] != x {
            f[x] = Self::find(f[x], f);
        }
        f[x]
    }

    #[inline]
    fn merge(i: usize, j: usize, size: &mut [usize], f: &mut [usize]) {
        let i = Self::find(i, f);
        let j = Self::find(j, f);
        if i != j {
            // join
            if size[i] > size[j] {
                // j -> i
                size[i] += size[j];
                f[j] = i;
            } else {
                // j -> i
                size[j] += size[i];
                f[i] = j;
            }
        }
    }

    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let n = m.len();

        let mut friend: Vec<usize> = (0..n).collect();
        let mut size: Vec<usize> = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                if m[i][j] == 1 {
                    Self::merge(i, j, &mut size, &mut friend);
                }
            }
        }

        friend
            .into_iter()
            .enumerate()
            .filter(|&(i, f)| i == f)
            .count() as i32
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($f:tt, $ans:expr) => {
            assert_eq!(
                Solution::find_circle_num(vec2d!$f),
                $ans
            );
        }
    };
    test!([[1, 1, 0], [1, 1, 0], [0, 0, 1]], 2);
    test!([[1, 1, 0], [1, 1, 1], [0, 1, 1]], 1);
    test!(
        [
            // 01  2  3  4  5  6  7  8  9  10 11 12 13 14
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        ],
        8
    );
    test!([[1, 0, 0, 1], [0, 1, 1, 0], [0, 1, 1, 1], [1, 0, 1, 1]], 1);
}
