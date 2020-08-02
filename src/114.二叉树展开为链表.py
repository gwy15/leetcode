#
# @lc app=leetcode.cn id=114 lang=python3
#
# [114] 二叉树展开为链表
#
from utils import TreeNode
# @lc code=start


class Solution:
    def _helper(self, root: TreeNode) -> TreeNode:
        # root is not None
        left, right = root.left, root.right
        if left is None:
            if right is not None:
                return self._helper(right)
            # both none
            return root

        # left is not None
        if right is None:
            left_last = self._helper(left)
            root.left = None
            root.right = left
            return left_last

        left_last = self._helper(root.left)
        right_last = self._helper(root.right)

        root.left = None
        root.right = left
        left_last.right = right
        return right_last

    def flatten(self, root: TreeNode) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        if root is None:
            return
        self._helper(root)


# @lc code=end
if __name__ == "__main__":
    from utils import Codec
    de = Codec().deserialize

    def test(t1, tans):
        print(t1, tans)
        t1 = de(t1)
        Solution().flatten(t1)
        tans = de(tans)
        print(t1, tans)
        print()
        assert t1 == tans

    test('[]', '[]')
    test('[1]', '[1]')
    test('[1,2]', '[1, null, 2]')
    test('[1,2,5,3,4,null,6]', '[1,null,2,null,3,null,4,null,5,null,6]')
