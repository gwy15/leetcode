#
# @lc app=leetcode id=1123 lang=python3
#
# [1123] Lowest Common Ancestor of Deepest Leaves
#

# @lc code=start
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:
    def lcaDeepestLeaves(self, root: TreeNode) -> TreeNode:
        def func(root, depth):
            'Return node, depth'
            if root.left == root.right == None:
                return root, depth  # self
            if root.left and root.right is None:
                return func(root.left, depth+1)
            if root.left is None and root.right:
                return func(root.right, depth+1)
            # both root.left and root.right
            leftRoot, leftDepth = func(root.left, depth+1)
            rightRoot, rightDepth = func(root.right, depth+1)
            if leftDepth == rightDepth:
                return root, leftDepth
            elif leftDepth > rightDepth:
                return leftRoot, leftDepth
            return rightRoot, rightDepth
        return func(root, 0)[0]


# @lc code=end
