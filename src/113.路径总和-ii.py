#
# @lc app=leetcode.cn id=113 lang=python3
#
# [113] 路径总和 II
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def search_path(self, root: TreeNode, target: int, prefix: List[int], ans: List[List[int]]):
        if root is None:
            return

        if root.left is None and root.right is None:
            if target == root.val:
                prefix.append(root.val)
                ans.append(prefix[:])
                prefix.pop()
            return

        prefix.append(root.val)
        if root.left:
            self.search_path(root.left, target - root.val, prefix, ans)
        if root.right:
            self.search_path(root.right, target - root.val, prefix, ans)
        prefix.pop()
        return

    def pathSum(self, root: TreeNode, sum: int) -> List[List[int]]:
        ans = []
        self.search_path(root, sum, [], ans)
        return ans

# @lc code=end


if __name__ == '__main__':
    def test(tree, sum, expected):
        calc = Solution().pathSum(Codec().deserialize(tree), sum)
        if calc != expected:
            print(f'case failed: `{tree, sum}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        '[5,4,8,11,null,13,4,7,2,null,null,5,1]',
        22,
        [
            [5, 4, 11, 2],
            [5, 8, 4, 5]
        ]
    )
    test(
        '[]', 0,
        []
    )
