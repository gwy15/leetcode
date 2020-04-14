#
# @lc app=leetcode.cn id=445 lang=python3
#
# [445] 两数相加 II
#

# Definition for singly-linked list.


class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

    def __repr__(self):
        if self.next is not None:
            return '({}) -> {}'.format(self.val, self.next)
        else:
            return '[{}]'.format(self.val)
# @lc code=start


class Solution:
    @staticmethod
    def lengthOfListNode(root):
        length = 1
        while root is not None:
            root = root.next
            length += 1
        return length

    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        len1 = Solution.lengthOfListNode(l1)
        len2 = Solution.lengthOfListNode(l2)

        cur = None
        if len1 > len2:
            for _ in range(len1 - len2):
                new_cur = ListNode(l1.val)
                new_cur.next = cur
                l1, cur = l1.next, new_cur
        else:
            for _ in range(len2 - len1):
                new_cur = ListNode(l2.val)
                new_cur.next = cur
                l2, cur = l2.next, new_cur
        while l1 != None:
            new_cur = ListNode(l1.val + l2.val)
            new_cur.next = cur
            l1, l2, cur = l1.next, l2.next, new_cur
        # do the flip
        head = None
        n = 0
        while cur is not None:
            cur.val += n
            n = cur.val // 10
            cur.val %= 10
            #
            next_cur = cur.next
            cur.next = head
            cur, head = next_cur, cur
        if n != 0:
            node = ListNode(n)
            node.next = head
            head = node
        return head

# @lc code=end


def list2node(values):
    head = None
    for v in values[::-1]:
        node = ListNode(v)
        node.next = head
        head = node
    return head


def main():
    s = Solution()
    l1 = list2node([9, 5, 4, 8])
    l2 = list2node([5, 6, 4])
    print(l1, '\n', l2)
    print(s.addTwoNumbers(l1, l2))


if __name__ == "__main__":
    main()
