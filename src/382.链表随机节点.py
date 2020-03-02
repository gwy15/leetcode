#
# @lc app=leetcode.cn id=382 lang=python3
#
# [382] 链表随机节点
#

# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

import random

class Solution:

    def __init__(self, head: ListNode):
        """
        @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node.
        """
        self.p = head
        

    def getRandom(self) -> int:
        """
        Returns a random node's value.
        """
        p = self.p
        count = 1
        val = None
        while p is not None:
            if random.random() < 1/count:
                val = p.val
            count += 1
            p = p.next
        return val
        


# Your Solution object will be instantiated and called as such:
# obj = Solution(head)
# param_1 = obj.getRandom()
# @lc code=end

