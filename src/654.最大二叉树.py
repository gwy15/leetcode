#
# @lc app=leetcode.cn id=654 lang=python3
#
# [654] 最大二叉树
#

class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

# @lc code=start
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

def merge(tree, n):
    if n > tree.val:
        new = TreeNode(n)
        new.left = tree
        return new
    # n < tree.val
    right = tree.right
    if right is None:
        tree.right = TreeNode(n)
        return tree
    tree.right = merge(tree.right, n)
    return tree


class Solution:
    def constructMaximumBinaryTree(self, nums: List[int]) -> TreeNode:
        tree = TreeNode(nums[0])
        for i in range(1, len(nums)):
            tree = merge(tree, nums[i])
        return tree
# @lc code=end

