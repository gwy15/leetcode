#
# @lc app=leetcode.cn id=1014 lang=python3
#
# [1014] 最佳观光组合
#
from typing import List
# @lc code=start


class Solution:
    def maxScoreSightseeingPair(self, a: List[int]) -> int:
        part_i = a[0] + 0
        ans = -float('inf')
        for j in range(1, len(a)):
            ans = max(
                ans,
                a[j] - j + part_i
            )
            part_i = max(
                part_i,
                a[j] + j
            )
        return ans


# @lc code=end
if __name__ == "__main__":
    assert Solution().maxScoreSightseeingPair([8, 1, 5, 2, 6]) == 11
