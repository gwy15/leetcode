/*
 * @lc app=leetcode.cn id=1290 lang=rust
 *
 * [1290] 二进制链表转整数
 */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

// @lc code=start
// Definition for singly-linked list.
impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut ans = 0;
        while let Some(node) = head {
            ans = (ans << 1) + node.val;
            head = node.next;
        }
        ans
    }
}
// @lc code=end
