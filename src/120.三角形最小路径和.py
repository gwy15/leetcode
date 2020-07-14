#
# @lc app=leetcode.cn id=120 lang=python3
#
# [120] 三角形最小路径和
#
from typing import List
# @lc code=start


class Solution:
    def minimumTotal(self, triangle: List[List[int]]) -> int:
        n = len(triangle)
        last = [0] * (n + 1)
        for i in range(n-1, -1, -1):
            row = triangle[i]
            best = [0] * n
            # 0 -> 1, n-1 -> n
            for j in range(i+1):
                best[j] = min(last[j], last[j+1]) + row[j]
            last = best
        return last[0]


# @lc code=end
if __name__ == "__main__":
    f = Solution().minimumTotal
    assert f(
        [
            [2],
            [3, 4],
            [6, 5, 7],
            [4, 1, 8, 3]
        ]
    ) == 11
    assert f([[1]]) == 1
    assert f([]) == 0
