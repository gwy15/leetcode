#
# @lc app=leetcode.cn id=543 lang=python3
#
# [543] 二叉树的直径
#


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None
# @lc code=start
# Definition for a binary tree node.


class Solution:
    def diameterOfBinaryTree(self, root: TreeNode) -> int:
        return Solution.depthAndDiameterOf(root)[1]

    @staticmethod
    def depthAndDiameterOf(root: TreeNode) -> (int, int):
        if root is None:
            return (0, 0)
        depth_left, diameter_left = Solution.depthAndDiameterOf(root.left)
        depth_right, diameter_right = Solution.depthAndDiameterOf(root.right)

        depth = 1 + max(depth_left, depth_right)
        diameter = max(
            diameter_left, diameter_right,
            depth_left + depth_right
        )
        return (depth, diameter)
# @lc code=end
