/*
 * @lc app=leetcode.cn id=1145 lang=rust
 *
 * [1145] 二叉树着色游戏
 */
mod utils;
use utils::TreeNode;
struct Solution;
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
type RRT = Rc<RefCell<TreeNode>>;
type ORRT = Option<RRT>;
impl Solution {
    fn count_node(node: ORRT, count: &mut [usize], nodes: &mut [ORRT]) -> usize {
        match node.clone() {
            None => 0,
            Some(rrt) => {
                let node_b = rrt.borrow();
                let left = node_b.left.clone();
                let right = node_b.right.clone();
                let c = 1
                    + Self::count_node(left, count, nodes)
                    + Self::count_node(right, count, nodes);
                count[(node_b.val - 1) as usize] = c;
                nodes[(node_b.val - 1) as usize] = node.clone();
                c
            }
        }
    }

    pub fn btree_game_winning_move(root: ORRT, n: i32, x: i32) -> bool {
        let n = n as usize;
        let mut count = vec![0; n];
        let mut nodes = vec![None; n];
        Self::count_node(root.clone(), &mut count, &mut nodes);
        // up
        let x = x as usize - 1;
        let up_nodes = n - count[x];
        if up_nodes > n / 2 {
            return true;
        }
        // find left
        let left = nodes[x].clone().unwrap().borrow().left.clone();
        let left_nodes = match left {
            None => 0,
            Some(rrt) => count[rrt.borrow().val as usize - 1],
        };
        if left_nodes > n / 2 {
            return true;
        }
        // right
        let right = nodes[x].clone().unwrap().borrow().right.clone();
        let right_nodes = match right {
            None => 0,
            Some(rrt) => count[rrt.borrow().val as usize - 1],
        };
        if right_nodes > n / 2 {
            return true;
        }

        false
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($r:tt, $x:expr, $ans:expr) => {
            let t = btree!($r);
            let n = $r.len();
            assert_eq!(Solution::btree_game_winning_move(t, n as i32, $x), $ans);
        };
    };
    test!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 3, true);
}
