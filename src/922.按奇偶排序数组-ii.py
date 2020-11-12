#
# @lc app=leetcode.cn id=922 lang=python3
#
# [922] 按奇偶排序数组 II
#
from prelude import *
# @lc code=start


class Solution:
    def sortArrayByParityII(self, A: List[int]) -> List[int]:
        n = len(A)
        i, j = 0, 1
        while i < n and j < n:
            while i < n and A[i] % 2 == 0:
                i += 2
            while j < n and A[j] % 2 == 1:
                j += 2
            if i < n and j < n:
                A[i], A[j] = A[j], A[i]
        return A


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().sortArrayByParityII(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([4, 2, 5, 7], [4, 5, 2, 7])
