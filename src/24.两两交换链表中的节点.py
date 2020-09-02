#
# @lc app=leetcode.cn id=24 lang=python3
#
# [24] 两两交换链表中的节点
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def swapPairs(self, head: ListNode) -> ListNode:
        hyper = ListNode(0)
        hyper.next = head
        # swap
        ptr = hyper
        while ptr.next and ptr.next.next:
            a, b = ptr.next, ptr.next.next
            far = b.next
            # swap a and b
            ptr.next = b
            b.next = a
            a.next = far
            ptr = a

        return hyper.next


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().swapPairs(ListNode.from_list(input))
        expected = str(ListNode.from_list(expected))
        if str(calc) != str(expected):
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 2, 3, 4], [2, 1, 4, 3])
    test([1, 2, 3, 4, 5, 6, 7], [2, 1, 4, 3, 6, 5, 7])
    test([], [])
