// Definition for singly-linked list.
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
// start code
struct Solution;

impl Solution {
    pub fn kth_to_last(head: Option<Box<ListNode>>, k: i32) -> i32 {
        // 第一个节点前进 k
        let (mut fast, mut slow) = (head.as_ref(), head.as_ref());
        for _ in 0..k {
            fast = fast.unwrap().next.as_ref();
        }
        loop {
            if fast.is_some() {
                fast = fast.unwrap().next.as_ref();
                slow = slow.unwrap().next.as_ref();
            } else {
                break slow.unwrap().val;
            }
        }
    }
}
