#
# @lc app=leetcode.cn id=834 lang=python3
#
# [834] 树中距离之和
#
from typing import List
from utils import *
# @lc code=start


class Solution:

    def sumOfDistancesInTree(self, N: int, edges: List[List[int]]) -> List[int]:
        ans = []
        size = [0 for _ in range(N)]
        ans = [-1 for _ in range(N)]
        # next graph
        next = [[] for _ in range(N)]
        for a, b in edges:
            next[a].append(b)
            next[b].append(a)

        #
        def dfs_for_dp_and_size(i) -> int:
            """Return dp
            """
            # mark as visiting
            size[i] = -1
            _size = 1
            dp = 0
            for n in next[i]:
                if size[n] == 0:  # unvisited
                    dp += dfs_for_dp_and_size(n)
                    dp += size[n]
                    _size += size[n]
            size[i] = _size
            return dp

        # compute ans
        ans[0] = dfs_for_dp_and_size(0)
        total_size = size[0]

        def dfs_for_ans(i):
            for n in next[i]:
                if ans[n] == -1:
                    # set ans for n
                    ans[n] = ans[i] + total_size - 2 * size[n]
                    dfs_for_ans(n)
        dfs_for_ans(0)

        return ans


# @lc code=end
if __name__ == '__main__':
    def test(N, edges, expected):
        calc = Solution().sumOfDistancesInTree(N, edges)
        if calc != expected:
            print(f'case failed: `{N, edges}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        6,
        [[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]],
        [8, 12, 6, 10, 10, 10]
    )
