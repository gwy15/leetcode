#
# @lc app=leetcode.cn id=40 lang=python3
#
# [40] 组合总和 II
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates.sort()
        ans = []

        def dfs(selected, sum, i):
            nonlocal ans
            if sum > target:
                return
            if i >= len(candidates):
                if sum == target:
                    ans.append(selected[:])
                return

            # must select i
            if len(selected) > 0 and selected[-1] == candidates[i]:
                selected.append(candidates[i])
                dfs(selected, sum+candidates[i], i+1)
                selected.pop()
                return

            if sum == target:
                ans.append(selected[:])
                return

            #
            selected.append(candidates[i])
            dfs(selected, sum+candidates[i], i+1)
            selected.pop()
            #
            dfs(selected, sum, i+1)

        dfs([], 0, 0)
        return ans


# @lc code=end
if __name__ == '__main__':
    def test(candidates, target, expected):
        calc = Solution().combinationSum2(candidates, target)
        def f(l): return ','.join(str(i) for i in sorted(l))
        if sorted(map(f, calc)) != sorted(map(f, expected)):
            print(f'case failed: `{candidates, target}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)

    # test([10, 1, 2, 7, 6, 1, 5], 8, [
    #     [1, 7],
    #     [1, 2, 5],
    #     [2, 6],
    #     [1, 1, 6]
    # ])
    # test(
    #     [2, 5, 2, 1, 2], 5, [
    #         [1, 2, 2], [5]
    #     ]
    # )
    # test(
    #     [1, 1], 1, [[1]]
    # )
    test(
        [2, 5, 2, 1, 2], 5,
        [[1, 2, 2], [5]]
    )
