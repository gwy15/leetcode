#
# @lc app=leetcode.cn id=119 lang=python3
#
# [119] 杨辉三角 II
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def getRow(self, rowIndex: int) -> List[int]:
        n = rowIndex
        ans = [1] * (n + 1)
        last = 1
        for i in range(1, n // 2 + 1):
            last = last * (n - i + 1) // i

            ans[i] = last
            ans[n - i] = last

        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().getRow(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(0, [1])
    test(1, [1, 1])
    test(2, [1, 2, 1])
    test(3, [1, 3, 3, 1])
    test(4, [1, 4, 6, 4, 1])
