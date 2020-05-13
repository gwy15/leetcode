/*
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * [102] 二叉树的层次遍历
 */
struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
// @lc code=start
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut q = VecDeque::new();
        if let Some(ptr) = root {
            q.push_back(ptr);
        }
        while q.len() > 0 {
            let n = q.len();
            let mut level = Vec::new();
            for _ in 0..n {
                let ptr = q.pop_front().unwrap();
                let node = ptr.borrow();
                level.push(node.val.clone());
                if let Some(left) = &node.left {
                    q.push_back(left.clone());
                }
                if let Some(right) = &node.right {
                    q.push_back(right.clone());
                }
            }
            result.push(level);
        }

        result
    }
}
// @lc code=end
