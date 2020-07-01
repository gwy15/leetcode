#
# @lc app=leetcode.cn id=718 lang=python3
#
# [718] 最长重复子数组
#
from typing import List
# @lc code=start


class Solution:
    def findLength(self, A: List[int], B: List[int]) -> int:
        m = len(A)
        n = len(B)
        dp = [0] * (n+1)
        last_dp = [0] * (n+1)
        ans = 0
        for i in range(m):
            for j in range(n):
                if A[i] == B[j]:
                    dp[j+1] = last_dp[j] + 1
                else:
                    dp[j+1] = 0
                ans = max(ans, dp[j+1])
            # print(f'{i=}, {dp=}')
            last_dp = dp[:]

        return ans


# @lc code=end
if __name__ == "__main__":
    f = Solution().findLength
    assert f([1, 2, 3, 2, 1], [3, 2, 1, 4, 7]) == 3
    assert f([], []) == 0
    assert f([1, 2], [1]) == 1
    assert f([3, 2, 1, 2, 3], [2, 1, 2]) == 3
    assert f([0, 1, 1, 1, 1], [1, 0, 1, 0, 1]) == 2
