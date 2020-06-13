#
# @lc app=leetcode.cn id=894 lang=python3
#
# [894] 所有可能的满二叉树
#

from typing import List, Dict
import unittest


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


# @lc code=start
trees = {}
trees[0] = []
trees[1] = [TreeNode(0)]


class Solution:
    def allPossibleFBT(self, N: int) -> List[TreeNode]:
        if N in trees:
            return trees[N]
        if N % 2 == 0:
            return []
        ans = []
        #
        # left = 1, ..=N-1-1
        for left_size in range(1, N-1, 2):
            left_tree = self.allPossibleFBT(left_size)
            right_tree = self.allPossibleFBT(N-1-left_size)
            for left in left_tree:
                for right in right_tree:
                    root = TreeNode(0)
                    root.left = left
                    root.right = right
                    ans.append(root)

        trees[N] = ans
        return ans


# @lc code=end


class Test(unittest.TestCase):
    def test(self):
        Solution().allPossibleFBT(20)


if __name__ == "__main__":
    unittest.main()
