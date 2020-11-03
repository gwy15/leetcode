#
# @lc app=leetcode.cn id=148 lang=python3
#
# [148] 排序链表
#
from prelude import *
# @lc code=start


class Solution:
    def mergeSortedList(self, p1: ListNode, p2: ListNode) -> ListNode:
        hyper = ListNode(0)
        hyper.next = None
        ptr = hyper
        while p1 and p2:
            if p1.val < p2.val:
                ptr.next = p1
                p1 = p1.next
            else:
                ptr.next = p2
                p2 = p2.next
            # unlink ptr.next -> rest
            ptr.next.next = None
            ptr = ptr.next
        if p1:
            ptr.next = p1
        if p2:
            ptr.next = p2

        return hyper.next

    def evenSplit(self, head: ListNode) -> ListNode:
        # [1,2,3,4] => [1,2]    [3,4]
        # [1,2,3]   => [1,2]    [2]
        # [1,2]     => [1]      [2]
        # [1]       => [1]      []
        fast, slow = head.next, head
        while fast:
            fast = fast.next
            if fast:
                slow = slow.next
                fast = fast.next
        # [.... slow] -> [ans ...]
        ans = slow.next
        slow.next = None
        return ans

    def sortList(self, head: ListNode) -> ListNode:
        if head is None or head.next is None:
            return head
        first, second = head, self.evenSplit(head)
        first = self.sortList(first)
        second = self.sortList(second)
        return self.mergeSortedList(first, second)


# @lc code=end
if __name__ == '__main__':
    def test(input):
        vals = input
        calc = Solution().sortList(ListNode.from_list(vals))
        expected = ListNode.from_list(sorted(vals))
        if calc.to_string() != expected.to_string():
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    # test([])
    test([1])
    test([2, 1])
    test([1, 2])
    test([1, 2, 3])
    test([3, 2, 1])
    test([4, 2, 1, 3])
