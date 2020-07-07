#
# @lc app=leetcode.cn id=112 lang=python3
#
# [112] 路径总和
#
from utils import TreeNode
# @lc code=start


class Solution:
    def hasPathSum(self, root: TreeNode, sum: int) -> bool:
        if root is None:
            return False
        if root.left is None and root.right is None:
            return root.val == sum
        if root.left and self.hasPathSum(root.left, sum - root.val):
            return True
        if root.right and self.hasPathSum(root.right, sum - root.val):
            return True
        return False


# @lc code=end
if __name__ == "__main__":
    from utils import Codec
    p = Codec().deserialize
    f = Solution().hasPathSum
    assert f(p('[]'), 0) == False
    assert f(p('[]'), 1) == False
    assert f(p('[1]'), 1) == True
