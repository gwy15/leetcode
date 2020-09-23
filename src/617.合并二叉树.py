#
# @lc app=leetcode.cn id=617 lang=python3
#
# [617] 合并二叉树
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def mergeTrees(self, t1: TreeNode, t2: TreeNode) -> TreeNode:
        if t1 is None or t2 is None:
            return t1 or t2
        t1.val += t2.val
        t1.left = self.mergeTrees(t1.left, t2.left)
        t1.right = self.mergeTrees(t1.right, t2.right)
        return t1

# @lc code=end


if __name__ == '__main__':
    def test(t1, t2, expected):
        t1, t2 = Codec().deserialize(t1), Codec().deserialize(t2)
        expected = Codec().deserialize(expected)
        calc = Solution().mergeTrees(t1, t2)
        if calc != expected:
            print(f'case failed: `{t1, t2}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('[1,3,2,5]', '[2,1,3,null,4,null,7]', '[3,4,5,5,4,null,7]')
