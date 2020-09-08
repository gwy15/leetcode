#
# @lc app=leetcode.cn id=849 lang=python3
#
# [849] 到最近的人的最大距离
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def maxDistToClosest(self, seats: List[int]) -> int:
        n = len(seats)
        dist = [n] * n
        #
        cur = n
        for i, occupied in enumerate(seats):
            if occupied == 1:
                cur = 0
            else:
                cur += 1
            dist[i] = cur
        cur = n
        for i in range(n-1, -1, -1):
            if seats[i] == 1:
                cur = 0
            else:
                cur += 1
            dist[i] = min(cur, dist[i])
        return max(dist)


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().maxDistToClosest(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 0, 0, 0, 1, 0, 1], 2)
    test([1, 0, 0, 0], 3)
    test([1], 0)
