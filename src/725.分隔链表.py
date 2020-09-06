#
# @lc app=leetcode.cn id=725 lang=python3
#
# [725] 分隔链表
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def splitListToParts(self, root: ListNode, k: int) -> List[ListNode]:
        hyper = ListNode(0)
        hyper.next = root
        # get length
        ptr = hyper
        length = 0
        while ptr.next is not None:
            length += 1
            ptr = ptr.next
        # get segment length
        segment_length = length // k
        long_segments = length % k
        ans = []
        # long segments
        ptr = hyper.next
        for i in range(long_segments):
            ans.append(ptr)
            # find tail
            for _ in range(segment_length):
                ptr = ptr.next
            next = ptr.next
            # unlink with rest
            ptr.next = None
            ptr = next

        if segment_length == 0:
            for i in range(k - long_segments):
                ans.append(None)
        else:
            for i in range(k - long_segments):
                ans.append(ptr)
                # find tail
                for _ in range(segment_length - 1):
                    ptr = ptr.next
                next = ptr.next
                # unlink with rest
                ptr.next = None
                ptr = next
        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, k, expected):
        calc = Solution().splitListToParts(ListNode.from_list(input), k)
        expected = [ListNode.from_list(i) for i in expected]
        if list(map(str, calc)) != list(map(str, expected)):
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        [1, 2, 3], 5,
        [[1], [2], [3], [], []])
    test(
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3,
        [[1, 2, 3, 4], [5, 6, 7], [8, 9, 10]]
    )
    test(
        [], 3,
        [[], [], []]
    )
