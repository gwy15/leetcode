#
# @lc app=leetcode.cn id=56 lang=python3
#
# [56] 合并区间
#
from prelude import *
# @lc code=start


class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        n = len(intervals)
        # sort by start edge
        intervals.sort(key=lambda i: i[0])
        i = 0
        ans = []
        while i < n:
            interval = intervals[i]
            start, end = interval
            while i < n and intervals[i][0] <= end:
                end = max(end, intervals[i][1])
                i += 1
            ans.append([start, end])
        return ans

# @lc code=end
