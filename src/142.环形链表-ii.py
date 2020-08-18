#
# @lc app=leetcode.cn id=142 lang=python3
#
# [142] 环形链表 II
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def detectCycle(self, head: ListNode) -> ListNode:
        fast, slow = head, head
        while fast is not None and fast.next is not None:
            fast = fast.next.next
            slow = slow.next
            if fast == slow:
                break
        if fast is None or fast.next is None:
            return None

        fast = head
        while fast != slow:
            fast = fast.next
            slow = slow.next
        return fast


# @lc code=end
