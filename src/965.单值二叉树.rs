/*
 * @lc app=leetcode.cn id=965 lang=rust
 *
 * [965] 单值二叉树
 */

// Definition for a binary tree node.
// use utils::TreeNode;
mod utils;
use utils::TreeNode;

struct Solution;
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused)]
impl Solution {
    fn verify(root: Rc<RefCell<TreeNode>>, val: i32) -> bool {
        if root.borrow().val != val {
            return false;
        }
        if let Some(left) = &root.borrow().left {
            if !Self::verify(left.clone(), val) {
                return false;
            }
        }
        if let Some(right) = &root.borrow().right {
            if !Self::verify(right.clone(), val) {
                return false;
            }
        }
        true
    }
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => Self::verify(root.clone(), root.borrow().val),
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($tree:expr, $ans:expr) => {
            let t = crate::utils::binary_tree::parse($tree);
            assert_eq!(Solution::is_unival_tree(t), $ans);
        };
    };
    test!("[]", true);
    test!("[1,1,1,1,1,null,1]", true);
    test!("[2,2,2,5,2]", false);
}
