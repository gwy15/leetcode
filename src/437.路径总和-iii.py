#
# @lc app=leetcode.cn id=437 lang=python3
#
# [437] 路径总和 III
#
from prelude import *


# @lc code=start
# Definition for a binary tree node.


class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> int:
        self.ans = 0
        prefix = {}
        prefix[0] = 1
        self.dfs(root, cur_sum=0, target=sum, prefix=prefix)
        return self.ans

    def dfs(self, ptr: TreeNode, target: int, cur_sum: int, prefix: dict[int, int]):
        if ptr == None:
            return
        cur_sum = cur_sum + ptr.val
        self.ans += prefix.get(cur_sum - target, 0)
        prefix[cur_sum] = prefix.get(cur_sum, 0) + 1
        self.dfs(ptr.left, target, cur_sum, prefix)
        self.dfs(ptr.right, target, cur_sum, prefix)
        prefix[cur_sum] -= 1

# @lc code=end


if __name__ == '__main__':
    def test(root, sum, expected):
        root = Codec().deserialize(root)
        calc = Solution().pathSum(root, sum)
        if calc != expected:
            print(f'case failed: `{root, sum}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('[10,5,-3,3,2,null,11,3,-2,null,1]', sum=8, expected=3)
