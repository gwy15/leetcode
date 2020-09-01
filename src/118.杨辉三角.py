#
# @lc app=leetcode.cn id=118 lang=python3
#
# [118] 杨辉三角
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        ans = []
        last = []
        for i in range(numRows):
            row = []
            for j in range(i + 1):
                if j == 0 or j == i:
                    row.append(1)
                else:
                    row.append(last[j-1] + last[j])
            last = row
            ans.append(row)

        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().generate(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        5,
        [
            [1],
            [1, 1],
            [1, 2, 1],
            [1, 3, 3, 1],
            [1, 4, 6, 4, 1]
        ]
    )
    test(0, [])
    test(1, [[1]])
