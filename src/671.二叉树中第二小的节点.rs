/*
 * @lc app=leetcode.cn id=671 lang=rust
 *
 * [671] 二叉树中第二小的节点
 */

mod utils;
use utils::TreeNode;
struct Solution;
// @lc code=start
use std::cell::RefCell;
use std::cmp::Ordering::*;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    #[allow(unused)]
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return -1;
        }
        let root = root.unwrap();
        let root_val = root.borrow().val;
        let mut ans = -1;
        let mut q = VecDeque::new();
        q.push_back(root);
        while q.len() > 0 {
            let node = q.pop_front().unwrap();
            match node.borrow().val.cmp(&root_val) {
                Less => unreachable!(),
                Equal => {
                    // search more
                    if let Some(left) = &node.borrow().left {
                        q.push_back(left.clone());
                    }
                    if let Some(right) = &node.borrow().right {
                        q.push_back(right.clone());
                    };
                }
                Greater => {
                    ans = if ans == -1 {
                        node.borrow().val
                    } else {
                        ans.min(node.borrow().val)
                    };
                }
            };
        }

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    use utils::binary_tree::parse;
    macro_rules! test {
        ($tree:tt, $ans:expr) => {
            assert_eq!(
                Solution::find_second_minimum_value(parse(stringify!($tree))),
                $ans
            );
        };
    };
    test!([2, 2, 5, null, null, 5, 7], 5);
    test!([2, 2, 2], -1);
    test!([2, 2, 2147483647], 2147483647);
}
