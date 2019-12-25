#
# @lc app=leetcode id=983 lang=python3
#
# [983] Minimum Cost For Tickets
#
from typing import List

# @lc code=start


class Solution:
    def mincostTickets(self, days: List[int], costs: List[int]) -> int:
        # day: start from 0
        dp = [float('inf')] * len(days)
        dp.append(0)  # workaround for day -1

        def findFarestDayWithIn(index, gap):
            # find farest day within a week
            today = days[index]
            if today <= gap: # e.g., 1,2,3...gap
                return -1  # start from the very beginning

            farestIndex = index - 1
            while True:
                index -= 1 # from last index
                if index >= 0 and today - days[index] < gap:
                    farestIndex = index - 1
                else:
                    break
            return farestIndex

        for index, day in enumerate(days):
            day -= 1  # start from 0
            # buy a single day ticket
            dp[index] = min(dp[index], dp[index - 1] + costs[0])
            # or a week?
            dp[index] = min(dp[index], dp[findFarestDayWithIn(index, 7)] + costs[1])
            # or a month?
            dp[index] = min(dp[index], dp[findFarestDayWithIn(index, 30)] + costs[2])

        return dp[len(days) - 1]

# @lc code=end

print(Solution().mincostTickets(
    # [1,4,6,7,8,20], [2,7,15]
    # [1,2,3,4,5,6,7,8,9,10,30,31], [2,7,15]
    # [1,4,6,7,8,20], [7,2,15]
    [1,5,8,9,10,12,13,16,17,18,19,20,23,24,29], [3,12,54]
))
