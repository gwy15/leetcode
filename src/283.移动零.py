#
# @lc app=leetcode.cn id=283 lang=python3
#
# [283] 移动零
#

# @lc code=start
class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        n = len(nums)
        i, j = 0, 0
        while i < n:
            while i < n and nums[i] == 0:
                i += 1
            if i < n:
                # move [i] to [j]
                if i != j:
                    nums[j] = nums[i]
                i += 1
                j += 1
        for j in range(j, n):
            nums[j] = 0

# @lc code=end

