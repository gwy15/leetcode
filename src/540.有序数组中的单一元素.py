#
# @lc app=leetcode.cn id=540 lang=python3
#
# [540] 有序数组中的单一元素
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def singleNonDuplicate(self, nums: List[int]) -> int:
        n = (1 + len(nums)) // 2
        left, right = 0, n-1
        while left < right:
            mid = (left + right) // 2
            if nums[2 * mid] == nums[2 * mid + 1]:
                left = mid + 1
            else:
                right = mid
        return nums[left * 2]


# @lc code=end
if __name__ == '__main__':
    def test(input, ans):
        calc = Solution().singleNonDuplicate(input)
        if calc != ans:
            print(f'case failed: `{input}`')
            print(f'  left = `{calc}`')
            print(f'  ans  = `{ans}`')
            exit(1)
    test([1], 1)
    test([1, 1, 2], 2)
    test([1, 1, 2, 3, 3], 2)
    test([3, 3, 7, 7, 10, 11, 11], 10)