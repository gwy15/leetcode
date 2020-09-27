#
# @lc app=leetcode.cn id=235 lang=python3
#
# [235] 二叉搜索树的最近公共祖先
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        l, r = p.val, q.val
        if l > r:
            return self.lowestCommonAncestor(root, q, p)

        inf = float('inf')
        left, right = -inf, inf
        node = root
        while True:
            if l <= node.val <= r:
                return node
            if node.val < l:
                node = node.right
            elif r < node.val:
                node = node.left


# @lc code=end
if __name__ == '__main__':
    def test(root, l, r, expected):
        root = Codec().deserialize(root)
        calc = Solution().lowestCommonAncestor(root, TreeNode(l), TreeNode(r))
        if calc.val != expected:
            print(f'case failed: `{root, l, r}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        '[6,2,8,0,4,7,9,null,null,3,5]',
        2, 8, 6
    )
    test(
        '[6,2,8,0,4,7,9,null,null,3,5]',
        2, 4, 2
    )
