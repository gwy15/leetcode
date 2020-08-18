#
# @lc app=leetcode.cn id=109 lang=python3
#
# [109] 有序链表转换二叉搜索树
#

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
# Definition for a binary tree node.


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
# @lc code=start


class Solution:
    @staticmethod
    def list_node_len(head: ListNode) -> int:
        n = 0
        while head:
            head = head.next
            n += 1
        return n

    @staticmethod
    def to_tree(ptr: ListNode, n: int) -> (ListNode, TreeNode):
        if n == 0:
            return ptr, None
        left_len = n // 2
        right_len = n - 1 - left_len

        ptr, left = Solution.to_tree(ptr, left_len)
        ptr, root = ptr.next, ListNode(ptr.val)
        ptr, right = Solution.to_tree(ptr, right_len)

        root.left = left
        root.right = right
        return ptr, root

    def sortedListToBST(self, head: ListNode) -> TreeNode:
        n = self.list_node_len(head)
        return self.to_tree(head, n)[1]

# @lc code=end
