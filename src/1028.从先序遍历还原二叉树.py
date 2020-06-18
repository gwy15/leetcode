#
# @lc app=leetcode.cn id=1028 lang=python3
#
# [1028] 从先序遍历还原二叉树
#
import unittest
from utils import TreeNode, Codec
# @lc code=start
from typing import Generator, Tuple


class Solution:
    def parse_string(self, s: str) -> Generator[Tuple[int, int], None, None]:
        n = len(s)
        i = 0
        #
        while i < n:
            # find depth
            depth = 0
            while i < n and s[i] == '-':
                depth += 1
                i += 1

            # find number
            number = 0
            while i < n and s[i].isdigit():
                number = number * 10 + int(s[i])
                i += 1
            yield (depth, number)

    def recoverFromPreorder(self, S: str) -> TreeNode:
        nodes = self.parse_string(S)
        stack = []

        super_root = TreeNode(-1)
        stack.append((-1, super_root))
        for (depth, val) in nodes:
            # pop until depth == stack[-1].depth + 1
            while depth <= stack[-1][0]:
                stack.pop()
            # insert to left or right
            node = TreeNode(val)
            last_node = stack[-1][1]
            if last_node.left is None:
                last_node.left = node
            else:
                last_node.right = node
            # push to stack
            stack.append((depth, node))
        return super_root.left

# @lc code=end


class Test(unittest.TestCase):
    def test(self):
        def f(s, ans):
            t1 = Solution().recoverFromPreorder(s)
            t2 = Codec().deserialize(ans)
            self.assertEqual(t1, t2)

        f(
            "1-2--3--4-5--6--7",
            '[1,2,5,3,4,6,7]'
        )
        f(
            "1-2--3---4-5--6---7",
            '[1,2,5,3,null,6,null,4,null,7]'
        )
        f(
            "1-401--349---90--88",
            '[1,401,null,349,88,90]'
        )


if __name__ == "__main__":
    unittest.main()
