#
# @lc app=leetcode.cn id=404 lang=python3
#
# [404] 左叶子之和
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def sumOfLeftLeaves(self, root: TreeNode) -> int:
        cnt = 0
        if root is None:
            return cnt
        if root.left:
            left = root.left
            if left.left is None and left.right is None:
                cnt += left.val
            else:
                cnt += self.sumOfLeftLeaves(left)

        if root.right:
            cnt += self.sumOfLeftLeaves(root.right)

        return cnt

# @lc code=end
