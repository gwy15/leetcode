#
# @lc app=leetcode.cn id=2 lang=python3
#
# [2] 两数相加
#

# Definition for singly-linked list.


class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None
# @lc code=start


class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        node = ListNode(0)
        head = node

        rest = 0
        while l1 or l2 or rest:
            val = rest
            if l1:
                val += l1.val
                l1 = l1.next
            if l2:
                val += l2.val
                l2 = l2.next
            rest = val // 10

            node.next = ListNode(val % 10)
            node = node.next

        return head.next


# @lc code=end
