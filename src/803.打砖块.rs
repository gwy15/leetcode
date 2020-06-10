/*
 * @lc app=leetcode.cn id=803 lang=rust
 *
 * [803] 打砖块
 */
struct Solution;
// @lc code=start
struct DSU {
    m: usize,
    n: usize,
    f: Vec<usize>,
    size: Vec<i32>,
}
#[allow(unused)]
impl DSU {
    pub fn new(m: usize, n: usize) -> Self {
        let f = (0..(m * n + 1)).collect();
        let mut size = vec![1; m * n + 1];
        // virtual block has no size
        size[m * n] = 0;

        Self { m, n, f, size }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.f[x] != x {
            self.f[x] = self.find(self.f[x]);
        }
        self.f[x]
    }

    pub fn union(&mut self, mut x: usize, mut y: usize) {
        x = self.find(x);
        y = self.find(y);
        if x != y {
            if self.size[x] < self.size[y] {
                // x -> y
                self.f[x] = y;
                self.size[y] += self.size[x];
            } else {
                self.f[y] = x;
                self.size[x] += self.size[y];
            }
        }
    }

    pub fn len(&mut self) -> i32 {
        let root = self.find(self.m * self.n);
        self.size[root]
    }
}

impl Solution {
    pub fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        let ceil = m * n;
        let xy2i = |x: usize, y: usize| x * n + y;
        // mark hits
        for hit in hits.iter() {
            let (i, j) = (hit[0] as usize, hit[1] as usize);
            // mark only bricks. ignore empty hit.
            if grid[i][j] == 1 {
                grid[i][j] = -1;
            }
        }
        // init DSU
        let mut dsu = DSU::new(m, n);
        // link blocks
        for i in 0..m {
            for j in 0..n {
                macro_rules! bind {
                    ($cond:expr, $x:expr, $y:expr) => {
                        if $cond && grid[$x][$y] == 1 {
                            dsu.union(xy2i($x, $y), xy2i(i, j));
                        }
                    };
                }
                if grid[i][j] == 1 {
                    bind!(0 < i, i - 1, j);
                    bind!(0 < j, i, j - 1);
                    bind!(i < m - 1, i + 1, j);
                    bind!(j < n - 1, i, j + 1);
                }
            }
        }
        // link to ceil
        for j in 0..n {
            if grid[0][j] == 1 {
                dsu.union(ceil, xy2i(0, j));
            }
        }
        // reverse hit to union blocks
        let mut ans = vec![];
        let mut size = dsu.len();
        // iterate
        for hit in hits.iter().rev() {
            let (i, j) = (hit[0] as usize, hit[1] as usize);
            match grid[i][j] {
                -1 => {
                    grid[i][j] = 1;
                }
                0 => {
                    ans.push(0);
                    continue;
                }
                _ => {
                    unreachable!();
                }
            }
            // link (i,j) with blocks linked to it
            macro_rules! check {
                ($cond:expr, $x:expr, $y:expr) => {
                    if $cond && grid[$x][$y] == 1 {
                        dsu.union(xy2i(i, j), xy2i($x, $y));
                    }
                };
            }
            check!(0 < i, i - 1, j);
            check!(0 < j, i, j - 1);
            check!(i < m - 1, i + 1, j);
            check!(j < n - 1, i, j + 1);
            if i == 0 {
                dsu.union(xy2i(i, j), ceil);
            }

            // update size
            let new_size = dsu.len();
            if new_size == size {
                ans.push(0);
            } else {
                let delta = new_size - size - 1; // added a block, does not count
                ans.push(delta);
                size = new_size;
            }
        }

        ans.reverse();
        ans
    }
}
// @lc code=end
/* BFS. TLE
use std::collections::VecDeque;
impl Solution {
    fn count(grid: &mut Vec<Vec<i32>>, target: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut q = VecDeque::new();
        let mut cnt = 0;

        macro_rules! check {
            ($cond:expr, $i:expr, $j:expr) => {
                if $cond && grid[$i][$j] == target {
                    grid[$i][$j] = target + 1;
                    q.push_back(($i, $j));
                    cnt += 1;
                }
            };
        }

        for j in 0..n {
            check!(true, 0, j);
        }

        while q.len() > 0 {
            let (i, j) = q.pop_back().unwrap();
            check!(0 < i, i - 1, j);
            check!(0 < j, i, j - 1);
            check!(i < m - 1, i + 1, j);
            check!(j < n - 1, i, j + 1);
        }

        cnt
    }
    pub fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = Vec::new();
        for (times, hit) in hits.into_iter().enumerate() {
            let (i, j) = (hit[0] as usize, hit[1] as usize);
            grid[i][j] = 0;
            //
            let target = 1 + times as i32;
            Self::count(&mut grid, target);

            // now find block with value=target
            let mut cnt = 0;
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == target {
                        cnt += 1;
                    }
                }
            }
            ans.push(cnt);
        }

        ans
    }
}
*/


mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($grid:tt, $hits:tt, $ans:tt) => {
            assert_eq!(
                Solution::hit_bricks(vec2d!$grid, vec2d!$hits),
                vec!$ans
            );
        }
    };
    test!([[1, 0, 0, 0], [1, 1, 1, 0]], [[1, 0]], [2]);
    test!([[1, 0, 0, 0], [1, 1, 0, 0]], [[1, 1], [1, 0]], [0, 0]);
    test!([[1, 0, 1], [1, 1, 1]], [[0, 0], [0, 2], [1, 1]], [0, 3, 0]);
    test!(
        [[1, 1, 1], [0, 1, 0], [0, 0, 0]],
        [[0, 2], [2, 0], [0, 1], [1, 2]],
        [0, 0, 1, 0]
    );
}
