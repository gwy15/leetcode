#
# @lc app=leetcode.cn id=128 lang=python3
#
# [128] 最长连续序列
#
from prelude import *
# @lc code=start


class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        seen = set(nums)
        ans = 0
        for n in nums:
            isFirst = (n-1) not in seen
            if not isFirst:
                continue
            # if first
            length = 1
            while (n + length) in seen:
                length += 1
            ans = max(ans, length)
        return ans

# @lc code=end
