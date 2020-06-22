#
# @lc app=leetcode.cn id=563 lang=python3
#
# [563] 二叉树的坡度
#
from utils import TreeNode
# @lc code=start


class Solution:
    def helper(self, root: TreeNode) -> (int, int, int):
        """return (sum, tilt, sum of tilt)"""
        if root is None:
            return 0, 0, 0
        left_sum, left_tilt, left_sum_tilt = self.helper(root.left)
        right_sum, right_tilt, right_sum_tilt = self.helper(root.right)

        _sum = root.val + left_sum + right_sum
        tilt = abs(left_sum - right_sum)
        sum_tilt = tilt + left_sum_tilt + right_sum_tilt
        return _sum, tilt, sum_tilt

    def findTilt(self, root: TreeNode) -> int:
        return self.helper(root)[2]

# @lc code=end
