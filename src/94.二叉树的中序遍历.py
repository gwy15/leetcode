#
# @lc app=leetcode.cn id=94 lang=python3
#
# [94] 二叉树的中序遍历
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def inorderTraversal(self, root: TreeNode) -> List[int]:
        ret = []
        stack = []
        while root is not None:
            # prepare for inorder traversal
            stack.append(root)
            # go left
            root = root.left
            # reaches end
            while root is None:
                if not stack:
                    break
                # inorder
                root = stack.pop()
                ret.append(root.val)
                # right tree
                root = root.right
        return ret


# @lc code=end
if __name__ == "__main__":
    def test(a, b):
        l = Solution().inorderTraversal(Codec().deserialize(a))
        assert l == b
    test("[1, null, 2, 3]", [1, 3, 2])
    test("[1]", [1])
    test("[1,2,3,4]", [4, 2, 1, 3])
    test("[]", [])
