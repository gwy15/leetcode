#
# @lc app=leetcode.cn id=1436 lang=python3
#
# [1436] 旅行终点站
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def destCity(self, paths: List[List[str]]) -> str:
        ends = set(pair[1] for pair in paths)
        starts = set(pair[0] for pair in paths)
        return list(ends - starts)[0]


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().destCity(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        [["London", "New York"], ["New York", "Lima"],
         ["Lima", "Sao Paulo"]], "Sao Paulo")
    test([["B", "C"], ["D", "B"], ["C", "A"]], "A")
    test([["A", "Z"]], "Z")
