/*
 * @lc app=leetcode.cn id=144 lang=rust
 *
 * [144] 二叉树的前序遍历
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut ptr = root;
        while let Some(node) = ptr {
            let node_b = node.borrow();
            if let Some(left) = node_b.left.clone() {
                // 遍历左侧，找到直接前置节点（左树最右）
                let mut predecessor = left;
                while predecessor.borrow().right.is_some()
                    && predecessor.borrow().right.as_ref().unwrap() != &node
                {
                    let next = predecessor.borrow().right.as_ref().unwrap().clone();
                    predecessor = next;
                }
                // 虽然很想写下面的写法，但是目前不行
                // while let Some(next) = predecessor.borrow().right.clone() && next != node {
                //     predecessor = next;
                // }

                // 如果直接前置没有后缀，说明还没遍历左树。否则说明已经遍历左树
                if predecessor.borrow().right.is_none() {
                    ans.push(node_b.val); // 前序遍历
                    predecessor.borrow_mut().right = Some(node.clone());
                    ptr = node_b.left.clone(); // 遍历左树
                } else {
                    predecessor.borrow_mut().right = None; // 解除链接
                    ptr = node_b.right.clone(); // 遍历右树
                }
            } else {
                // 处理左侧为空
                ans.push(node_b.val); // 前序遍历
                ptr = node_b.right.clone(); // 遍历右树
                continue;
            }
        }
        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    let mut root = TreeNode::new(2);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    assert_eq!(
        Solution::preorder_traversal(Some(Rc::new(RefCell::new(root)))),
        vec![2, 1, 3]
    );
}
