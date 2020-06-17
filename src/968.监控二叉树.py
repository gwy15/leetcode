#
# @lc app=leetcode.cn id=968 lang=python3
#
# [968] 监控二叉树
#
import unittest
from utils import TreeNode, Codec
# @lc code=start

from typing import List, Tuple


class Solution_DP:
    @staticmethod
    def solve(root: TreeNode) -> Tuple[int, int, int]:
        """Solve the minium cameras needed for covering this tree

        Args:
            root (TreeNode): [description]

        Returns:
            Tuple[int, int, int]: (root not covered, root covered, root put a camera)
        """
        if root is None:
            #
            return (0, 0, float('inf'))
        left = Solution_DP.solve(root.left)
        right = Solution_DP.solve(root.right)
        #
        not_covered = left[1] + right[1]
        covered = min(
            left[2] + right[2],
            left[2] + right[1],
            left[1] + right[2]
        )
        put = min(left) + min(right) + 1
        return (not_covered, covered, put)

    def minCameraCover(self, root: TreeNode) -> int:
        return min(Solution_DP.solve(root)[1:])


class Solution_DFS:
    def minCameraCover(self, root: TreeNode) -> int:
        from enum import IntEnum

        class Status(IntEnum):
            UNCOVERED = 0
            COVERED = 1
            PUT = 2

        ans = 0

        def dfs(node: TreeNode) -> Status:
            nonlocal ans
            if node is None:
                return Status.COVERED
            left = dfs(node.left)
            right = dfs(node.right)
            if left == Status.UNCOVERED or right == Status.UNCOVERED:
                # cover
                ans += 1
                return Status.PUT
            if left == Status.PUT or right == Status.PUT:
                return Status.COVERED
            else:
                # both covered
                return Status.UNCOVERED
        status = dfs(root)
        if status == Status.UNCOVERED:
            ans += 1
        return ans


Solution = Solution_DFS
# @lc code=end


class Test(unittest.TestCase):
    def test(self):
        t = Codec().deserialize("[0, 0, null, 0, 0]")
        self.assertEqual(Solution().minCameraCover(t), 1)

        t = Codec().deserialize('[0,0,null,0,null,0,null,null,0]')
        self.assertEqual(Solution().minCameraCover(t), 2)


if __name__ == "__main__":
    unittest.main()
