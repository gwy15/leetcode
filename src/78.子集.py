#
# @lc app=leetcode.cn id=78 lang=python3
#
# [78] 子集
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        n = len(nums)
        ans = []
        for k in range(2 ** n):
            subset = []
            for i in range(n):
                if (k >> i) & 1 == 1:
                    subset.append(nums[i])
            ans.append(subset)
        return ans

# @lc code=end


if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().subsets(input)
        if set(map(tuple, calc)) != set(map(tuple, expected)):
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 2, 3], [
        [3],
        [1],
        [2],
        [1, 2, 3],
        [1, 3],
        [2, 3],
        [1, 2],
        []
    ])
