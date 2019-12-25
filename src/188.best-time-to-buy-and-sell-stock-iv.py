#
# @lc app=leetcode id=188 lang=python3
#
# [188] Best Time to Buy and Sell Stock IV
#
from typing import List
import numpy as np

def getInitDP(shape):
    dp = np.zeros(shape, dtype=np.int32)
    dp[0, 1] = -0xFFFFFFF
    return dp

class Solution:
    def maxProfit(self, k: int, prices: List[int]) -> int:
        k = min(k, len(prices) // 2 + 1)
        
        shape = (len(prices+1, 2))

        # shape = (1+k, len(prices)+1, 2)
        # dp = np.zeros(shape, np.int32) # [trans, day, holding]
        # dp[0, :, 1] = -0xFFFFFFF
        # dp[:, 0, 1] = -0xFFFFFFF
        lastTrans = getInitDP()
        dp = np.get
        for trans in range(1, k+1):  # trans in 1 ... k
            for today in range(1, len(prices)+1):  # today in 1...days
                dp[trans, today, 1] = max(
                    dp[trans, today-1, 1], # no trans
                    dp[trans-1, today-1, 0] - prices[today-1] # or buy
                )
                dp[trans, today, 0] = max(
                    dp[trans, today-1, 0], # no trans
                    dp[trans, today-1, 1] + prices[today-1]
                )
        return dp[k, len(prices), 0]


# print(Solution().maxProfit(0, [1,3]))
# print(Solution().maxProfit(2, [2, 4, 1]))
# print(Solution().maxProfit(2, [3, 3, 5, 0, 0, 3, 1, 4]))
