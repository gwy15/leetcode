#
# @lc app=leetcode.cn id=1395 lang=python3
#
# [1395] 统计作战单位数
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def numTeams(self, rating: List[int]) -> int:
        n = len(rating)
        cnt = 0
        for i in range(1, n-1):
            r = rating[i]
            # left
            left_lt, left_gt = 0, 0
            for j in range(i):
                if rating[j] < r:
                    left_lt += 1
                elif rating[j] > r:
                    left_gt += 1
            right_lt, right_gt = 0, 0
            for j in range(i+1, n):
                if rating[j] < r:
                    right_lt += 1
                elif rating[j] > r:
                    right_gt += 1
            cnt += (left_lt * right_gt) + (left_gt * right_lt)
        return cnt


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().numTeams(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([2, 5, 3, 4, 1], 3)
    test([2, 1, 3], 0)
    test([1, 2, 3, 4], 4)
    test([1, 2], 0)
    test([1], 0)
    test([], 0)
