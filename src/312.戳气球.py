#
# @lc app=leetcode.cn id=312 lang=python3
#
# [312] 戳气球
#
from typing import List
# @lc code=start


class Solution:
    def maxCoins(self, nums: List[int]) -> int:
        # 过滤一遍 0
        nums = [i for i in nums if i > 0]
        n = len(nums)
        val = [1] + nums + [1]

        dp = [[0] * (n+2) for _ in range(n+2)]
        # i 从右到左，保证区间搜索过
        for i in range(n+1, -1, -1):
            for j in range(i, n+2):
                # (left, right)
                ans = 0
                for mid in range(i+1, j):
                    ans = max(
                        ans,
                        val[i] * val[mid] * val[j]
                        + dp[i][mid] + dp[mid][j]
                    )
                dp[i][j] = ans

        return dp[0][n+1]


# @lc code=end
if __name__ == "__main__":
    f = Solution().maxCoins
    assert f([3, 1, 5, 8]) == 167
    assert f([]) == 0
    assert f([1]) == 1
    assert f([2]) == 2
