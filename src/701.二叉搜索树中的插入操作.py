#
# @lc app=leetcode.cn id=701 lang=python3
#
# [701] 二叉搜索树中的插入操作
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def insertIntoBST(self, root: TreeNode, val: int) -> TreeNode:
        hyper = TreeNode(-1)
        hyper.right = root

        parent = hyper
        node = root

        while node:
            if node.val < val:
                parent = node
                node = node.right
            else:
                parent = node
                node = node.left
        #
        if parent.val < val:
            parent.right = TreeNode(val)
        else:
            parent.left = TreeNode(val)

        return hyper.right


# @lc code=end
if __name__ == '__main__':
    def test(root, val, expected):
        root = Codec().deserialize(root)
        calc = Solution().insertIntoBST(root, val)

        if Codec().serialize(calc) != expected:
            print(f'case failed: `{root, val}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        '[4,2,7,1,3]',
        5,
        '[4,2,7,1,3,5]'
    )
    test(
        '[]',
        1,
        '[1]'
    )
    test(
        '[1]',
        2,
        '[1,null,2]'
    )
