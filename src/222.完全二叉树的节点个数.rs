/*
 * @lc app=leetcode.cn id=222 lang=rust
 *
 * [222] 完全二叉树的节点个数
 */
mod utils;
use utils::TreeNode;
struct Solution;
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn count_nodes_tail(root: Option<Rc<RefCell<TreeNode>>>, mut count: i32) -> i32 {
        match root {
            None => count,
            Some(rrt) => {
                count += 1;
                count = Self::count_nodes_tail(rrt.borrow().left.clone(), count);
                Self::count_nodes_tail(rrt.borrow().right.clone(), count)
            }
        }
    }
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::count_nodes_tail(root, 0)
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($t:tt, $ans:expr) => {
            assert_eq!(Solution::count_nodes(btree!($t)), $ans);
        };
    };
    test!([1, 2, 3, 4, 5, 6], 6);
}
