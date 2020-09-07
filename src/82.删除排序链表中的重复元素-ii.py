#
# @lc app=leetcode.cn id=82 lang=python3
#
# [82] 删除排序链表中的重复元素 II
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def deleteDuplicates(self, head: ListNode) -> ListNode:
        hyper = ListNode(0)
        hyper.next = head

        #
        ptr = hyper
        while ptr.next:
            val = ptr.next.val
            # count
            p, cnt = ptr, 0
            while p.next and p.next.val == val:
                p = p.next
                cnt += 1

            if cnt == 1:
                ptr = ptr.next
            else:
                # unlink
                ptr.next = p.next

        return hyper.next


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().deleteDuplicates(ListNode.from_list(input))
        if str(calc) != str(ListNode.from_list(expected)):
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 2, 3, 3, 4, 4, 5], [1, 2, 5])
    test([1, 1, 1, 2, 3], [2, 3])
    test([1, 1], [])
    test([1, 1, 1, 2, 3, 3], [2])
