#
# @lc app=leetcode.cn id=25 lang=python3
#
# [25] K 个一组翻转链表
#
from prelude import *
# @lc code=start
from typing import Tuple as tuple


class Solution:

    def reverse_k_group(self, head: ListNode, tail: ListNode) -> tuple[ListNode, ListNode]:
        """翻转 [head, tail] 之间的链表。返回头部和尾部"""
        prev = None
        ptr = head
        while ptr != tail:
            next = ptr.next
            ptr.next = prev
            prev = ptr
            ptr = next
        # ptr == tail
        ptr.next = prev
        return ptr, head

    def reverseKGroup(self, head: ListNode, k: int) -> ListNode:
        if k <= 1:
            return head
        hyper = ListNode(0)
        hyper.next = head

        ptr = hyper
        while True:
            # 找到一组
            tail = ptr
            for i in range(k):
                tail = tail.next
                if tail is None:
                    return hyper.next
            next_group = tail.next
            group_head, group_tail = self.reverse_k_group(ptr.next, tail)
            ptr.next = group_head
            group_tail.next = next_group
            ptr = group_tail


# @lc code=end
if __name__ == '__main__':
    def test(l, k, expected):
        calc = Solution().reverseKGroup(ListNode.from_list(l), k)
        expected = ListNode.from_list(expected).to_string()
        if calc.to_string() != expected:
            print(f'case failed: `{(l, k)}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        [1, 2, 3, 4, 5],
        2,
        [2, 1, 4, 3, 5]
    )
    test(
        [1, 2, 3, 4, 5],
        3,
        [3, 2, 1, 4, 5]
    )
    test(
        [1, 2, 3, 4, 5],
        1,
        [1, 2, 3, 4, 5]
    )
