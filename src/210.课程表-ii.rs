/*
 * @lc app=leetcode.cn id=210 lang=rust
 *
 * [210] 课程表 II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        // init graph (next and in_rank)
        let mut next: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut in_degree: Vec<u32> = vec![0; n];
        for edge in prerequisites.iter() {
            // i <- j
            let (i, j) = (edge[0], edge[1]);
            next[j as usize].push(i as usize);
            in_degree[i as usize] += 1;
        }
        // extract zero in-degree nodes
        let mut orphans: Vec<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
        // sort
        let mut ans = Vec::new();
        while orphans.len() > 0 {
            let i = orphans.pop().unwrap();
            // i is the minium now
            ans.push(i as i32);
            // clear in rank
            for &j in &next[i] {
                in_degree[j] -= 1;
                if in_degree[j] == 0 {
                    orphans.push(j);
                }
            }
        }
        // return
        if ans.len() == n {
            ans
        } else {
            Vec::new()
        }
    }
}
// @lc code=end
#[test]
fn test() {
    macro_rules! vec2d {
        [$(
            [$(
                $i:expr
            ),*]
        ),*] => {
            vec![$(
                vec![$($i),*]
            ),*]
        }
    }
    macro_rules! test {
        ($n:expr, $pre:tt, $ans:tt) => {
            assert_eq!(
                Solution::find_order($n, vec2d!$pre),
                vec!$ans
            );
        };
    }
    test!(2, [[1, 0]], [0, 1]);
    test!(4, [[1, 0], [2, 0], [3, 1], [3, 2]], [0, 2, 1, 3]);
    // no answer
    test!(3, [[1, 0], [1, 2], [0, 1]], []);
}
