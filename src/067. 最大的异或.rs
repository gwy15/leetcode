#[allow(unused)]
struct Solution;

// start

struct TreeNode {
    zero: Option<Box<TreeNode>>,
    one: Option<Box<TreeNode>>,
}
impl TreeNode {
    #[inline]
    pub fn new() -> Self {
        Self {
            zero: None,
            one: None,
        }
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        // 对于每个元素，尝试每个最大的可能
        let n = nums.len();
        let mut ans = 0;

        let root = Self::make_nodes(&nums);

        for x in nums {
            ans = ans.max(Self::find_max_pair(&root, x));
        }

        ans
    }

    fn make_nodes(nums: &[i32]) -> TreeNode {
        let mut root = TreeNode::new();
        for x in nums {
            let mut ptr = &mut root;
            for i in (0..32).rev() {
                match ((x >> i) & 1) == 1 {
                    true => {
                        if ptr.one.is_none() {
                            ptr.one = Some(Box::new(TreeNode::new()));
                        }
                        ptr = ptr.one.as_mut().unwrap();
                    }
                    false => {
                        if ptr.zero.is_none() {
                            ptr.zero = Some(Box::new(TreeNode::new()));
                        }
                        ptr = ptr.zero.as_mut().unwrap();
                    }
                }
            }
        }

        root
    }

    fn find_max_pair(root: &TreeNode, x: i32) -> i32 {
        let mut ans = 0;
        let mut ptr = root;
        for i in (0..32).rev() {
            // 对每个位判定
            let bit = ((x >> i) & 1) == 1;
            ptr = if bit {
                if let Some(zero) = ptr.zero.as_ref() {
                    ans += 1 << i;
                    zero
                } else {
                    ptr.one.as_ref().unwrap()
                }
            } else {
                if let Some(one) = ptr.one.as_ref() {
                    ans += 1 << i;
                    one
                } else {
                    ptr.zero.as_ref().unwrap()
                }
            };
        }

        ans
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(
                Solution::find_maximum_xor(vec!$args),
                $ans
            )
        };
    }
    test!([3, 10, 5, 25, 2, 8], 28);
    test!([0], 0);
    test!([2, 4], 6);
    test!([8, 10, 2], 10);
    test!([14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70], 127);
}
