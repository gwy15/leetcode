#
# @lc app=leetcode.cn id=98 lang=python3
#
# [98] 验证二叉搜索树
#

# @lc code=start
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:
    def isValidBST(self, root: TreeNode) -> bool:
        if root is None:
            return True
        return self.isValidRangeBST(root.left, None, root.val) and \
            self.isValidRangeBST(root.right, root.val, None)

    def isValidRangeBST(self, root: TreeNode, left: int, right: int) -> bool:
        if root is None:
            return True
        if (left is None or left < root.val) and (right is None or root.val < right):
            return self.isValidRangeBST(root.left, left, root.val) and \
                self.isValidRangeBST(root.right, root.val, right)
        return False

# @lc code=end
