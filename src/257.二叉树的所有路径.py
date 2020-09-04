#
# @lc app=leetcode.cn id=257 lang=python3
#
# [257] 二叉树的所有路径
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def binaryTreePaths(self, root: TreeNode) -> List[str]:
        ans = []

        def helper(root: TreeNode, prefix: List[int]):
            if root is None:
                return
            prefix.append(str(root.val))
            if root.left is None and root.right is None:
                ans.append('->'.join(prefix))
            if root.left:
                helper(root.left, prefix)
            if root.right:
                helper(root.right, prefix)
            prefix.pop()

        helper(root, [])
        return ans
# @lc code=end
