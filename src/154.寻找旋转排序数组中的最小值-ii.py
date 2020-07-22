#
# @lc app=leetcode.cn id=154 lang=python3
#
# [154] 寻找旋转排序数组中的最小值 II
#
from typing import List
import unittest
# @lc code=start


class Solution:
    def findMin(self, nums: List[int]) -> int:
        left, right = 0, len(nums)-1
        while left < right:
            mid = (left + right) // 2
            if nums[mid] > nums[right]:
                left = mid + 1
            elif nums[mid] < nums[right]:
                right = mid
            else:  # nums[mid] == nums[right]
                right -= 1

        return nums[left]

# @lc code=end


class Test(unittest.TestCase):
    def test(self):
        def f(arr, ans):
            self.assertEqual(Solution().findMin(arr), ans)
        f([3, 4, 5, 1, 2], 1)
        f([2, 2, 2, 0, 1], 0)
        f([1], 1)
        f([1, 1], 1)
        f([1, 1, 1], 1)
        f([1, 1, 1, 1], 1)
        f([1, 1, 2, 1, 1], 1)
        f([1, 1, 0, 1, 1], 0)
        f([1, 2, 3, 4, 5], 1)
        f([0, 0, 0, -1, 0], -1)
        f([3, 1], 1)
        f([5, 1, 1, 1, 1, 1, 1], 1)


if __name__ == "__main__":
    unittest.main()
