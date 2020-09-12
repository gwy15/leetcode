#
# @lc app=leetcode.cn id=637 lang=python3
#
# [637] 二叉树的层平均值
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def averageOfLevels(self, root: TreeNode) -> List[float]:
        cnt = []
        avg = []

        def dfs(node: TreeNode, depth: int):
            if node is None:
                return
            if depth >= len(cnt):
                cnt.append(0)
                avg.append(0)
            cnt[depth] += 1
            avg[depth] += node.val

            dfs(node.left, depth + 1)
            dfs(node.right, depth + 1)

        dfs(root, 0)
        return [a / b for a, b in zip(avg, cnt)]


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().averageOfLevels(Codec().deserialize(input))
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('[3, 9, 20, null, null, 15, 7]', [3, 14.5, 11])
