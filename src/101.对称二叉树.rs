/*
 * @lc app=leetcode.cn id=101 lang=rust
 *
 * [101] 对称二叉树
 */
mod utils;
use utils::TreeNode;
struct Solution;
// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;

type RRT = Rc<RefCell<TreeNode>>;
impl Solution {
    fn is_symmetric2(left: Option<RRT>, right: Option<RRT>) -> bool {
        match (left, right) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(left), Some(right)) => {
                if left.borrow().val != right.borrow().val {
                    false
                } else {
                    Self::is_symmetric2(left.borrow().left.clone(), right.borrow().right.clone())
                        && Self::is_symmetric2(
                            left.borrow().right.clone(),
                            right.borrow().left.clone(),
                        )
                }
            }
        }
    }
    #[allow(unused)]
    pub fn is_symmetric(root: Option<RRT>) -> bool {
        match root {
            None => true,
            Some(root) => {
                Self::is_symmetric2(root.borrow().left.clone(), root.borrow().right.clone())
            }
        }
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($tree:tt, $ans:expr) => {
            assert_eq!(Solution::is_symmetric(btree!($tree)), $ans);
        };
    };
    test!([1, 2, 2, 3, 4, 4, 3], true);
    test!([1, 2, 2, null, 3, null, 3], false);
    test!([1, 2, 2, null, 3, 3], true);
}
