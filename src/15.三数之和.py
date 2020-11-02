#
# @lc app=leetcode.cn id=15 lang=python3
#
# [15] 三数之和
#
from prelude import *
# @lc code=start


class Solution:
    def twoSum(self, nums: List[int], i: int, j: int, target: int):
        l, r = i, j - 1
        ans = []
        while l < r:
            left_val = nums[l]
            right_val = nums[r]
            sum = left_val + right_val
            if sum == target:
                ans.append([left_val, right_val])
                while l < r and nums[l] == left_val:
                    l += 1
            elif sum < target:
                while l < r and nums[l] == left_val:
                    l += 1
            else:
                while l < r and nums[r] == right_val:
                    r -= 1
        return ans

    def threeSum(self, nums: List[int]) -> List[List[int]]:
        n = len(nums)
        nums.sort()
        ans = []
        i = 0
        while i < n:
            first = nums[i]
            for second, third in self.twoSum(nums, i+1, n, target=-first):
                ans.append([first, second, third])
            while i < n and nums[i] == first:
                i += 1
        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().threeSum(input)
        calc = set(tuple(sorted(i)) for i in calc)
        expected = set(tuple(sorted(i)) for i in expected)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        [-1, 0, 1, 2, -1, -4],
        [
            [-1, 0, 1],
            [-1, -1, 2]
        ]
    )
