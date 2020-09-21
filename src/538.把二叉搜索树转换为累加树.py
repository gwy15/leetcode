#
# @lc app=leetcode.cn id=538 lang=python3
#
# [538] 把二叉搜索树转换为累加树
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def convertBST(self, root: TreeNode) -> TreeNode:
        def dfs(node, cnt: int) -> int:
            if node is None:
                return 0
            if node.right:
                cnt = dfs(node.right, cnt)

            #
            cnt += node.val
            node.val = cnt

            if node.left:
                cnt = dfs(node.left, cnt)
            return cnt

        dfs(root, 0)
        return root


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().convertBST(Codec().deserialize(input))
        expected = Codec().deserialize(expected)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)

    test(
        '[5,2,13]', '[18, 20, 13]'
    )

    test(
        '[4,2,6,1,3,5,7]', '[22,27,13,28,25,18,7]'
    )
    test('[]', '[]')
