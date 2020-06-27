#
# @lc app=leetcode.cn id=1008 lang=python3
#
# [1008] 先序遍历构造二叉树
#

from utils import Codec
import unittest
from utils import TreeNode
from typing import List
# @lc code=start


class StackFrame:
    def __init__(self, node: TreeNode, upper_bound: int):
        self.node = node
        self.upper_bound = upper_bound


class Solution:
    def bstFromPreorder(self, nodes: List[int]) -> TreeNode:
        stack = []
        stack.append(
            StackFrame(TreeNode(1_000_000), 1_000_000)
        )
        for val in nodes:
            # pop until val < upper_bound
            while True:
                stack_frame = stack[-1]
                if val > stack_frame.upper_bound:
                    stack.pop()
                    continue
                break
            # now val < upper_bound
            stack_frame = stack[-1]
            root, upper_bound = stack_frame.node, stack_frame.upper_bound
            # if val < root.left, keep going left
            if val < root.val:
                left = TreeNode(val)
                root.left = left
                stack.append(
                    StackFrame(
                        left, root.val
                    )
                )
            else:
                right = TreeNode(val)
                root.right = right
                stack.append(
                    StackFrame(
                        right, upper_bound
                    )
                )

        return stack[0].node.left


# @lc code=end


class Test(unittest.TestCase):
    def test(self):
        c = Codec().deserialize
        p = Solution().bstFromPreorder
        self.assertEqual(
            p([8, 5, 1, 7, 10, 12]),
            c('[8, 5, 10, 1, 7, null, 12]')
        )
        self.assertEqual(
            p([1]),
            c('[1]')
        )
        self.assertEqual(
            p([]),
            c('[]')
        )


if __name__ == "__main__":
    unittest.main()
