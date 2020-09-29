#
# @lc app=leetcode.cn id=145 lang=python3
#
# [145] 二叉树的后序遍历
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def postorderTraversal(self, root: TreeNode) -> List[int]:
        ans = []

        stack = []
        if root:
            stack.append((root, False))

        while stack:
            node, is_visit = stack.pop()
            if is_visit:
                ans.append(node.val)
                continue

            stack.append((node, True))  # visit last
            if node.right:
                stack.append((node.right, False))
            if node.left:
                stack.append((node.left, False))

        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        tree = Codec().deserialize(input)
        calc = Solution().postorderTraversal(tree)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        '[1,null,2,3]',
        [3, 2, 1]
    )
    test(
        '[]',
        []
    )
