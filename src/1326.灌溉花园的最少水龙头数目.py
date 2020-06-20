#
# @lc app=leetcode.cn id=1326 lang=python3
#
# [1326] 灌溉花园的最少水龙头数目
#
from typing import List
# @lc code=start
from itertools import chain


class Solution:
    def minTaps(self, n: int, ranges: List[int]) -> int:
        furthest = [-1] * (n+1)
        for i in range(n+1):
            left, right = max(0, i - ranges[i]), min(n, i + ranges[i])
            furthest[left] = max(furthest[left], right)

        # choose from 0
        times = 1
        pos, reachable = 0, furthest[0]
        while reachable < n:
            best = pos
            for i in range(pos + 1, reachable + 1):
                best = max(best, furthest[i])
            # best = max(chain(
            #     (pos,),
            #     (furthest[i] for i in range(pos + 1, reachable + 1)),
            # ))

            if best == pos:
                return -1
            else:
                times += 1
                pos = reachable
                reachable = best
        return times


# @lc code=end
if __name__ == "__main__":
    s = Solution()

    def f(n, r, a):
        assert s.minTaps(n, r) == a
    f(5, [3, 4, 1, 1, 0, 0], 1)
    f(3, [0, 0, 0, 0], -1)
    f(7, [1, 2, 1, 0, 2, 1, 0, 1], 3)
    f(8, [4, 0, 0, 0, 0, 0, 0, 0, 4], 2)
    f(8, [4, 0, 0, 0, 4, 0, 0, 0, 4], 1)
    f(0, [0], 1)
