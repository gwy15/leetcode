#
# @lc app=leetcode.cn id=1033 lang=python3
#
# [1033] 移动石子直到连续
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def numMovesStones(self, a: int, b: int, c: int) -> List[int]:
        a, b, c = sorted([a, b, c])
        max_steps = c - a - 2
        if a + 2 == b + 1 == c:
            min_steps = 0
        elif a + 2 >= b or b + 2 >= c:
            min_steps = 1
        else:
            min_steps = 2
        return [min_steps, max_steps]


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().numMovesStones(*input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([4, 3, 2], [0, 0])
