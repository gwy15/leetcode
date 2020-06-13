#
# @lc app=leetcode.cn id=95 lang=python3
#
# [95] 不同的二叉搜索树 II
#
from typing import List


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

# @lc code=start


class Solution:
    def generate(self, a, b) -> List[TreeNode]:
        if a == b:
            return [None]
        if a + 1 == b:
            return [TreeNode(a)]
        ans = []
        for root in range(a, b):
            for left in self.generate(a, root):
                for right in self.generate(root+1, b):
                    t = TreeNode(root)
                    t.left = left
                    t.right = right
                    ans.append(t)
        return ans

    def generateTrees(self, n: int) -> List[TreeNode]:
        if n == 0:
            return []
        return self.generate(1, n+1)
# @lc code=end


if __name__ == "__main__":
    s = Solution().generateTrees(3)
    assert len(s) == 5
