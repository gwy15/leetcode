#
# @lc app=leetcode.cn id=1339 lang=python3
#
# [1339] 分裂二叉树的最大乘积
#
from typing import List
from utils import *
# @lc code=start
MOD = 1_000_000_007


class Solution:
    @staticmethod
    def sumTree(root: TreeNode) -> int:
        if root is None:
            return 0
        return (
            root.val +
            Solution.sumTree(root.left) +
            Solution.sumTree(root.right)
        )

    def maxProduct(self, root: TreeNode) -> int:
        total = self.sumTree(root)
        ans = 0

        def visit(node: TreeNode):
            nonlocal ans
            if node is None:
                return 0
            left_sum = visit(node.left)
            right_sum = visit(node.right)
            tree_sum = (node.val + left_sum + right_sum)
            rest_sum = (total - tree_sum)
            ans = max(ans, (tree_sum * rest_sum))
            #
            return tree_sum

        visit(root)
        return ans % MOD


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().maxProduct(Codec().deserialize(input))
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('[1,2,3,4,5,6]', 110)
    test('[1,null,2,3,4,null,null,5,6]', 90)
    test('[2,3,9,10,7,8,6,5,4,11,1]', 1025)
    test('[1,1]', 1)
