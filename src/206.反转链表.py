#
# @lc app=leetcode.cn id=206 lang=python3
#
# [206] 反转链表
#

# Definition for singly-linked list.


class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

    def __repr__(self):
        s = f'{self.val}'
        if self.next:
            s += ' -> ' + str(self.next)
        return s
# @lc code=start


class Solution:
    def reverseList(self, head: ListNode) -> ListNode:
        if head is None:
            return None
        last_node = None
        ptr = head
        while ptr.next is not None:
            next_ptr = ptr.next
            ptr.next = last_node
            last_node = ptr
            ptr = next_ptr
        ptr.next = last_node
        return ptr

# @lc code=end


def list_2_node(nums):
    head = ListNode(None)
    ptr = head
    for num in nums:
        node = ListNode(num)
        ptr.next = node
        ptr = ptr.next
    return head.next


nums = [1, 2, 3, 4, 5]
node = list_2_node(nums)
print(node)

r = Solution().reverseList(node)
print(r)
