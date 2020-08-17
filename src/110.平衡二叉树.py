#
# @lc app=leetcode.cn id=110 lang=python3
#
# [110] 平衡二叉树
#
from utils import TreeNode
# @lc code=start


class Solution:
    def isBalanced(self, root: TreeNode) -> bool:
        def helper(node) -> (bool, int):
            if node is None:
                return (True, 0)
            left_ok, left_height = helper(node.left)
            if not left_ok:
                return False, 0
            right_ok, right_height = helper(node.right)
            if not right_ok:
                return False, 0
            if abs(left_height - right_height) > 1:
                return False, 0
            return True, 1 + max(left_height, right_height)

        return helper(root)[0]


# @lc code=end
if __name__ == "__main__":
    from utils import Codec
    de = Codec().deserialize
    f = Solution().isBalanced
    assert f(de('[3,9,20,null,null,15,7]'))
    assert not f(de('[1,2,2,3,3,null,null,4,4]'))
