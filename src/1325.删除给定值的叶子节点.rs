/*
 * @lc app=leetcode.cn id=1325 lang=rust
 *
 * [1325] 删除给定值的叶子节点
 */
struct Solution;
mod utils;
use utils::TreeNode;
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(unused)]
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(root) => {
                let mut r = root.borrow_mut();
                r.left = Self::remove_leaf_nodes(r.left.clone(), target);
                r.right = Self::remove_leaf_nodes(r.right.clone(), target);
                if r.val == target && r.left.is_none() && r.right.is_none() {
                    None
                } else {
                    std::mem::drop(r);
                    Some(root)
                }
            }
        }
    }
}
// @lc code=end

#[test]
fn test_solution() {
    macro_rules! test {
        ($tree:tt, $target:expr, $ans:tt) => {
            assert_eq!(
                Solution::remove_leaf_nodes(btree!($tree), $target),
                btree!($ans)
            );
        };
    };
    test!([1, 2, 3, 2, null, 2, 4], 2, [1, null, 3, null, 4]);
    test!([1, 3, 3, 3, 2], 3, [1, 3, null, null, 2]);
    test!([1, 2, null, 2, null, 2], 2, [1]);
    test!([1, 1, 1], 1, []);
    test!([1, 2, 3], 1, [1, 2, 3]);
}
