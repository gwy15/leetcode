#
# @lc app=leetcode.cn id=1207 lang=python3
#
# [1207] 独一无二的出现次数
#
from typing import List
from utils import *
# @lc code=start

from collections import Counter


class Solution:
    def uniqueOccurrences(self, arr: List[int]) -> bool:
        c = Counter(arr)
        times = c.values()
        return len(times) == len(set(times))


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().uniqueOccurrences(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 2, 2, 1, 1, 3], True)
    test([1, 2], False)
    test([-3, 0, 1, -3, 1, 1, 1, -3, 10, 0], True)
