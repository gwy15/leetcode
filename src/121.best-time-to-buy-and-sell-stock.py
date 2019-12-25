#
# @lc app=leetcode id=121 lang=python3
#
# [121] Best Time to Buy and Sell Stock
#
from typing import List

class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        if not prices:
            return 0
        minPrices = [prices[0]] * len(prices)
        maxProfits = [0] * len(prices)
        for i in range(1, len(prices)):
            minPrices[i] = min(minPrices[i-1], prices[i])
            maxProfits[i] = max(maxProfits[i-1], prices[i]-minPrices[i])
        return maxProfits[-1]
