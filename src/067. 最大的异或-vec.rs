#[allow(unused)]
struct Solution;

// start

const NONE: usize = 0;

#[derive(Debug, Clone)]
struct TreeNode {
    zero: usize,
    one: usize,
}
impl TreeNode {
    #[inline]
    pub const fn new() -> Self {
        Self {
            zero: NONE,
            one: NONE,
        }
    }
}

struct Tree {
    buffer: Vec<TreeNode>,
}
impl Tree {
    pub fn new(n: usize) -> Self {
        Self {
            // 对于本题目，节点数不会超过树高*节点数，即 32 n
            buffer: vec![TreeNode::new(); n * 32 + 2],
        }
    }
    pub fn build(&mut self, nums: &[i32]) {
        let mut next = 1;

        for x in nums {
            let mut ptr = 1;
            for i in (0..32).rev() {
                match ((x >> i) & 1) == 1 {
                    true => {
                        if self.buffer[ptr].one == NONE {
                            // allocate new
                            next += 1;
                            self.buffer[ptr].one = next;
                        }
                        ptr = self.buffer[ptr].one;
                    }
                    false => {
                        if self.buffer[ptr].zero == NONE {
                            next += 1;
                            self.buffer[ptr].zero = next;
                        }
                        ptr = self.buffer[ptr].zero;
                    }
                }
            }
        }
    }

    pub fn find_max_pair(&self, x: i32) -> i32 {
        let mut ans = 0;
        let mut ptr = 1;
        for i in (0..32).rev() {
            // 对每个位判定
            let bit = ((x >> i) & 1) == 1;
            ptr = if bit {
                if self.buffer[ptr].zero != NONE {
                    ans += 1 << i;
                    self.buffer[ptr].zero
                } else {
                    self.buffer[ptr].one
                }
            } else {
                if self.buffer[ptr].one != NONE {
                    ans += 1 << i;
                    self.buffer[ptr].one
                } else {
                    self.buffer[ptr].zero
                }
            };
        }

        ans
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        // 对于每个元素，尝试每个最大的可能
        let n = nums.len();
        let mut tree = Tree::new(n);
        tree.build(&nums);

        let mut ans = 0;
        for x in nums {
            ans = ans.max(tree.find_max_pair(x));
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
