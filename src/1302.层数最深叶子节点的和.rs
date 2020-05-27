/*
 * @lc app=leetcode.cn id=1302 lang=rust
 *
 * [1302] 层数最深叶子节点的和
 */

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
struct Solution;
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(unused)]
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut max_depth, mut ans) = (0, 0);

        let mut stack: Vec<(i32, Rc<RefCell<TreeNode>>)> = Vec::new();
        stack.push((0, root.unwrap()));

        while stack.len() > 0 {
            let (depth, node) = stack.pop().unwrap();
            if depth >= max_depth {
                if depth > max_depth {
                    max_depth = depth;
                    ans = 0;
                }
                ans += node.borrow().val;
            }
            if let Some(left) = &node.borrow().left {
                stack.push((depth + 1, left.clone()));
            }
            if let Some(right) = &node.borrow().right {
                stack.push((depth + 1, right.clone()));
            };
        }

        ans
    }
}
// @lc code=end
