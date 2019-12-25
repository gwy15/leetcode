#
# @lc app=leetcode id=31 lang=python3
#
# [31] Next Permutation
#

from typing import List


class Solution:
    def nextPermutation(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        def swap(i, j):
            nums[i], nums[j] = nums[j], nums[i]

        for index in range(len(nums)-1, -1, -1):
            if index == 0:  # all sorted
                # print('all sorted')
                nums[:] = nums[::-1]
                break
            if nums[index-1] < nums[index]:  # found, swap index-1
                # print(f'swap {index-1}')
                # find the last greater than nums[index-1] and swap
                for i in range(index, len(nums)):
                    if i == len(nums)-1 or nums[i+1] <= nums[index-1]:
                        swap(index-1, i)
                        break
                # reverse
                nums[index:] = nums[-1:index-1:-1]
                break

# a = [2,4,3,2,1]
# a = [1,2,3]
# a = [3,2,1]
# Solution().nextPermutation(a)
# print(a)
# # print([3,4,2,1,2])
# print([3,1,2,2,4])
