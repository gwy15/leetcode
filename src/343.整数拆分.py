#
# @lc app=leetcode.cn id=343 lang=python3
#
# [343] 整数拆分
#

# @lc code=start


class Solution:
    def integerBreak(self, n: int) -> int:
        dp = [0] * 60
        dp[1] = 1
        for i in range(2, n+1):
            # 1 + 1
            dp[i] = (i // 2) * (i - i // 2)
            for j in range(1, i):
                # 1 + n
                dp[i] = max(dp[i], j * dp[i - j])
        return dp[n]


# @lc code=end
if __name__ == "__main__":
    f = Solution().integerBreak
    assert f(2) == 1
    assert f(10) == 36
