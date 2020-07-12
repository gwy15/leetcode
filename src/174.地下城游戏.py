#
# @lc app=leetcode.cn id=174 lang=python3
#
# [174] 地下城游戏
#
from typing import List
# @lc code=start


class Solution:
    def calculateMinimumHP(self, dungeon: List[List[int]]) -> int:
        m, n = len(dungeon), len(dungeon[0])
        INF = 1 << 30
        # dp: 到达本格之后、未结算时需要的最小初始血量
        #     i.e.，从上一个格子出发时的血量
        last_dp = [INF] * n
        last_dp[n-1] = 1
        for i in range(m-1, -1, -1):
            dp = [INF] * n
            for j in range(n-1, -1, -1):
                #
                leave_val = last_dp[j]
                if j+1 < n and dp[j+1] < leave_val:
                    leave_val = dp[j+1]

                arrive_val = max(1, leave_val - dungeon[i][j])
                dp[j] = arrive_val

            last_dp = dp

        return last_dp[0]

# @lc code=end


if __name__ == "__main__":
    f = Solution().calculateMinimumHP
    assert f(
        [
            [-2, -3, 3],
            [-5, -10, 1],
            [10, 30, -5]
        ]
    ) == 7

    assert f(
        [[1, 1, 1],
         [1, 1, 1]]
    ) == 1
    assert f(
        [[-1, -1],
         [-1, -1]]
    ) == 4
