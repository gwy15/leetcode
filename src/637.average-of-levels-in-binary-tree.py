#
# @lc app=leetcode id=637 lang=python3
#
# [637] Average of Levels in Binary Tree
#

# @lc code=start
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

from collections import defaultdict
class Solution:
    def averageOfLevels(self, root: TreeNode) -> List[float]:
        self.NOE = defaultdict(int) # number of elements
        self.sum = defaultdict(int) # sum of values
        self.visit(root, 0)
        return [
            self.sum[i] / self.NOE[i]
            for i in range(len(self.NOE))
        ]
    
    def visit(self, root, level):
        self.NOE[level] += 1
        self.sum[level] += root.val
        if root.left:
            self.visit(root.left, level+1)
        if root.right:
            self.visit(root.right, level+1)
           
# @lc code=end

