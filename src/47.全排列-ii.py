#
# @lc app=leetcode.cn id=47 lang=python3
#
# [47] 全排列 II
#
from typing import List
from utils import *
# @lc code=start
from collections import Counter


class Solution:
    def permuteUnique(self, nums: List[int]) -> List[List[int]]:
        n = len(nums)
        nums.sort()
        nums = [list(_) for _ in Counter(nums).items()]

        ans = []

        def dfs(items, chosen):
            if len(chosen) == n:
                ans.append(chosen[:])
                return
            # 遍历每一个数字
            for idx, (num, times) in enumerate(items):
                if times == 0:
                    continue
                # 要求跟上一个数字不重复
                if len(chosen) and chosen[-1] == num:
                    continue

                # 对于每一个数字，填入 1~times 遍
                for chosen_times in range(1, times + 1):
                    for _ in range(chosen_times):
                        chosen.append(num)
                    items[idx][1] -= chosen_times

                    dfs(items, chosen)

                    for _ in range(chosen_times):
                        chosen.pop()
                    items[idx][1] += chosen_times

        dfs(nums, [])

        return ans
# @lc code=end


if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().permuteUnique(input)
        calc = set(tuple(i) for i in calc)
        expected = set(tuple(i) for i in expected)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 1, 2], [[1, 1, 2], [1, 2, 1], [2, 1, 1]])
    test([1, 2, 3],
         [
        [1, 2, 3],
        [1, 3, 2],
        [2, 1, 3],
        [2, 3, 1],
        [3, 1, 2],
        [3, 2, 1]
    ])
    test([1, 1, 1],
         [[1, 1, 1]])
    test([], [[]])
    test([1], [[1]])
