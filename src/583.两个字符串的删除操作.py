#
# @lc app=leetcode.cn id=583 lang=python3
#
# [583] 两个字符串的删除操作
#

# @lc code=start


class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        m, n = len(word1), len(word2)

        last_dp = [i for i in range(n+1)]
        for i in range(1, m+1):
            dp_i = [0] * (n + 1)
            dp_i[0] = i
            for j in range(1, n+1):
                if word1[i - 1] == word2[j - 1]:
                    dp_i[j] = last_dp[j-1]
                else:
                    dp_i[j] = 1 + min(
                        last_dp[j],
                        dp_i[j-1]
                    )
            last_dp = dp_i
        return last_dp[n]


# @lc code=end
if __name__ == "__main__":
    f = Solution().minDistance
    assert f('sea', 'eat') == 2
    assert f('', '') == 0
    assert f('', '123') == 3
    assert f('123', '') == 3
    assert f('abcd', 'abbd') == 2
    assert f('ab', 'a') == 1
