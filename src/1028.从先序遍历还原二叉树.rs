/*
 * @lc app=leetcode.cn id=1028 lang=rust
 *
 * [1028] 从先序遍历还原二叉树
 */
mod utils;
use utils::TreeNode;

struct Solution;

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;

struct Parse {
    chars: Vec<u8>,
    i: usize,
}

impl Parse {
    pub fn from(s: String) -> Self {
        Self {
            chars: s.into_bytes(),
            i: 0,
        }
    }
}

impl std::iter::Iterator for Parse {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.chars.len() {
            return None;
        }
        // find dashes
        let mut depth = 0;
        while self.i < self.chars.len() && self.chars[self.i] == ('-' as u8) {
            depth += 1;
            self.i += 1;
        }
        let mut number = 0;
        while self.i < self.chars.len() && self.chars[self.i].is_ascii_digit() {
            number = number * 10 + (self.chars[self.i] - '0' as u8) as i32;
            self.i += 1;
        }

        Some((depth, number))
    }
}

impl Solution {
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let super_root = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut stack = vec![(-1, super_root.clone())];
        //
        for (depth, val) in Parse::from(s) {
            while depth <= stack.last().unwrap().0 {
                stack.pop();
            }
            let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
            let last_layer = stack.last().unwrap();
            if last_layer.1.borrow().left.is_none() {
                last_layer.1.borrow_mut().left = Some(new_node.clone());
            } else {
                last_layer.1.borrow_mut().right = Some(new_node.clone());
            }
            stack.push((depth, new_node));
        }

        let root = super_root.borrow_mut().left.clone();
        root
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $ans:tt) => {
            assert_eq!(
                Solution::recover_from_preorder($s.to_string()),
                btree!($ans)
            );
        };
    };
    test!("1-2--3--4-5--6--7", [1, 2, 5, 3, 4, 6, 7]);
    test!(
        "1-2--3---4-5--6---7",
        [1, 2, 5, 3, null, 6, null, 4, null, 7]
    );
    test!("1-401--349---90--88", [1, 401, null, 349, 88, 90]);
}
