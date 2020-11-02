#
# @lc app=leetcode.cn id=33 lang=python3
#
# [33] 搜索旋转排序数组
#
from prelude import *
# @lc code=start


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        n = len(nums)

        if target < nums[0] and target > nums[n-1]:
            return -1

        left, right = 0, n
        baseline = nums[left]
        while left + 1 < right:
            mid = (left + right) // 2
            if nums[mid] == target:
                return mid
            if nums[mid] >= baseline:
                if baseline <= target < nums[mid]:
                    right = mid  # exclusive
                else:
                    left = mid + 1
            else:
                if nums[mid] <= target < baseline:
                    left = mid + 1
                else:
                    right = mid

        if left < n and nums[left] == target:
            return left
        return -1


# @lc code=end
if __name__ == '__main__':
    def test(input, target, expected):
        calc = Solution().search(input, target)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 3], 0, -1)
    test([1, 2, 3, 0], 0, 3)
