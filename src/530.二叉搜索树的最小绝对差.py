#
# @lc app=leetcode.cn id=530 lang=python3
#
# [530] 二叉搜索树的最小绝对差
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def visit(self, root):
        if root:
            for n in self.visit(root.left):
                yield n
            yield root.val
            for n in self.visit(root.right):
                yield n

    def getMinimumDifference(self, root: TreeNode) -> int:
        gen = self.visit(root)
        last = next(gen)
        ans = float('inf')
        while True:
            try:
                this = next(gen)
                ans = min(ans, this - last)
                last = this
            except StopIteration:
                break

        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().getMinimumDifference(Codec().deserialize(input))
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        '[1,null,3,2]',
        1
    )
    test(
        '[543,384,652,null,445,null,699]',
        47
    )
