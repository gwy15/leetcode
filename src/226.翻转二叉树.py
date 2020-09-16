#
# @lc app=leetcode.cn id=226 lang=python3
#
# [226] 翻转二叉树
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def invertTree(self, root: TreeNode) -> TreeNode:
        if root is None:
            return root
        left, right = root.left, root.right
        root.left = self.invertTree(right)
        root.right = self.invertTree(left)
        return root
# @lc code=end
