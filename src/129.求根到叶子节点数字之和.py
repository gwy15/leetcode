#
# @lc app=leetcode.cn id=129 lang=python3
#
# [129] 求根到叶子节点数字之和
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def sumNumbers(self, root: TreeNode) -> int:
        total = 0

        def dfs(node: TreeNode, prefix: int):
            nonlocal total
            if node is None:
                return
            val = prefix * 10 + node.val
            if node.left is None and node.right is None:
                total += val
            if node.left:
                dfs(node.left, val)
            if node.right:
                dfs(node.right, val)

        dfs(root, 0)

        return total


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().sumNumbers(Codec().deserialize(input))
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('[1,2,3]', 25)
    test('[4,9,0,5,1]', 1026)
