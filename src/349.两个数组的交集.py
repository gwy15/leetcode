#
# @lc app=leetcode.cn id=349 lang=python3
#
# [349] 两个数组的交集
#
from prelude import *
# @lc code=start


class Solution:
    def intersection(self, nums1: List[int], nums2: List[int]) -> List[int]:
        nums1 = set(nums1)
        nums2 = set(nums2)

        return list(nums1 & nums2)
# @lc code=end
