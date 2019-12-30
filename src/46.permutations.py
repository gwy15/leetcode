#
# @lc app=leetcode id=46 lang=python3
#
# [46] Permutations
#
from typing import List

# @lc code=start


class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        if len(nums) <= 2:
            if len(nums) == 2:
                return [
                    [nums[0], nums[1]],
                    [nums[1], nums[0]]]
            elif len(nums) == 1:
                return [[nums[0]]]
            else:
                return []
        results = []
        for index, num in enumerate(nums):
            rests = nums[:index] + nums[index+1:]
            for rest_permute in self.permute(rests):
                results.append(
                    [num] + rest_permute
                )
        return results
# @lc code=end


print(Solution().permute([1, 2, 3]))
print(Solution().permute([1]))
