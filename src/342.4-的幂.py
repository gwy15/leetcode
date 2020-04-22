#
# @lc app=leetcode.cn id=342 lang=python3
#
# [342] 4的幂
#

# @lc code=start


class Solution:
    def isPowerOfFour(self, num: int) -> bool:
        return any(
            num == 4 ** i
            for i in range(0, 16)
        )
# @lc code=end


assert Solution().isPowerOfFour(4 ** 15)
