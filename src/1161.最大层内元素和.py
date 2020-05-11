#
# @lc app=leetcode.cn id=1161 lang=python3
#
# [1161] 最大层内元素和
#

# @lc code=start
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

from queue import Queue


class Solution:
    def maxLevelSum(self, root: TreeNode) -> int:
        q = Queue()
        q.put(root)
        level = 1
        max_level_sum = float('-inf')
        while q.qsize():
            n = q.qsize()
            level_sum = 0
            for _ in range(n):
                node = q.get()
                level_sum += node.val
                if node.left:
                    q.put(node.left)
                if node.right:
                    q.put(node.right)
            if level_sum > max_level_sum:
                max_level_sum = level_sum
                max_level = level
            level += 1
        return max_level

# @lc code=end
