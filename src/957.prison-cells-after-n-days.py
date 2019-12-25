#
# @lc app=leetcode id=957 lang=python3
#
# [957] Prison Cells After N Days
#
from typing import List
# @lc code=start
class Solution:
    def prisonNextDay(self, cells: List[int]) -> List[int]:
        nextDay = [0] * len(cells)
        for i in range(1, len(cells)-1):
            if cells[i-1] == cells[i+1]:
                nextDay[i] = 1
        # print('%s -> %s' % (cells, nextDay))
        return nextDay
        
    def prisonAfterNDays(self, cells: List[int], N: int) -> List[int]:
        if N == 0:
            return cells
        cells = originCells = self.prisonNextDay(cells)
        N -= 1
        for day in range(N):
            cells = self.prisonNextDay(cells)
            if cells == originCells:
                circle = day + 1
                # print('circle: ', circle)
                return self.prisonAfterNDays(cells, N % circle)
        return cells       
# @lc code=end


Solution().prisonAfterNDays([1,0,0,1,0,0,1,0], 1000000000)
