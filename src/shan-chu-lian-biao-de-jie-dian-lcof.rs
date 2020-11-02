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

struct Solution;

impl Solution {
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut hyper = Box::new(ListNode::new(0));
        hyper.next = head;
        let mut hyper = Some(hyper);

        let mut ptr = hyper.as_mut();
        while let Some(prev) = ptr {
            match prev.next.as_mut() {
                Some(next) => {
                    if next.val == val {
                        prev.next = next.next.take();
                    }
                }
                None => {}
            }
            ptr = prev.next.as_mut();
        }
        hyper.unwrap().next.take()
    }
}
