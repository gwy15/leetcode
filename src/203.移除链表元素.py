#
# @lc app=leetcode.cn id=203 lang=python3
#
# [203] 移除链表元素
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def removeElements(self, head: ListNode, val: int) -> ListNode:
        hyper = ListNode(0)
        hyper.next = head
        #
        p = hyper
        while p:
            while p.next and p.next.val == val:
                p.next = p.next.next
            p = p.next

        return hyper.next


# @lc code=end
if __name__ == '__main__':
    def test(input, val, expected):
        head = ListNode.from_list(input)
        calc = Solution().removeElements(head, val)
        expected = ListNode.from_list(expected)
        if str(calc) != str(expected):
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        [1, 2, 6, 3, 4, 5, 6],
        6,
        [1, 2, 3, 4, 5]
    )
    test(
        [1, 1, 2, 1, 1],
        1,
        [2]
    )
    test(
        [1],
        1,
        []
    )
    test(
        [],
        1,
        []
    )
