pub mod binary_tree {
    use std::cell::RefCell;
    use std::rc::Rc;

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

    pub fn parse(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        if s == "[]" {
            return None;
        }
        let mut data_provider = s[1..s.len() - 1].split(',');
        // make root
        let root_str = data_provider.next().unwrap().trim();
        let root_val = root_str
            .parse()
            .expect(&format!("Unexpected string: {}", root_str));
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        // BFS
        let mut q = std::collections::VecDeque::new();
        q.push_back(root.clone());
        macro_rules! make {
            ($node:expr, $left_or_right:tt) => {
                match data_provider.next() {
                    None => break,
                    Some(val_str) => match val_str.trim() {
                        "null" => {}
                        val_str => {
                            let val = val_str
                                .parse()
                                .expect(&format!("Unexpected val: {}", val_str));
                            let new_node = TreeNode::new(val);
                            let node_ptr = Rc::new(RefCell::new(new_node));
                            $node.borrow_mut().$left_or_right = Some(node_ptr.clone());
                            q.push_back(node_ptr);
                        }
                    },
                }
            };
        }
        while q.len() > 0 {
            let node = q.pop_front().unwrap();
            make!(node, left);
            make!(node, right);
        }

        Some(root)
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_parse() {
            // []
            assert!(parse("[]").is_none());
            // [1,2,3]
            let t = parse("[1,2,3]").unwrap();
            assert_eq!(t.borrow().val, 1);
            assert_eq!(t.borrow().left.as_ref().unwrap().borrow().val, 2);
            assert_eq!(t.borrow().right.as_ref().unwrap().borrow().val, 3);
            // [1,null,2,3]
            let t = parse("[1, null, 2, 3]").unwrap();
            assert_eq!(t.borrow().val, 1);
            assert_eq!(t.borrow().right.as_ref().unwrap().borrow().val, 2);
            assert_eq!(
                t.borrow()
                    .right
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .left
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .val,
                3
            );
        }
    }
}

// export
pub use binary_tree::TreeNode;

pub mod macros {
    #[macro_export]
    macro_rules! vec2d {
        [$(
            [$(
                $i:expr
            ),*]
        ),*] => {
            vec![$(
                vec![$($i),*]
            ),*]
        }
    }

    #[macro_export]
    macro_rules! btree {
        ($tree:tt) => {
            crate::utils::binary_tree::parse(stringify!($tree))
        };
    }
}
