/*
 * @lc app=leetcode.cn id=1161 lang=rust
 *
 * [1161] 最大层内元素和
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
//
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut max_level, mut max_level_sum) = (1, i32::min_value());
        let mut level = 1;
        let mut q = VecDeque::new();
        q.push_back(root.unwrap());
        //
        while q.len() > 0 {
            let n = q.len();
            let mut level_sum = 0;
            for _ in 0..n {
                let node = q.pop_front().unwrap();
                level_sum += node.borrow().val;
                if let Some(left) = &node.borrow().left {
                    q.push_back(left.clone());
                }
                if let Some(right) = &node.borrow().right {
                    q.push_back(right.clone());
                };
            }
            if level_sum > max_level_sum {
                max_level_sum = level_sum;
                max_level = level;
            }
            level += 1;
        }

        max_level
    }
}
// @lc code=end
