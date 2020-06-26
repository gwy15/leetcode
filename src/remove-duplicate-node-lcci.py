class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None
# Definition for singly-linked list.


class Solution:
    def removeDuplicateNodes(self, head: ListNode) -> ListNode:
        seen = set()
        ans = ListNode(None)
        ans_ptr = ans

        ptr = head
        while ptr:
            if ptr.val not in seen:
                seen.add(ptr.val)
                ans_ptr.next = ListNode(ptr.val)
                ans_ptr = ans_ptr.next
            ptr = ptr.next

        return ans.next
