#
# @lc app=leetcode.cn id=124 lang=python3
#
# [124] 二叉树中的最大路径和
#
from utils import Codec
import unittest
from utils import TreeNode
# @lc code=start
from typing import List, Tuple


class Solution:
    def helper(self, root: TreeNode) -> Tuple[int, int]:
        #
        if root is None:
            return (0, float('-inf'))
        left = self.helper(root.left)
        right = self.helper(root.right)

        #
        from_root = root.val + max(left[0], right[0], 0)
        sub_path = max(
            left[1], right[1],
            root.val + max(0, left[0]) + max(0, right[0])
        )
        return from_root, sub_path

    def maxPathSum(self, root: TreeNode) -> int:
        return max(self.helper(root))

# @lc code=end


class Test(unittest.TestCase):
    def test(self):
        c = Codec()
        s = Solution()

        def f(tree, ans):
            self.assertEqual(
                s.maxPathSum(c.deserialize(tree)), ans
            )

        f('[1,2,3]', 6)
        f('[-10,9,20,null,null,15,7]', 42)
        f('[-1, -2, -3]', -1)
        f('[6,0,-6]', 6)
        f('[-2,6,null,0,-6]', 6)


if __name__ == "__main__":
    unittest.main()
