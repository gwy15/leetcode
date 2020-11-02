#
# @lc app=leetcode.cn id=83 lang=python3
#
# [83] 删除排序链表中的重复元素
#
from prelude import *
# @lc code=start


class Solution:
    def deleteDuplicates(self, head: ListNode) -> ListNode:
        hyper = ListNode(0)
        hyper.next = head

        ptr = hyper
        while ptr.next:
            next_node = ptr.next
            while next_node.next and next_node.next.val == next_node.val:
                # remove next_node.next
                next_node.next = next_node.next.next

            ptr = next_node

        return hyper.next
# @lc code=end
