#
# @lc app=leetcode.cn id=105 lang=python3
#
# [105] 从前序与中序遍历序列构造二叉树
#

# Definition for a binary tree node.

from typing import List
import unittest


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None
# @lc code=start


class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> TreeNode:
        num_to_in_index = {num: i for i, num in enumerate(inorder)}
        n = len(preorder)
        return self.buildSubTree(
            preorder, 0,
            inorder, 0,
            n, num_to_in_index)

    def buildSubTree(self, preorder, pre_i, inorder, in_i, n, num_to_index) -> TreeNode:
        if n == 0:
            return None
        elif n == 1:
            return TreeNode(preorder[pre_i])
        # build root
        root_val = preorder[pre_i]
        root = TreeNode(root_val)
        # find children tree
        root_index_inorder = num_to_index[root_val]
        length_of_left = root_index_inorder - in_i

        root.left = self.buildSubTree(
            # left tree in preorder
            preorder, pre_i + 1,
            # left tree in inorder
            inorder, in_i,
            length_of_left, num_to_index)
        root.right = self.buildSubTree(
            # right tree in preorder
            preorder, pre_i + 1 + length_of_left,
            # right, inorder
            inorder, in_i + 1 + length_of_left,
            n - 1 - length_of_left, num_to_index)
        return root

# @lc code=end


class Test(unittest.TestCase):
    def test(self):
        t = Solution().buildTree(
            [3, 9, 20, 15, 7],
            [9, 3, 15, 20, 7]
        )
        import pprint
        pprint.pprint(t)


unittest.main()
