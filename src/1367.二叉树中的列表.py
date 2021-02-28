#
# @lc app=leetcode.cn id=1367 lang=python3
#
# [1367] 二叉树中的列表
#

# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def dfs(self, head: ListNode, root: TreeNode) -> bool:
        if head is None:
            return True
        # head != None
        if root is None:
            return False
        # root != None
        if root.val == head.val:
            return self.dfs(head.next, root.left) or self.dfs(head.next, root.right)

    def isSubPath(self, head: ListNode, root: TreeNode) -> bool:
        if head is None:
            return True
        if root is None:
            return False
        if self.dfs(head, root):
            return True
        return self.isSubPath(head, root.left) or self.isSubPath(head, root.right)
            
# @lc code=end
