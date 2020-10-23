#
# @lc app=leetcode.cn id=234 lang=python3
#
# [234] 回文链表
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    @staticmethod
    def reverse(head: ListNode) -> ListNode:
        prev, ptr = None, head
        while ptr:
            n = ptr.next
            ptr.next = prev
            prev = ptr
            ptr = n
        return prev

    def isPalindrome(self, head: ListNode) -> bool:
        # find middle point
        fast, slow = head, head
        while fast:
            fast = fast.next
            slow = slow.next
            if fast:
                fast = fast.next
        # the latter half (include middle point)
        middle = slow

        # reverse the latte half
        rev = self.reverse(middle)

        # compare
        left, right = head, rev
        while right:
            if left.val != right.val:
                return False
            left = left.next
            right = right.next

        return True


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().isPalindrome(ListNode.from_list(input))
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([], True)
    test([1], True)
    test([1, 2], False)
    test([1, 2, 1], True)
    test([1, 2, 2, 1], True)
    test([1, 2, 3, 2, 1], True)
    test([1, 2, 3, 3, 2, 1], True)
    test([1, 2, 3, 4, 2, 1], False)
