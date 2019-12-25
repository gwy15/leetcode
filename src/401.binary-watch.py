#
# @lc app=leetcode id=401 lang=python3
#
# [401] Binary Watch
#
from typing import List
# @lc code=start
import itertools


class Solution:
    @staticmethod
    def getPosibleHours(hrCount):
        if hrCount == 0:
            return [0]
        return (hr for hr in (
            sum(lights) for lights in
            itertools.combinations((8, 4, 2, 1), hrCount)
        ) if 0 <= hr <= 11)

    @staticmethod
    def getPosibleMinutes(minCount):
        if minCount == 0:
            return [0]
        return (m for m in (
            sum(lights) for lights in
            itertools.combinations((32, 16, 8, 4, 2, 1), minCount)
        ) if 0 <= m <= 59)

    def readBinaryWatch(self, num: int) -> List[str]:
        ans = []
        for hrCount in range(0, 1+min(4, num)):  # 0, ... 4 or n
            minCount = num - hrCount
            if minCount > 6:
                continue
            # legal now
            ans += [
                f'{hour:d}:{minute:02d}'
                for hour in self.getPosibleHours(hrCount)
                for minute in self.getPosibleMinutes(minCount)
            ]
        return ans

# @lc code=end


print(Solution().readBinaryWatch(5))
