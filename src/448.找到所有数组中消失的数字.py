#
# @lc app=leetcode.cn id=448 lang=python3
#
# [448] 找到所有数组中消失的数字
#
from prelude import *
# @lc code=start


class Solution:
    def findDisappearedNumbers(self, nums: List[int]) -> List[int]:
        n = len(nums)

        for i in range(n):
            x = abs(nums[i])
            # x => [x-1]
            nums[x-1] = - abs(nums[x-1])
        ans = []
        for i in range(n):
            if nums[i] > 0:
                ans.append(i+1)
        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().findDisappearedNumbers(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([4, 3, 2, 7, 8, 2, 3, 1], [5, 6])
