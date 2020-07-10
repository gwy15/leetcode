#
# @lc app=leetcode.cn id=309 lang=python3
#
# [309] 最佳买卖股票时机含冷冻期
#
from typing import List
# @lc code=start


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        n = len(prices)
        holder = [0] * (n+1)
        holder[0] = - (1 << 30)
        none = [0] * (n+1)
        freeze = [0] * (n+1)
        freeze[0] = - (1 << 30)

        for i in range(1, n+1):
            #
            holder[i] = max(holder[i-1], none[i-1] - prices[i-1])
            none[i] = max(none[i-1], freeze[i-1])
            freeze[i] = holder[i-1] + prices[i-1]

        return max(none[n], freeze[n], holder[n])


# @lc code=end
if __name__ == "__main__":
    assert Solution().maxProfit([1, 2, 3, 0, 2]) == 3
    assert Solution().maxProfit([5, 4, 3, 2, 1]) == 0
    assert Solution().maxProfit([1]) == 0
    assert Solution().maxProfit([]) == 0
    assert Solution().maxProfit([1, 2, 3, 4, 100]) == 99
