#
# @lc app=leetcode.cn id=1578 lang=python3
#
# [1578] 避免重复字母的最小删除成本
#
from prelude import *
# @lc code=start

import itertools


class Solution:
    def minCost(self, s: str, cost: List[int]) -> int:
        cur_ch = None
        kept_cost = 0
        max_cost = 0
        iter = itertools.chain(zip(s, cost), [(None, 0)])
        for ch, c in iter:
            if cur_ch != ch:
                # pop
                kept_cost += max_cost
                # reset
                cur_ch = ch
                max_cost = c
            else:
                max_cost = max(max_cost, c)

        return sum(cost) - kept_cost


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().minCost(*input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(("abaac", [1, 2, 3, 4, 5]), 3)
    test(("abc", [1, 2, 3]), 0)
    test(('aabaa', [1, 2, 3, 4, 1]), 2)
