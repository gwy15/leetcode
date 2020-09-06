#
# @lc app=leetcode.cn id=86 lang=python3
#
# [86] 分隔链表
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def partition(self, head: ListNode, x: int) -> ListNode:
        ptr = head
        hyper_less, hyper_ge = ListNode(0), ListNode(0)
        ptr_less, ptr_ge = hyper_less, hyper_ge
        while ptr:
            val = ptr.val
            # unlink ptr from linked list
            next_ptr, ptr.next = ptr.next, None
            if val < x:
                ptr_less.next = ptr
                ptr_less = ptr
            else:
                ptr_ge.next = ptr
                ptr_ge = ptr

            ptr = next_ptr
        # link less with greater/equal
        ptr_less.next = hyper_ge.next
        return hyper_less.next


# @lc code=end
if __name__ == '__main__':
    def test(input, x, expected):
        calc = Solution().partition(ListNode.from_list(input), x)
        if str(calc) != str(ListNode.from_list(expected)):
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        [1, 4, 3, 2, 5, 2],
        3,
        [1, 2, 2, 4, 3, 5]
    )
    test(
        [1, 2, 3],
        0,
        [1, 2, 3]
    )
    test(
        [0, 0, 0], 0, [0, 0, 0]
    )
    test([], 1, [])
    test([1, 2, 3], 5, [1, 2, 3])
