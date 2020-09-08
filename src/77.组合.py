#
# @lc app=leetcode.cn id=77 lang=python3
#
# [77] 组合
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:

        def f(prefix: List[int], i: int, ans: List[List[int]]):
            if len(prefix) == k:
                ans.append(prefix[:])
                return
            if i > n:
                return
            if len(prefix) + (n - i + 1) < k:
                return

            # without i
            f(prefix, i+1, ans)
            # with i
            prefix.append(i)
            f(prefix, i+1, ans)
            prefix.pop()

        if n < k:
            return []

        ans = []
        f([], 1, ans)
        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        n, k = input
        calc = Solution().combine(n, k)
        def f(l): return ','.join(str(i) for i in l)
        def ff(l): return set(f(i) for i in l)
        if ff(calc) != ff(expected):
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test((4, 2), [
        [2, 4],
        [3, 4],
        [2, 3],
        [1, 2],
        [1, 3],
        [1, 4],
    ])
    test((1, 2), [])
    test((2, 1), [[1], [2]])
