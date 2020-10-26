#
# @lc app=leetcode.cn id=1365 lang=python3
#
# [1365] 有多少小于当前数字的数字
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def smallerNumbersThanCurrent(self, nums: List[int]) -> List[int]:
        N = 100
        #
        counter = [0 for _ in range(N+1)]
        for n in nums:
            counter[n] += 1
        #
        prefix_counter = [0 for _ in range(N+1)]
        for i in range(1, N+1):
            prefix_counter[i] = prefix_counter[i-1] + counter[i-1]
        #
        ans = [0 for _ in range(len(nums))]
        for i in range(len(nums)):
            ans[i] = prefix_counter[nums[i]]

        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().smallerNumbersThanCurrent(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([8, 1, 2, 2, 3], [4, 0, 1, 1, 3])
    test([6, 5, 4, 8], [2, 1, 0, 3])
    test([7, 7, 7, 7], [0, 0, 0, 0])
