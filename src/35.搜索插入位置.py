#
# @lc app=leetcode.cn id=35 lang=python3
#
# [35] 搜索插入位置
#
from typing import List
# @lc code=start


class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        n = len(nums)
        if target > nums[n-1]:
            return n
        # bisect
        left, right = 0, n - 1
        while left < right:
            mid = (left + right) // 2
            if nums[mid] < target:
                left = mid + 1
            else:  # nums[mid] >= target:
                right = mid

        return left


# @lc code=end
if __name__ == "__main__":
    f = Solution().searchInsert
    assert f([1, 3, 5, 6], 5) == 2
    assert f([1, 3, 5, 6], 2) == 1
    assert f([1, 3, 5, 6], 7) == 4
    assert f([1, 3, 5, 6], 0) == 0
    assert f([1, 1, 1, 1], 1) == 0
    assert f([1, 1, 1, 1], 2) == 4
