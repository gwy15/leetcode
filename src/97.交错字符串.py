#
# @lc app=leetcode.cn id=97 lang=python3
#
# [97] 交错字符串
#

# @lc code=start


class Solution:
    def isInterleave(self, s1: str, s2: str, s3: str) -> bool:
        LEN_1, LEN_2, LEN_3 = len(s1), len(s2), len(s3)

        if LEN_1 + LEN_2 != LEN_3:
            return False

        # dp: [_i, j] => s3[.._i] 与 s1[..j] 是否匹配
        last = [False] * (LEN_1 + 1)
        last[0] = True

        for i in range(1, LEN_3+1):  # i 是 s3 的长度
            dp = [False] * (LEN_1 + 1)
            # s1 的长度最长为 len_1
            length = min(i, LEN_1)
            for j in range(length + 1):  # 剩下的不用算了
                # s3 长度为 i，最后一个数字为 s3[i-1]
                # s1 长度为 j
                if j > 0 and s3[i - 1] == s1[j - 1]:
                    # f[i,j] |= f[i-1, j-1]
                    dp[j] |= last[j-1]
                # 同理
                if 0 < i - j <= LEN_2 and s3[i - 1] == s2[i - j - 1]:
                    dp[j] |= last[j]
            last = dp

        return last[LEN_1]


# @lc code=end
if __name__ == "__main__":
    f = Solution().isInterleave
    assert f('aabcc', 'dbbca', 'aadbbcbcac')
    assert not f('aabcc', 'dbbca', 'aadbbbaccc')
    assert f('abc', 'azc', 'aabczc')
    assert f('', '', '')
    assert f('abc', '', 'abc')
