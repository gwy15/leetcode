#
# @lc app=leetcode.cn id=19 lang=python3
#
# [19] 删除链表的倒数第N个节点
#
# Definition for singly-linked list.
from utils import ListNode
# @lc code=start


class Solution:
    def removeNthFromEnd(self, head: ListNode, n: int) -> ListNode:
        hyperhead = ListNode(0)
        hyperhead.next = head

        fast = hyperhead
        for i in range(n):
            fast = fast.next
        slow = hyperhead
        while fast.next is not None:
            fast = fast.next
            slow = slow.next
        slow.next = slow.next.next
        return hyperhead.next


# @lc code=end
if __name__ == "__main__":
    def test(vals, n, ans):
        l = ListNode.from_list(vals)
        a = Solution().removeNthFromEnd(l, n)
        assert a == ListNode.from_list(ans)
    test([1, 2, 3, 4, 5], 2, [1, 2, 3, 5])
    test([1, 2], 1, [1])
    test([1, 2], 2, [2])
