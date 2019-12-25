#
# @lc app=leetcode id=123 lang=python3
#
# [123] Best Time to Buy and Sell Stock III
#
class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        if not prices:
            return 0
        minPricesFromStart = [prices[0]] * len(prices)
        firstProfit = [0] * len(prices)
        for i in range(1, len(prices)):
            minPricesFromStart[i] = min(minPricesFromStart[i-1], prices[i])
            firstProfit[i] = max(firstProfit[i-1], prices[i]-minPricesFromStart[i])

        maxPricesFromEnd = [prices[-1]] * len(prices)
        secondProfit = [0] * len(prices)
        for i in range(len(prices)-2, -1, -1):
            maxPricesFromEnd[i] = max(maxPricesFromEnd[i+1], prices[i])
            secondProfit[i] = max(secondProfit[i+1], maxPricesFromEnd[i] - prices[i])
        
        return max(
            firstProfit[i] + secondProfit[i]
            for i in range(len(prices))
        )

