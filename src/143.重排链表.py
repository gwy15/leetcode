#
# @lc app=leetcode.cn id=143 lang=python3
#
# [143] 重排链表
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def reorderList(self, head: ListNode) -> None:
        """
        Do not return anything, modify head in-place instead.
        """
        hyper = ListNode(0)
        hyper.next = head
        # find length
        l, slow, fast = 0, hyper, hyper
        while fast.next:
            fast = fast.next
            slow = slow.next
            l += 1
            if fast.next:
                fast = fast.next
        # reverse the rear part
        last, ptr = None, slow.next
        slow.next = None
        while ptr:
            n = ptr.next
            ptr.next = last
            last = ptr
            ptr = n
        rear = last

        # join the two
        ptr = head
        while ptr and rear:
            n = ptr.next
            ptr.next = rear
            rear_next = rear.next
            rear.next = n
            ptr = n
            rear = rear_next

        return hyper.next


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        l = ListNode.from_list(input)
        Solution().reorderList(l)
        calc = l.to_string()
        if calc != ListNode.from_list(expected).to_string():
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    # test([], [])
    test([1], [1])
    test([1, 2], [1, 2])
    test([1, 2, 3], [1, 3, 2])
    test([1, 2, 3, 4], [1, 4, 2, 3])
    test([1, 2, 3, 4, 5], [1, 5, 2, 4, 3])
    test([1, 2, 3, 4, 5, 6], [1, 6, 2, 5, 3, 4])
