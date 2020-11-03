#
# @lc app=leetcode.cn id=461 lang=python3
#
# [461] 汉明距离
#

# @lc code=start
class Solution:
    def hammingDistance(self, x: int, y: int) -> int:
        diff = x ^ y
        ans = 0
        for i in range(32):
            ans += (diff & 1)
            diff >>= 1
        return ans

# @lc code=end
