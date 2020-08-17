#
# @lc app=leetcode.cn id=141 lang=python3
#
# [141] 环形链表
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def hasCycle(self, head: ListNode) -> bool:
        if head is None:
            return False
        fast = head.next
        slow = head
        while fast != slow:
            if fast is None or fast.next is None:
                return False
            fast = fast.next.next
            slow = slow.next
        return True


# @lc code=end
if __name__ == "__main__":
    def test(vals, pos, ans):
        h = ListNode.from_list(vals, pos)
        assert Solution().hasCycle(h) == ans
    test([3, 2, 0, -4], 1, True)
    test([1, 2], 0, True)
    test([1], -1, False)
    test(
        [-21, 10, 17, 8, 4, 26, 5, 35, 33, -7, -16, 27, -12, 6,
            29, -12, 5, 9, 20, 14, 14, 2, 13, -24, 21, 23, -21, 5],
        -1, False)
