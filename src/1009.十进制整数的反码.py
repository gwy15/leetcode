#
# @lc app=leetcode.cn id=1009 lang=python3
#
# [1009] 十进制整数的反码
#

# @lc code=start
class Solution:
    def bitwiseComplement(self, N: int) -> int:
        n = N.bit_length() or 1
        return (2 ** n) - 1 - N


# @lc code=end

