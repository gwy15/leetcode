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
        return self.find(nums, 0, len(nums))

    def find(self, nums: List[int], i: int, j: int) -> int:
        # print(f'find {nums[i:j]}, i={i}, j={j}')
        if i + 1 >= j:
            return nums[i]
        if j - i <= 3:
            return min(nums[i:j])
        # 退化
        if nums[i] < nums[j-1]:
            return nums[i]

        # 首位相等
        if nums[i] == nums[j-1]:
            mid = (i + j) // 2
            if nums[mid] > nums[i]:
                return self.find(nums, mid+1, j)
            elif nums[mid] < nums[i]:
                return self.find(nums, i, mid+1)
            else:
                while i + 1 < j and nums[i] == nums[j-1]:
                    i += 1
                return self.find(nums, i, j)

        mid = (i + j) // 2
        # print(f'mid={mid}, [mid]={nums[mid]}')
        if nums[mid] >= nums[i]:
            # 搜索 (mid, j)
            return self.find(nums, mid+1, j)
        if nums[mid] <= nums[j-1]:
            assert mid + 1 != j
            # 搜索 [i, mid]
            return self.find(nums, i, mid+1)
        raise ValueError


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
