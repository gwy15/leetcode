#
# @lc app=leetcode.cn id=322 lang=python3
#
# [322] 零钱兑换
#
from prelude import *
# @lc code=start


class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        dp = [-1 for _ in range(amount+1)]
        dp[0] = 0
        for i in range(amount):
            if dp[i] == -1:
                continue
            # i => ...
            for coin in coins:
                if i + coin <= amount:
                    if dp[i+coin] == -1:
                        dp[i+coin] = dp[i] + 1
                    else:
                        dp[i+coin] = min(dp[i]+1, dp[i+coin])

        return dp[amount]
# @lc code=end
