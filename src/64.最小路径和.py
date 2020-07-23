#
# @lc app=leetcode.cn id=64 lang=python3
#
# [64] 最小路径和
#
from typing import List
# @lc code=start


class Solution:
    def minPathSum(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        last = [float('inf')] * n
        last[0] = 0
        for i in range(m):
            dp = [0] * n
            for j in range(n):
                dp[j] = last[j]
                if j > 0 and dp[j-1] < dp[j]:
                    dp[j] = dp[j-1]
                dp[j] += grid[i][j]
            last = dp
        return last[n-1]


# @lc code=end
if __name__ == "__main__":
    f = Solution().minPathSum
    assert f([
        [1, 3, 1],
        [1, 5, 1],
        [4, 2, 1]
    ]) == 7
