/*
 * @lc app=leetcode.cn id=52 lang=rust
 *
 * [52] N皇后 II
 */

// @lc code=start
struct Occupied {
    n: usize,
    // pub row: Vec<bool>,
    _col: Vec<bool>,
    _diag: Vec<bool>,
    _rdiag: Vec<bool>,
}

impl Occupied {
    pub fn new(n: i32) -> Self {
        let n = n as usize;
        Self {
            n,
            // row: vec![false; n],
            _col: vec![false; n],
            _diag: vec![false; 2 * n - 1],
            _rdiag: vec![false; 2 * n - 1],
        }
    }

    #[inline(always)]
    pub fn col(&mut self, col: usize) -> &mut bool {
        &mut self._col[col]
    }

    #[inline(always)]
    pub fn diag(&mut self, x: usize, y: usize) -> &mut bool {
        &mut self._diag[x + y]
    }

    #[inline(always)]
    pub fn rdiag(&mut self, x: usize, y: usize) -> &mut bool {
        &mut self._rdiag[x + self.n - 1 - y]
    }
}

#[allow(unused)]
impl Solution {
    #[inline(always)]
    fn ok(row: usize, col: usize, occupied: &mut Occupied) -> bool {
        !*occupied.col(col) && !*occupied.diag(row, col) && !*occupied.rdiag(row, col)
    }

    #[inline(always)]
    fn render(poses: &Vec<usize>) -> Vec<String> {
        let n = poses.len();
        let mut r = vec![];
        for &p in poses {
            let mut chars = vec!['.'; n];
            chars[p] = 'Q';
            r.push(chars.into_iter().collect());
        }
        r
    }

    /// try dfs from row
    fn dfs(
        n: usize,
        row: usize,
        poses: &mut Vec<usize>,
        occupied: &mut Occupied,
        ans: &mut Vec<Vec<String>>,
    ) {
        // try put in each row
        for pos in 0..n {
            // try pos at row
            if Self::ok(row, pos, occupied) {
                if row == n - 1 {
                    poses.push(pos);
                    ans.push(Self::render(poses));
                    poses.pop();
                    // only one ans possible
                    return;
                }
                // update occupied, these flags must be false
                *occupied.col(pos) = true;
                *occupied.diag(row, pos) = true;
                *occupied.rdiag(row, pos) = true;
                poses.push(pos);
                // dfs
                Self::dfs(n, row + 1, poses, occupied, ans);
                // reset
                poses.pop();
                *occupied.col(pos) = false;
                *occupied.diag(row, pos) = false;
                *occupied.rdiag(row, pos) = false;
            }
        }
    }

    ///
    pub fn total_n_queens(n: i32) -> i32 {
        let mut ans = vec![];
        let mut o = Occupied::new(n);
        let mut poses = Vec::new();
        Self::dfs(n as usize, 0, &mut poses, &mut o, &mut ans);
        ans.len() as i32
    }
}
// @lc code=end
