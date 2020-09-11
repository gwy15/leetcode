#
# @lc app=leetcode.cn id=216 lang=python3
#
# [216] 组合总和 III
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def combinationSum3(self, k: int, n: int) -> List[List[int]]:
        ans = []

        def dfs(selected: List[int], cur_sum: int, i: int):
            nonlocal ans
            if len(selected) == k:
                if cur_sum == n:
                    ans.append(selected[:])
                return
            if cur_sum >= n:
                return
            if i >= 10:
                return
            #
            dfs(selected, cur_sum, i+1)
            #
            selected.append(i)
            dfs(selected, cur_sum+i, i+1)
            selected.pop()

        dfs([], 0, 1)

        return ans


# @lc code=end
if __name__ == '__main__':
    def test(k, n, expected):
        calc = Solution().combinationSum3(k, n)
        if set(map(tuple, calc)) != set(map(tuple, expected)):
            print(f'case failed: `{k, n}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(3, 7, [[1, 2, 4]])
    test(3, 9, [[1, 2, 6], [1, 3, 5], [2, 3, 4]])
