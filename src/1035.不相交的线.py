#
# @lc app=leetcode.cn id=1035 lang=python3
#
# [1035] 不相交的线
#
from prelude import *
# @lc code=start


class Solution:
    def maxUncrossedLines(self, A: List[int], B: List[int]) -> int:
        # 最长公共子序列
        # dp[i, j] = if A[i] == B[j]
        #   dp[i-1, j-1]
        # else
        #   max(dp[i, j-1], dp[i-1, j])
        n, m = len(A), len(B)
        dp = [0] * m
        for i in range(n):
            new_dp = dp[:]
            for j in range(m):
                if A[i] == B[j]:
                    new_dp[j] = (dp[j-1] if j > 0 else 0) + 1
                else:
                    new_dp[j] = max(
                        new_dp[j-1] if j > 0 else 0,
                        dp[j]
                    )
            dp = new_dp
            # print(f'for line {i}: dp = {dp}')
        return dp[m-1]


# @lc code=end
if __name__ == '__main__':
    def test(A, B, expected):
        calc = Solution().maxUncrossedLines(A, B)
        if calc != expected:
            print(f'case failed: `{A, B}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        [1, 4, 2],
        [1, 2, 4],
        2
    )
    test(
        A=[2, 5, 1, 2, 5], B=[10, 5, 2, 1, 5, 2],
        expected=3
    )
    test(
        A=[1, 3, 7, 1, 7, 5], B=[1, 9, 2, 5, 1],
        expected=2
    )
    test(
        [0], [0], 1
    )
    test([0], [1], 0)
