#
# @lc app=leetcode.cn id=553 lang=python3
#
# [553] 最优除法
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def optimalDivision(self, nums: List[int]) -> str:
        if len(nums) == 1:
            return str(nums[0])
        if len(nums) == 2:
            return '{}/{}'.format(nums[0], nums[1])

        return '{}/({})'.format(
            nums[0],
            '/'.join(str(i) for i in nums[1:])
        )


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().optimalDivision(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1000, 100, 10, 2], '1000/(100/10/2)')
    test([1], '1')
    test([1, 2], '1/2')
