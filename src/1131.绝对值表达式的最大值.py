#
# @lc app=leetcode.cn id=1131 lang=python3
#
# [1131] 绝对值表达式的最大值
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def maxAbsValExpr(self, arr1: List[int], arr2: List[int]) -> int:
        INF = 1 << 30
        n = len(arr1)
        ans = 0
        for k in range(2**3):
            min_p, max_p = INF, -INF
            for i in range(n):
                p = (2 * (k & 1) - 1) * arr1[i] + \
                    (2 * ((k >> 1) & 1) - 1) * arr2[i] + \
                    (2 * ((k >> 2) & 1) - 1) * i
                max_p = max(max_p, p)
                min_p = min(min_p, p)
            ans = max(ans, max_p - min_p)

        return ans


# @lc code=end
if __name__ == '__main__':
    def test(arr1, arr2, expected):
        calc = Solution().maxAbsValExpr(arr1, arr2)
        if calc != expected:
            print(f'case failed: `{arr1, arr2}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 2, 3, 4], [-1, 4, 5, 6], 13)
    test([1, -2, -5, 0, 10], [0, -2, -1, -7, -4], 20)
