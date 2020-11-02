from prelude import *

from typing import Tuple


class Solution:
    def lowestCommonAncestor(self, root: TreeNode, p: TreeNode, q: TreeNode) -> TreeNode:
        def helper(node: TreeNode) -> Tuple[TreeNode, int]:
            """Returns:
                Tuple[TreeNode, int]: 如果找到解，返回解，否则返回 status:
                status = 0 没找到任何一个
                status = 1 找到一个
                status = 2 找到两个
            """
            if node is None:
                return (None, 0)
            # node 找到
            if node.val == p.val or node.val == q.val:
                left_result = helper(node.left)
                if left_result[1]:
                    return [node, 2]
                right_result = helper(node.right)
                if right_result[1]:
                    return [node, 2]
                return [None, 1]

            left_result = helper(node.left)
            if left_result[1] == 2:
                return left_result
            #
            right_result = helper(node.right)
            if right_result[1] == 2:
                return right_result

            if left_result[1] and right_result[1]:
                return [node, 2]
            return [None, left_result[1] + right_result[1]]

        return helper(root)[0]


if __name__ == '__main__':
    def test(input, p, q, expected):
        calc = Solution().lowestCommonAncestor(
            Codec().deserialize(input),
            TreeNode(p), TreeNode(q)
        )
        if calc is None or calc.val != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('[3,5,1,6,2,0,8,null,null,7,4]', 5, 1, 3)
