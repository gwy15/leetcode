#
# @lc app=leetcode.cn id=329 lang=python3
#
# [329] 矩阵中的最长递增路径
#
from typing import List
# @lc code=start


class Solution:
    def longestIncreasingPath(self, matrix: List[List[int]]) -> int:
        if len(matrix) == 0:
            return 0
        m, n = len(matrix), len(matrix[0])
        dist = [[0 for _ in range(n)] for _ in range(m)]
        #

        def dfs(i, j):
            if dist[i][j] != 0:
                return dist[i][j]

            dist[i][j] = 1

            def check(cond, ii, jj):
                if cond and matrix[ii][jj] < matrix[i][j]:
                    dist[i][j] = max(dist[i][j], 1 + dfs(ii, jj))

            check(i > 0, i-1, j)
            check(j > 0, i, j-1)
            check(i+1 < m, i+1, j)
            check(j+1 < n, i, j+1)

            return dist[i][j]

        ans = 0
        for i in range(m):
            for j in range(n):
                ans = max(ans, dfs(i, j))

        return ans


# @lc code=end
if __name__ == "__main__":
    f = Solution().longestIncreasingPath
    assert f([
        [9, 9, 4],
        [6, 6, 8],
        [2, 1, 1]
    ]) == 4
    assert f(
        [
            [3, 4, 5],
            [3, 2, 6],
            [2, 2, 1]
        ]
    ) == 4
    assert f([]) == 0
