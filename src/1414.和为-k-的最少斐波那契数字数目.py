#
# @lc app=leetcode.cn id=1414 lang=python3
#
# [1414] 和为 K 的最少斐波那契数字数目
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def findMinFibonacciNumbers(self, k: int) -> int:
        fibs = []
        a, b = 0, 1
        while b <= k:
            fibs.append(b)
            a, b = b, a + b
        cnt = 0
        i, n = 0, len(fibs)
        while i < n and k > 0:
            if fibs[n - 1 - i] <= k:
                cnt += 1
                k -= fibs[n - 1 - i]
            i += 1

        return cnt


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().findMinFibonacciNumbers(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(7, 2)
    test(10, 2)
    test(19, 3)
    test(13, 1)
