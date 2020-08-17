#
# @lc app=leetcode.cn id=34 lang=python3
#
# [34] 在排序数组中查找元素的第一个和最后一个位置
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    @staticmethod
    def lower_bound(nums: List[int], target: int) -> int:
        # find the first i s.t. nums[i] >= target
        left, right = 0, len(nums)
        while left < right:
            mid = (left + right) // 2
            if target <= nums[mid]:
                right = mid
            else:
                left = mid + 1
        return left

    @staticmethod
    def upper_bound(nums: List[int], target: int) -> int:
        # find the first i s.t. nums[i] > target
        left, right = 0, len(nums)
        while left < right:
            mid = (left + right) // 2
            if target < nums[mid]:
                right = mid
            else:
                left = mid + 1
        return left

    def searchRange(self, nums: List[int], target: int) -> List[int]:
        n = len(nums)
        if n == 0:
            return [-1, -1]

        lower = self.lower_bound(nums, target)
        if lower == n or nums[lower] != target:
            return [-1, -1]
        # find upper bound
        upper = self.upper_bound(nums, target)
        return [lower, upper - 1]


# @lc code=end
if __name__ == "__main__":
    def test(i, n, ans):
        o = Solution().searchRange(i, n)
        print(o)
        assert o == ans
    test([5, 7, 7, 8, 8, 10], 7, [1, 2])
    test([5, 7, 7, 8, 8, 10], 8, [3, 4])
    test([5, 7, 7, 8, 8, 10], 6, [-1, -1])
    test([5, 7, 7, 8, 8, 10], 10, [5, 5])
    test([], 0, [-1, -1])
    test([1], 2, [-1, -1])
    test([2], 2, [0, 0])
    test([2, 2], 2, [0, 1])
