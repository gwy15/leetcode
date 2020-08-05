#
# @lc app=leetcode.cn id=337 lang=python3
#
# [337] 打家劫舍 III
#
from utils import TreeNode
# @lc code=start


class Solution:
    def helper(self, root: TreeNode) -> (int, int):
        """返回 (root打劫，root不打劫)的最高金额"""
        if root is None:
            return (0, 0)
        left = self.helper(root.left)
        right = self.helper(root.right)
        #
        return (
            root.val + left[1] + right[1],
            max(left) + max(right)
        )

    def rob(self, root: TreeNode) -> int:
        return max(self.helper(root))


# @lc code=end
if __name__ == "__main__":
    from utils import Codec
    f = Solution().rob
    de = Codec().deserialize
    assert f(de('[3,2,3,null,3,null,1]')) == 7
    assert f(de('[3,4,5,1,3,null,1]')) == 9
