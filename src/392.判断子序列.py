#
# @lc app=leetcode.cn id=392 lang=python3
#
# [392] 判断子序列
#

# @lc code=start


class Solution:
    def isSubsequence(self, s: str, t: str) -> bool:
        i, j = 0, 0
        while j < len(t) and i < len(s):
            if t[j] == s[i]:
                i += 1
            j += 1

        if i == len(s):
            return True
        return False

# @lc code=end
if __name__ == "__main__":
    f = Solution().isSubsequence
    assert f('abc', 'ahbgdc') == True
    assert f('axc', 'ahbgdc') == False
    assert f('', '')
    assert f('', 'abc')
    assert not f('abc', 'ab')
    